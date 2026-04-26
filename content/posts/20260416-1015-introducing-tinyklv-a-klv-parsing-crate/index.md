# Introducing TinyKlv: A KLV parsing crate

I wanted to announce the official launch of `tinyklv`

* crates.io: [http://crates.io/crates/tinyklv](http://crates.io/crates/tinyklv)
* repo: [http://github.com/arpadav/tinyklv](http://github.com/arpadav/tinyklv)
* book: [http://arpadvoros.com/tinyklv](http://arpadvoros.com/tinyklv)
* more info: [http://arpadvoros.com/projects/2024t_tinyklv](http://arpadvoros.com/projects/2024t_tinyklv)

i've started this project about 2 years ago, when i was at my previous job. i worked on it in sprints and took a better part of a whole year break from it to focus on other priorities. originally, i built the core and was able to learn the fundamentals of rust's proc-macros.

since leaving, i've been itching to continue. i've since taken a deep dive on proc-macros, how to properly display errors, and i was able to add an unbelievable amount of functionality into this crate.

please check it out! if you're unfamiliar, its essentially just protobufs, with more customization on how the bytes are encoded and decoded. protobufs are just primitive version of KLV, and note that using a zero-copy parser like `winnow` makes `tinyklv` a far more attractive and faster parser than existing protobuf crates in rust. 
