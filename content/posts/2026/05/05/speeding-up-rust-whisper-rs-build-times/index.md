# speeding up rust / whisper-rs build times

i've been working on [Inspectra](https://inspectra.dev) and the backend has grown significantly despite my best efforts to keep it slim. the project is monolithic and pulls in:

* `axum` + `utoipa` for API endpoints
* `ffmpeg-next` + `image` + `cudarc` for video ingestion
* `cudarc` for custom CUDA kernels
* `candle` for DL inference
* `whisper-rs` for [whisper.cpp](https://github.com/ggml-org/whisper.cpp) bindings
* `sea-orm` for DB
* `tokio` for runtime
* and the usual suspects - `serde`, `tracing`, etc.

which lands at ~700 crates and a 3-minute compile. ugh.

those who know me, know i am a huge stickler for pruning Cargo.toml - unused deps still get downloaded and compiled even if no module touches them. i hadnt cleaned up in a while. [`cargo-udeps`](https://crates.io/crates/cargo-udeps) misses a lot, so i did a manual pass and got it down to ~2 minutes.

still no good.

so i went deeper, and i was able to bring it down to **~25s**. the rest of this post walks through the experiments that got me there

## T0 - baseline

### what

no custom profiles or `.cargo/config.toml`. this is the number i'm trying to beat.

### result

| mean | run type | median | stddev | N |
|---|---|---|---|---|
| **139.30s** | baseline | 140.00s | 1.70s | 10 |

## T1 - `mold` only

### what

i use [`mold`](https://github.com/rui314/mold) on my other projects - particularly [my website](https://github.com/arpadav/website) - where it gives a meaningful speed-up as a drop-in linker

### config

```toml
# .cargo/config.toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

### result

| mean | run type | median | stddev | N |
|---|---|---|---|---|
| **140.00s** | mold | 140.00s | 1.41s | 10 |

huh, thats weird? barely budged - and if anything, slightly worse.

clearly the time is being spent elsewhere, not on linking. without firing up a profiler, i noticed `whisper-rs-sys` was always the last crate standing, eating 60+ seconds on its own, with a LOT of cpu cores pinned during that stage

![`btm` showing cpu graph](cpu-annotated.png)

#### `--timings` to confirm

using `cargo build --timings` build made the bottleneck obvious:

![`cargo build --timings` output for the mold-only run](timings-mold-only.png)

| # | unit | total | frontend | codegen | features |
|---|------|-------|----------|---------|----------|
| 1 | whisper-rs-sys v0.15.0 build-script (run) | 114.6s | | | cuda |
| 2 | aws-sdk-s3 v1.119.0 | 51.2s | 23.8s (46%) | 27.4s (54%) | default, default-https-client, rt-tokio, rustls, sigv4a |
| 3 | candle-transformers v0.10.2 | 44.6s | 15.4s (35%) | 29.2s (65%) | cuda, default |
| 4 | candle-core v0.10.2 | 41.8s | 5.4s (13%) | 36.4s (87%) | cuda, cudarc, default |
| 5 | candle-kernels v0.10.2 build-script (run) | 34.6s | | | |
| 6 | aws-lc-sys v0.40.0 build-script (run) | 31.9s | | | prebuilt-nasm |

114.6s on a single build-script is absurd. everything else can run in parallel - `whisper-rs-sys` is the single thread blocking the whole graph from finishing.

## T2 - `whisper` env flags only

### what

`whisper-rs-sys` exposes [`WHISPER_DONT_GENERATE_BINDINGS`](https://codeberg.org/tazz4843/whisper-rs/src/commit/3354d83d5535b2e091166a672b45a3c4d912c7d5/sys/build.rs#L119) - set to `1` and it uses a pre-generated `bindings.rs` instead of running `bindgen` from scratch. easy try.

while i was at it, `whisper.cpp` defaults `CMAKE_CUDA_ARCHITECTURES` to `native`, which can re-detect on every clean build. pinning it explicitly has saved a couple seconds for me on other projects (probably negligible here, but free).

### config

```bash
cargo clean && \
  WHISPER_DONT_GENERATE_BINDINGS=1 \
  CMAKE_CUDA_ARCHITECTURES=120a-real \
  cargo build -r --workspace
```

or in my cargo config:

```toml
# .cargo/config.toml
[env]
WHISPER_DONT_GENERATE_BINDINGS = "1"
CMAKE_CUDA_ARCHITECTURES = "120a-real"
```

### result

| mean | run type | median | stddev | N |
|---|---|---|---|---|
| **135.00s** | env flags only | 135.00s | 1.89s | 10 |

so the `whisper-rs-sys` build-script itself only dropped 114.6s -> 114.0s, but overall mean shifted from 139.30s -> 135.00s. ~4s saved, basically free, but nowhere near the win i wanted

## T3 - env flags + `ccache`

### what

since `whisper-rs-sys` is a C/C++/CUDA build under the hood, [`ccache`](https://ccache.dev/) is the natural lever. point cmake's compiler launchers at `ccache` and stash the cache inside the project so it survives `cargo clean`

### config

```toml
# .cargo/config.toml
[env]
WHISPER_DONT_GENERATE_BINDINGS = "1"
CMAKE_CUDA_ARCHITECTURES = "120a-real"

CMAKE_C_COMPILER_LAUNCHER = "ccache"
CMAKE_CXX_COMPILER_LAUNCHER = "ccache"
CMAKE_CUDA_COMPILER_LAUNCHER = "ccache"
CCACHE_DIR = { value = ".cache/ccache", relative = true }
CCACHE_NOHASHDIR = "1"
CCACHE_BASEDIR = { value = ".", relative = true }
```

### result

| mean | run type | median | stddev | N |
|---|---|---|---|---|
| **139.33s** | cold (no cache yet) | 140.00s | 4.04s | 3 |
| **65.56s** | warm (cache populated) | 66.00s | 0.53s | 9 |

cold is roughly the same as baseline - expected, since the first compile has to populate the cache. warm cuts it in half. the new `--timings` ranking confirms `whisper-rs-sys` is no longer the elephant in the room:

![cargo --timings output after enabling ccache](timings-ccache-env-vars.png)

| # | unit | total | frontend | codegen | features |
|---|------|-------|----------|---------|----------|
| 1 | aws-lc-sys v0.40.0 build-script (run) | 28.0s | | | prebuilt-nasm |
| 2 | candle-kernels v0.10.2 build-script (run) | 24.9s | | | |
| 3 | aws-sdk-s3 v1.119.0 | 22.6s | 15.7s (70%) | 6.8s (30%) | default, default-https-client, rt-tokio, rustls, sigv4a |
| 4 | candle-transformers v0.10.2 | 17.6s | 11.2s (64%) | 6.4s (36%) | cuda, default |
| 5 | candle-core v0.10.2 | 16.5s | 4.0s (24%) | 12.5s (76%) | cuda, cudarc, default |

now the bottleneck has shifted to the rust crates themselves. which means time to bring `mold` back, and start caching rust too

## T4 - env flags + `ccache` + `mold`

### what

same `ccache` setup, plus mold back as the linker

### config

```toml
# .cargo/config.toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[env]
WHISPER_DONT_GENERATE_BINDINGS = "1"
CMAKE_CUDA_ARCHITECTURES = "120a-real"

CMAKE_C_COMPILER_LAUNCHER = "ccache"
CMAKE_CXX_COMPILER_LAUNCHER = "ccache"
CMAKE_CUDA_COMPILER_LAUNCHER = "ccache"
CCACHE_DIR = { value = ".cache/ccache", relative = true }
CCACHE_NOHASHDIR = "1"
CCACHE_BASEDIR = { value = ".", relative = true }
```

### result

| mean | run type | median | stddev | N |
|---|---|---|---|---|
| **135.00s** | cold | 135.00s | n/a | 1 |
| **65.67s** | warm | 66.00s | 0.58s | 3 |

within noise of T3. `mold` is still doing nothing here - the link step just isnt where the wall-clock is going. so the next move is to actually cache the rust compilation, which means swapping `ccache` for [`sccache`](https://github.com/mozilla/sccache) (it caches both rust and C/C++)

## T5 - env flags + `sccache` + `ccache` + `mold`

### what

`sccache` wraps `rustc` directly, so it can cache rust crate compilation across `cargo clean`s. before dropping `ccache` entirely, try stacking the two - `sccache` for rust, `ccache` for the cmake-driven C/C++/CUDA paths

### config

```toml
# .cargo/config.toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[build]
rustc-wrapper = "sccache"

[env]
WHISPER_DONT_GENERATE_BINDINGS = "1"
CMAKE_CUDA_ARCHITECTURES = "120a-real"
SCCACHE_DIR = { value = ".cache/sccache", relative = true }

CMAKE_C_COMPILER_LAUNCHER = "ccache"
CMAKE_CXX_COMPILER_LAUNCHER = "ccache"
CMAKE_CUDA_COMPILER_LAUNCHER = "ccache"
CCACHE_DIR = { value = ".cache/ccache", relative = true }
CCACHE_NOHASHDIR = "1"
CCACHE_BASEDIR = { value = ".", relative = true }
```

### result

| mean | run type | median | stddev | N |
|---|---|---|---|---|
| **143.33s** | cold | 144.00s | 1.15s | 3 |
| **25.07s** | warm | 24.88s | 0.59s | 9 |

big jump - warm goes from ~65s (T4) to ~25s. cold is slightly worse than baseline, same overhead story as T3. the question now is whether `ccache` is still pulling its weight, or `sccache` alone is doing all the work

## T6 - env flags + `sccache` only (no `mold`, no `ccache`)

### what

isolate `sccache` itself - drop both `ccache` and `mold` and see what `sccache` alone is worth.

two things motivated this:

* T1 and T4 both said the same thing - `mold` was, if anything, _ever so slightly_ slower here. linking just isnt where the wall-clock goes on this graph, so the wrapper overhead is pure cost
* `sccache` wraps `rustc`, which is where most of the time was actually being spent in T3/T4. if rust caching is doing the heavy lifting, the cmake-driven C/C++/CUDA paths that `ccache` was covering may not be material anymore

### config

```toml
# .cargo/config.toml
[build]
rustc-wrapper = "sccache"

[env]
WHISPER_DONT_GENERATE_BINDINGS = "1"
CMAKE_CUDA_ARCHITECTURES = "120a-real"
SCCACHE_DIR = { value = ".cache/sccache", relative = true }
```

### result

| mean | run type | median | stddev | N |
|---|---|---|---|---|
| **143.67s** | cold | 143.00s | 1.15s | 3 |
| **25.05s** | warm | 24.89s | 0.56s | 10 |

within noise of T5 - `ccache` was contributing essentially nothing once `sccache` was in place, and dropping `mold` cost nothing either

## T7 - env flags + `sccache` + `mold`

### what

add `mold` back on top of T6 as a final A/B - T1, T4, and T6 all hinted at the same conclusion (linking isnt the bottleneck), so this is the cleanest data point to put it to bed. while i was at it, ran a debug profile too to compare

### config

```toml
# .cargo/config.toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[build]
rustc-wrapper = "sccache"

[env]
WHISPER_DONT_GENERATE_BINDINGS = "1"
CMAKE_CUDA_ARCHITECTURES = "120a-real"
SCCACHE_DIR = { value = ".cache/sccache", relative = true }
```

### result

| mean | run type | median | stddev | N |
|---|---|---|---|---|
| **145.00s** | release - cold | 145.00s | n/a | 1 |
| **25.35s** | release - warm | 25.07s | 0.77s | 5 |
| **121.00s** | debug - cold | 121.00s | n/a | 1 |
| **31.64s** | debug - warm | 31.64s | 0.02s | 2 |

cold release is slightly worse than baseline since `sccache` has overhead populating the cache. warm release lands a hair slower than T6. T5, T6, T7 are all within noise of each other warm, but the with-`mold` runs (T1, T4, T7) and their no-`mold` pairs (T3, T6) tell the same story - `mold` isnt moving the needle on this graph.

`--timings` on the warm path confirms what's left isnt cacheable rust - its build-scripts:

![cargo --timings output for the sccache-only warm run](sccache-only.png)

| # | unit | total | frontend | codegen | features |
|---|------|-------|----------|---------|----------|
| 1 | candle-kernels v0.10.2 build-script (run) | 17.9s | | | |
| 2 | aws-lc-sys v0.40.0 build-script (run) | 6.2s | | | prebuilt-nasm |

interestingly, **debug warm is _slower_ than release warm** - `sccache` caches release artifacts more aggressively in this configuration. didnt expect that.

**the fastest configuration is actually T6 (`sccache` only, no `mold`, no `ccache`)**, with T5 (`sccache` + `ccache` + `mold`) statistically tied. dropping `ccache` and `mold` costs nothing and removes two moving parts.

## what's left - candle-kernels

even with `sccache` carrying the rust crates, the warm `--timings` table shows `candle-kernels v0.10.2` build-script still costs **17.9s** every clean build. cracking open the registry source at `~/.cargo/registry/src/index.crates.io-*/candle-kernels-0.10.2/build.rs` explains why:

* it uses [`cudaforge::KernelBuilder`](https://crates.io/crates/cudaforge) to compile **14 `.cu` files** under `src/` into PTX (everything except `moe_*.cu`)
* then compiles 3 MoE kernels (`moe_gguf.cu`, `moe_wmma.cu`, `moe_wmma_gguf.cu`) into a static `libmoe.a` that gets linked into the rust crate
* compiler flags: `--expt-relaxed-constexpr -std=c++17 -O3`, plus `-Xcompiler -fPIC` on linux

the main issue is `cudaforge` spawns `nvcc` directly via `Command::new`. there's no CMake in the loop, so the `CMAKE_CUDA_COMPILER_LAUNCHER = ccache` hook from T3-T5 never fires for these compiles. and `sccache` is a `rustc` wrapper - build-scripts (and the `nvcc` they spawn) are out of scope. so every `cargo clean` re-runs `nvcc` on 17 `.cu` translation units with no cache layer in front of it. that fits the observed ~17.9s pretty well

this one is open for future work - probably either teaching `cudaforge` to honor a launcher env var, wrapping `nvcc` with `sccache` directly, or persisting the `OUT_DIR` across cleans somehow.

## summary

| config | warm mean | speedup vs T0 |
|---|---|---|
| T0 baseline | 139.30s | 1.0x |
| T1 `mold` only | 140.00s | ~0.99x |
| T2 env flags only | 135.00s | 1.03x |
| T3 env flags + `ccache` | 65.56s | 2.13x |
| T4 env flags + `ccache` + `mold` | 65.67s | 2.12x |
| T5 env flags + `sccache` + `ccache` + `mold` | 25.07s | 5.56x |
| T6 env flags + `sccache` | **25.05s** | **5.56x** |
| T7 env flags + `sccache` + `mold` | 25.35s | 5.50x |

you might be asking why im not running T5 in practice (some of the runs were faster than T6 by a couple seconds), reason being keeping `ccache` on top of `sccache` doubles the cache footprint on disk for a statistically-tied result - not worth a whole second build system around for zero gain. and `mold` turning out to be a no-op tracks - linking just isnt the bottleneck on this graph, so dropping it is negligible.
