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

so i went deeper, and TLDR i was able to bring it down to **~25s**. the rest of this post walks through the experiments that got me there.

> all timings below are wall-clock seconds for `cargo clean && cargo build -r --workspace -p xapi --features blip-cuda`. i ran the runs through a tiny [python script](./stats.py) (`N`, mean, median, stddev) so the numbers below are summaries, not dumps of every sample.

## T0 - baseline

no `.cargo/config.toml`, no custom profiles, nothing.

| run type | N | mean | median | stddev |
|---|---|---|---|---|
| baseline | 10 | 139.30s | 140.00s | 1.70s |

this is the number i'm trying to beat.

## T1 - `mold` as linker

i use [`mold`](https://github.com/rui314/mold) on my other projects - particularly [my website](https://github.com/arpadav/website) - where it gives a meaningful speed-up as a drop-in linker:

```toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

| run type | N | mean | median | stddev |
|---|---|---|---|---|
| mold | 10 | 140.00s | 140.00s | 1.41s |

huh, thats weird? barely budged - and if anything, slightly worse.

clearly the time is being spent elsewhere, not on linking. without firing up a profiler, i noticed `whisper-rs-sys` was always the last crate standing, eating 60+ seconds on its own, with a LOT of cpu cores pinned during that stage.

<!-- TODO: insert cpu-annotated.png here, showing the htop/cores during whisper-rs-sys build -->

### `--timings` to confirm

a `--timings` build made the bottleneck obvious:

| # | unit | total | frontend | codegen | features |
|---|------|-------|----------|---------|----------|
| 1 | whisper-rs-sys v0.15.0 build-script (run) | 114.6s | | | cuda |
| 2 | aws-sdk-s3 v1.119.0 | 51.2s | 23.8s (46%) | 27.4s (54%) | default, default-https-client, rt-tokio, rustls, sigv4a |
| 3 | candle-transformers v0.10.2 | 44.6s | 15.4s (35%) | 29.2s (65%) | cuda, default |
| 4 | candle-core v0.10.2 | 41.8s | 5.4s (13%) | 36.4s (87%) | cuda, cudarc, default |
| 5 | candle-kernels v0.10.2 build-script (run) | 34.6s | | | |
| 6 | aws-lc-sys v0.40.0 build-script (run) | 31.9s | | | prebuilt-nasm |

114.6s on a single build-script is absurd. everything else can run in parallel - `whisper-rs-sys` is the single thread blocking the whole graph from finishing.

## T2 - drop mold, add `whisper` env flags

`whisper-rs-sys` exposes [`WHISPER_DONT_GENERATE_BINDINGS`](https://codeberg.org/tazz4843/whisper-rs/src/commit/3354d83d5535b2e091166a672b45a3c4d912c7d5/sys/build.rs#L119) - set to `1` and it uses a pre-generated `bindings.rs` instead of running `bindgen` from scratch. easy try.

while i was at it, `whisper.cpp` defaults `CMAKE_CUDA_ARCHITECTURES` to `native`, which can re-detect on every clean build. pinning it explicitly has saved a couple seconds for me on other projects (probably negligible here, but free).

```bash
cargo clean && \
WHISPER_DONT_GENERATE_BINDINGS=1 \
CMAKE_CUDA_ARCHITECTURES=120a-real \
cargo build -r --workspace -p xapi --features blip-cuda
```

| run type | N | mean | median | stddev |
|---|---|---|---|---|
| env flags only | 10 | 135.00s | 135.00s | 1.89s |

so the `whisper-rs-sys` build-script itself only dropped 114.6s -> 114.0s, but overall mean shifted from 139.30s -> 135.00s. ~4s saved, basically free, but nowhere near the win i wanted.

<!-- TODO: T3 - i had a third experiment slot but the notes are blank here; add or remove this section before publishing -->

## T4 - `ccache`

since `whisper-rs-sys` is a C/C++/CUDA build under the hood, [`ccache`](https://ccache.dev/) is the natural lever. point cmake's compiler launchers at `ccache` and stash the cache inside the project so it survives `cargo clean`:

```toml
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

| run type | N | mean | median | stddev |
|---|---|---|---|---|
| cold (no cache yet) | 3 | 139.33s | 140.00s | 4.04s |
| warm (cache populated) | 9 | 65.56s | 66.00s | 0.53s |

cold is roughly the same as baseline - expected, since the first compile has to populate the cache. warm cuts it in half. and the new `--timings` ranking confirms `whisper-rs-sys` is no longer the elephant in the room:

| # | unit | total | frontend | codegen | features |
|---|------|-------|----------|---------|----------|
| 1 | aws-lc-sys v0.40.0 build-script (run) | 28.0s | | | prebuilt-nasm |
| 2 | candle-kernels v0.10.2 build-script (run) | 24.9s | | | |
| 3 | aws-sdk-s3 v1.119.0 | 22.6s | 15.7s (70%) | 6.8s (30%) | default, default-https-client, rt-tokio, rustls, sigv4a |
| 4 | candle-transformers v0.10.2 | 17.6s | 11.2s (64%) | 6.4s (36%) | cuda, default |
| 5 | candle-core v0.10.2 | 16.5s | 4.0s (24%) | 12.5s (76%) | cuda, cudarc, default |

now the bottleneck has shifted to the rust crates themselves. which means time to bring `mold` back, and start caching rust too.

<!-- TODO: timings-ccache-env-vars.png + timings-mold-only.png are floating in the asset folder - figure out which experiment each belongs to and embed inline -->

## T5 - `ccache` + `mold`

same `ccache` setup, plus mold back as the linker:

```toml
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

| run type | N | mean | median | stddev |
|---|---|---|---|---|
| cold | 1 | 135.00s | 135.00s | n/a (single sample) |
| warm | 3 | 65.67s | 66.00s | 0.58s |

<!-- TODO: collect more runs for T5 - cold N=1 and warm N=3 is too few for confident comparison vs T4 -->

within noise of T4. mold is still doing nothing here - the link step just isnt where the wall-clock is going. so the next move is to actually cache the rust compilation, which means swapping `ccache` for [`sccache`](https://github.com/mozilla/sccache) (it caches both rust and C/C++).

## T6 - `sccache` + `ccache`?

<!-- TODO: i had this section as an open question - whether running both sccache (for rust) and ccache (for the cmake side) gives any extra wins, or whether sccache covers both. need to actually run this experiment and fill in the table. -->

## T7 - just `sccache`

`sccache` wraps rustc directly, so it can cache rust crate compilation across `cargo clean`s - which is exactly the bottleneck T4 left behind.

```toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]

