* crates.io: [http://crates.io/crates/tinyklv](http://crates.io/crates/tinyklv)
* book: [http://arpadvoros.com/tinyklv](http://arpadvoros.com/tinyklv)

_Overview (May 30 2026)_

Since release, I have been taken a step back to focus more on startup work amongst other personal projects. But the past couple days I've taken the liberty to look at some optimizations. A couple I noticed quick when comparing this crate to existing protobuf crates (despite the fact that this isnt protobuf, protobuf is still technically a flavor of TLV), how the encoding path of things was using a bit too much heap allocations. This was because I was using the `EncodeValue` trait to return a `Vec<u8>` everytime and then every single field which implements this had to copy its contents from a slice of the owned value. That was the biggest win, but then i went even further and changed some of the early API (its pre 1.0 release with barely any downloads so im okay with it) and noticed how reserving select bytes for both decoding and encoding was a huge win. In fact, `tinyklv` ***still*** beat existing klv crates like `serde-klv` and `tlv_parser` in terms of speed, although speed was _never_ its intention.

now i have significantly closed the gap on protobuf crates, i am considering even making a `tinypb` crate which just emits `tinyklv` code... its lilke code-gen on code-gen. ironic if it works

see some of the gains here (as of may 30 2026, unpublished):

<img
  src="/images/tklv_bench_klv.jpg"
  alt="tinyklv vs other klv crates"
  style="float:left; width:48%; margin-right:2%;">

<img
  src="/images/tklv_bench_proto.jpg"
  alt="tinyklv vs protobuf crates"
  style="width:48%;">

<div style="clear:both;"></div>

_Overview (April 2026)_

* more info: [https://arpadvoros.com/posts/20260425-2213-introducing-tinyklv-a-klv-parsing-crate](/posts/20260425-2213-introducing-tinyklv-a-klv-parsing-crate)

A lot has changed since I was trying to publish this package. I developed this package on my own personal time, on my own personal resources, but I got into a bit of a disclosure disagreement with my previous employer (JHU/APL). They argued that it was relevant to the work I was performing at my job, which was not true. This was a completely independent project for generic bytestream parsing. Long story short, JHU/APL did end up granting me the rights and my ability to keep and publish this repo under a MIT license! However, the [MISB crate](/projects/2024t_misb/) (crate built using `tinyklv`) has been completely handed over to JHU/APL and I will have to start-over. The namespace, however, is completely open, so expect to see it up soon :)

I did have to compromise and branch off at a point in time in September 2024 and re-do a lot of the work I had done. I was set to release this (as you can see in my previous overview below) in July 2025, but I got caught up with start-up work, selling my condo, and haven't had time to sit down and work through it.

Note that the core of this project was **made 100% by hand and without the use of AI** (as of this update and version `^0.1`, however, im sure this will change). However, now that I use coding agents more frequently, I am running audits on the code and helping expand the documentation and testing - which has been admittedly great. However, this is one the largest projects I worked on without AI which brings huge satisfaction. 

_Overview (May 2025)_

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
    #[klv(key = 0x01, varlen = true, dec = tinyklv::dec::binary::to_string_utf8)]
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
match Foo::decode_frames(&mut stream1) {
    Ok(foo) => {
        assert_eq!(foo.name, "KLV");
        assert_eq!(foo.number, 258);
    },
    Err(e) => panic!("{}", e),
}
// decode data directly (without seeking sentinel)
match Foo::decode_value(&mut &stream1_[4..]) {
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
match Foo::decode_frames(&mut stream2) {
    Ok(foo) => {
        assert_eq!(foo.name, "Hello World!");
        assert_eq!(foo.number, 42);
    },
    Err(e) => panic!("{}", e),
}
```
