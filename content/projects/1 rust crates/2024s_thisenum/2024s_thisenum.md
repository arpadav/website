[https://crates.io/crates/thisenum](https://crates.io/crates/thisenum)

_Overview_

I have worked on many parser/generator projects, implementing standards, and whatnot. The most frequent pattern I see is there exists some enumeration or 'set' of values which, although qualitative, have some explicitly defined quantitative value assigned to them during parsing/generating. 

I know you can have internal representations of enums in Rust, e.g. using

```rust
#[repr(u8)]
enum Foo {
    Bar = 0,
    Baz = 1,
    Qux = 2,
}
```

However, this internal represenation then has to be cast in order to get the actual value. More-over, what if I want to do something a bit more complicated, like using `'static str` as the enum's type? A perfect use-case would be using [EPSG](https://en.wikipedia.org/wiki/European_Petroleum_Survey_Group) codes, which are strings like `"4326"`

```rust
use thisenum::Const;

#[derive(Const, Debug)]
#[armtype(&'static str)]
enum EpsgCode {
    #[value = "4326"]
    Ellipsoid2dWgs84,
    #[value = "3857"]
    ProjectiveWebMercator,
}
```

That is what this crate was attempting to solve. Previously, it only allowed for constant-literals (e.g. `syn::Lit`), but now, any expression can be used as a value, and it will return `&'static T` when associated with `#[armtype(&'static T)]`.

_README example_

```rust
use thisenum::Const;

#[derive(Const, Debug)]
#[armtype(&[u8])]
/// https://exiftool.org/TagNames/EXIF.html
enum ExifTag {
    // ...
    #[value = b"\x01\x00"]
    ImageWidth,
    #[value = b"\x01\x01"]
    ImageHeight,
    #[value = b"\x01\x02"]
    BitsPerSample,
    #[value = b"\x01\x03"]
    Compression,
    #[value = b"\x01\x06"]
    PhotometricInterpretation,
    // ...
}

assert_eq!(ExifTag::ImageWidth.value(), b"\x01\x00");
#[cfg(feature = "eq")]
assert_eq!(ExifTag::ImageWidth, b"\x01\x00");
```
