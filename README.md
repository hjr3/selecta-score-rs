# Selecta Scoring Algorithm

A more performant version of the [selecta][selecta] scoring algorithm.

This library includes a C interface to make it easier to use in other
langauges. As an example, there is a [fork][selecta-hjr3] that demonstrates
how to include this library in Ruby code. Check out [rust-ffi-examples][rust-ffi-examples]
for details on how Rust FFI works.

## Build

`cargo build`

### Release

`cargo build --release`

## Test

`cargo test`

## Bench

Note: Rust nightly is required to run benchmarks

`cargo bench`

[selecta]: https://github.com/garybernhardt/selecta
[selecta-hjr3]: https://github.com/hjr3/selecta/tree/rust
[rust-ffi-examples]: https://github.com/alexcrichton/rust-ffi-examples
