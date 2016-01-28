# Selecta Scoring Algorithm

A more performant version of the [selecta][selecta] scoring algorithm.

[![Build Status](https://travis-ci.org/hjr3/selecta-score-rs.svg?branch=master)](https://travis-ci.org/hjr3/selecta-score-rs)
[![Crates.io](https://img.shields.io/crates/v/selecta_score.svg)](https://crates.io/crates/selecta_score/)


## FFI

This library includes a C interface to make it easier to use in other
langauges. As an example, there is a [fork][selecta-hjr3] that demonstrates
how to include this library in Ruby code.

On Mac OS X:

```ruby
require "fiddle"
require "fiddle/import"

module Score
  extend Fiddle::Importer

  dlload "/path/to/libselecta_score.dylib"

  extern "double selecta_score(char *, char *)"
end

score = Score::selecta_score("README.md", "em")

puts score
```

Check out [rust-ffi-examples][rust-ffi-examples] for details on how Rust FFI
works.

## Build

`cargo build`

### Release

`cargo build --release`

## Test

`cargo test`

## Bench

Rust nightly is required to run benchmarks. Comment out the lines in `tests/`.
Then run:

`cargo bench`

[selecta]: https://github.com/garybernhardt/selecta
[selecta-hjr3]: https://github.com/hjr3/selecta/tree/rust
[rust-ffi-examples]: https://github.com/alexcrichton/rust-ffi-examples
