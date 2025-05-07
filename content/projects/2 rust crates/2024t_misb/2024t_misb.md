_Overview_

This crate is currently privated due to being under-review by my current employer for intellectual property reasons.

However, many multi-media (streaming) protocols are defined using [MISB](https://nsgreg.nga.mil/misb.jsp), which has many partial or incomplete implementations. As a result, I always see industry use slow adaptations of parsing/generating using Python, or incorrect implementation using their own proprietary tools. I have yet to see an exhaustive implementation which is open-source.

The intention here was to open-source the Rust implemenation, and create CPP and Python bindings. Then, industry could reference this single project and contribute to it under a GPL-like license.