[build]
rustc-wrapper = "sccache"

[env]
WHISPER_DONT_GENERATE_BINDINGS = "1"
CMAKE_CUDA_ARCHITECTURES = "120a-real"
SCCACHE_DIR = { value = ".cache/sccache", relative = true }
```

| run type | N | mean | median | stddev |
|---|---|---|---|---|
| release - cold | 1 | 145.00s | 145.00s | n/a (single sample) |
| release - warm | 5 | 25.35s | 25.07s | 0.77s |
| debug - cold | 1 | 121.00s | 121.00s | n/a (single sample) |
| debug - warm | 2 | 31.64s | 31.64s | 0.02s |

<!-- TODO: collect more cold runs for T7 (release + debug) - currently N=1 each -->

cold release is slightly worse than baseline (145s vs 139s) since sccache has overhead populating the cache, but the **warm release is 25.35s, down from a 139.30s baseline** - a ~5.5x speedup on the steady-state developer loop.

`--timings` on the warm path confirms what's left isnt cacheable rust - its build-scripts:

| # | unit | total | frontend | codegen | features |
|---|------|-------|----------|---------|----------|
| 1 | candle-kernels v0.10.2 build-script (run) | 17.9s | | | |
| 2 | aws-lc-sys v0.40.0 build-script (run) | 6.2s | | | prebuilt-nasm |

partial timings sum: ~82s (the rest is parallelized away by the cache).

interestingly, **debug warm (31.64s) is _slower_ than release warm (25.35s)** - sccache caches release artifacts more aggressively in this configuration. didnt expect that.

## summary

| config | warm mean | speedup vs T0 |
|---|---|---|
| T0 baseline | 139.30s | 1.0x |
| T1 mold only | 140.00s | ~0.99x |
| T2 env flags | 135.00s | 1.03x |
| T4 ccache | 65.56s | 2.13x |
| T5 ccache + mold | 65.67s | 2.12x |
| T7 sccache + mold | **25.35s** | **5.50x** |

<!-- TODO: closing thoughts - the intro promises "10s", but my best measured warm config is 25.35s. clarify which run hit ~10s (incremental rebuild after no-op edit?) and whether to walk back the headline or add a final section showing how to get there. -->
