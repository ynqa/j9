# j9

This repository contains two Rust crates:
- [j9-sys](./j9-sys/)
- [j9](./j9/)

j9-sys provides Rust bindings to 
[libjq](https://github.com/jqlang/jq)
library, enabling Rust applications to leverage jq's powerful
JSON processing capabilities at a low level.

j9 builds on j9-sys to offer a more
user-friendly API for executing jq programs directly from Rust code.
