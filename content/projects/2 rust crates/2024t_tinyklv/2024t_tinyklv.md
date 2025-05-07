_Overview_

This is by-far the most comprehensive personal project I have ever worked on. **It is currently under active development, and due to be released in the near future.** I have had limited time to work on it, and can work on it when I can in my spare time.

It is a procedural macro library which is similar to [`serde`](https://crates.io/crates/serde) and [`bitfields`](https://crates.io/crates/bitfields), where you can create a Rust struct that defines a key-length-value (KLV) packet. The macro will then generate code to automatically parse out this struct from any buffer, as well as generate the packet from the struct.

One of the reasons I created this is to be able to parse and generate KLV packets for the [MISB](https://www.misb.org/) standard. However, the beauty of making this general purpose is that it can be used for custom KLV packets. Why would someone need that? To reduce bandwidth versus using something like [`MessagePack`](https://msgpack.org/) or [`CBOR`](https://cbor.io/), but also giving more flexibility than `bitfields` would provide.

_Example_

```rust
use tinyklv::Klv;
use tinyklv::prelude::*;

#[derive(Klv)]
#[klv(
    stream = &[u8],
    sentinel = b"\x00\x00\x00",
    allow_unimplemented_encode,
    key(dec = tinyklv::dec::binary::u8),
    len(dec = tinyklv::dec::binary::u8_as_usize),
)]
struct Foo {
    #[klv(key = 0x01, var = true, dec = tinyklv::dec::binary::to_string_utf8)]
    // value length is dynamically determined, always as input from stream
    // 
    // therefore, it is used as an input arg in decoder: `tinyklv::dec::binary::to_string_utf8`
    // (function signature = `fn(&mut S, usize) -> winnow::PResult<String>`)
    name: String,

    #[klv(key = 0x02, dec = tinyklv::dec::binary::be_u16)]
    // value length is always 2 bytes
    // 
    // therefore, it is not used as an input arg in decoder: `tinyklv::dec::binary::be_u16`
    // (function signature = `fn(&mut S) -> winnow::PResult<u16>`)
    number: u16,
}

let mut stream1: &[u8] = &[
    0x00, 0x00, 0x00,       // sentinel
    0x09,                   // packet length = 9 bytes
    0x01, 0x03,             // key: 0x01, len: 3 bytes
    0x4B, 0x4C, 0x56,       // value: "KLV"
    0x02, 0x02,             // key: 0x02, len: 2 bytes
    0x01, 0x02,             // value: 258
];
let stream1_ = stream1.clone();
// decode by seeking sentinel, then decoding data
match Foo::extract(&mut stream1) {
    Ok(foo) => {
        assert_eq!(foo.name, "KLV");
        assert_eq!(foo.number, 258);
    },
    Err(e) => panic!("{}", e),
}
// decode data directly (without seeking sentinel)
match Foo::decode(&mut &stream1_[4..]) {
    Ok(foo) => {
        assert_eq!(foo.name, "KLV");
        assert_eq!(foo.number, 258);
    },
    Err(e) => panic!("{}", e),
}

let mut stream2: &[u8] = &[
    0x00, 0x00, 0x00,       // sentinel
    0x12,                   // packet length = 18 bytes
    0x01, 0x0C,             // key: 0x01, len: 12 bytes
                            // value: "Hello World!"
    0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x57, 0x6F, 0x72, 0x6C, 0x64, 0x21,
    0x02, 0x02,             // key: 0x02, len: 2 bytes
    0x00, 0x2A,             // value: 42
];
match Foo::extract(&mut stream2) {
    Ok(foo) => {
        assert_eq!(foo.name, "Hello World!");
        assert_eq!(foo.number, 42);
    },
    Err(e) => panic!("{}", e),
}
```