[https://crates.io/crates/crosstalk](https://crates.io/crates/crosstalk)

_Overview_

This project spawned out of a need to have in-memory topic-based pub-sub communication for a monolithic project, which was modular in development. Meaning, there are various "sub-modules" of a monolithic binary that are spawned on different threads, with their own async runtimes.

There were many existing Rust-crates which had `mpmc` channels implemented, but none of them were "topic-based". In addition, all topic-based solutions were things like [ZMQ](https://en.wikipedia.org/wiki/ZeroMQ) or [DDS](https://en.wikipedia.org/wiki/Data_Distribution_Service) which required data to be serialized, exposed on some port, sent over a bus to a buffer, deserialized, and then used. This is so unnecessary when I am working with a single binary and require it be in-memory, so the existing `mpmc` channels were the solution - I just needed to "topic-ize" them. Since all this was happening inside of the Rust binary, then using an enum for the topics was the natural choice.

This was my first time using procedural macros, but have since refactored the code with my current (2025) knowledge for some improvements. Until Rust decides to add a shared-state for proc-macro calls, the function to get subcribers/publishers requires a generic type `T`, which is a bit of a bummer for 2 main reasons:

1. The compiler can not infer the channel's data-type `T`, requiring the developer to specify it.
2. If the channel topic's data-type `D` is not user-specified `T`, this can only be caught during run-time.

This crate will never panic, but (2) is quite disappointing, and can only be fixed with the shared-state proc-macro calls which can "store" the channel topic -> channel data-type in some `HashMap<Topic, proc_macro2::TokenStream>`.

Originally, this project had used a manual implementation of pub-sub using [`flume`](https://docs.rs/flume/latest/flume/), but when I found out that [`tokio`](https://tokio.rs/) already had a broadcast mpmc channel, I switched to them in `0.2`, which ended up making a lot of my work redundant and this crate ended up being more of a "wrapper", but it was significantly faster and more performant, which is all I wanted.

_README example_

```rust
#![allow(dead_code)]

use std::thread;
use std::collections::HashMap;
use crosstalk::AsTopic;

#[derive(AsTopic)] // required for crosstalk topic
enum TopicZoo {
    Topic1,
    Topic2,
    Topic3,
    Topic4,
    Topic5,
    Topic6,
}

#[derive(Clone)] // required for crosstalk data
#[derive(PartialEq, Debug)]
struct Vehicle {
    make: String,
    model: String,
    color: Color,
    wheels: u8,
}

#[derive(Clone)] // required for crosstalk data
#[derive(PartialEq, Debug)]
enum Color {
    Red,
    Blue,
    Green
}

crosstalk::init! {
    TopicZoo::Topic1 => Vec<u32>,
    TopicZoo::Topic2 => String,
    TopicZoo::Topic3 => Vehicle,
    TopicZoo::Topic4 => HashMap<&str, Vec<Vehicle>>,
    TopicZoo::Topic5 => Color,
}
// TopicZoo::Topic6 not included: defaults to String

#[tokio::main]
async fn main() {
    let mut node = crosstalk::BoundedNode::<TopicZoo>::new(1024);

    let (pub0_topic5, mut sub0_topic5) = node
        .pubsub(TopicZoo::Topic5)
        .await
        .unwrap();
    let mut sub1_topic5 = node
        .subscriber(TopicZoo::Topic5)
        .await
        .unwrap();

    let message = Color::Red;

    thread::spawn(move || { pub0_topic5.write(message); });

    let received_0 = sub0_topic5.read().await;
    let received_1 = sub1_topic5.read().await;

    println!("{:?}", received_0);
    println!("{:?}", received_1);
    assert_eq!(received_0, received_1);
}
```
