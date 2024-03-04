# j9

[![ci](https://github.com/ynqa/j9/actions/workflows/ci.yml/badge.svg)](https://github.com/ynqa/j9/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/j9.svg)](https://crates.io/crates/j9)
[![Documentation](https://docs.rs/j9/badge.svg)](https://docs.rs/j9)
[![License](https://img.shields.io/crates/l/j9.svg)](LICENSE)

This repository contains two Rust crates:
- [j9-sys](./j9-sys/)
- [j9](./j9/)

j9-sys provides Rust bindings to 
[libjq](https://github.com/jqlang/jq)
library, enabling Rust applications to leverage jq's powerful
JSON processing capabilities at a low level.

j9 builds on j9-sys to offer a more
user-friendly API for executing jq programs directly from Rust code.
