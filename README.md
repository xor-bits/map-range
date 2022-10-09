<div align="center">

# fontsdf

A super simple crate to map values from a range to another range.

[![dependency status](https://deps.rs/repo/github/Overpeek/map-range/status.svg)](https://deps.rs/repo/github/Overpeek/map-range)
[![build status](https://github.com/Overpeek/map-range/actions/workflows/rust.yml/badge.svg)](https://github.com/Overpeek/map-range/actions)
[![crates.io](https://img.shields.io/crates/v/map-range.svg?label=map-range)](https://crates.io/crates/map-range)
[![docs.rs](https://docs.rs/map-range/badge.svg)](https://docs.rs/map-range/)

</div>

### Example

```rust
use map_range::MapRange;
let _ = _5_i32.map_range(0..10, -10..10);
```
