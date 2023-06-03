# Timelib for Rust

[![Crates.io](https://img.shields.io/crates/v/timelib)](https://crates.io/crates/timelib)
[![Build Status](https://github.com/westy92/timelib-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/westy92/timelib-rust/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/timelib)](https://docs.rs/timelib)
[![Code Coverage](https://codecov.io/gh/westy92/timelib-rust/branch/main/graph/badge.svg)](https://codecov.io/gh/westy92/timelib-rust)
[![Funding Status](https://img.shields.io/github/sponsors/westy92)](https://github.com/sponsors/westy92)

Timelib for Rust is a small Rust wrapper around the [timelib](https://github.com/derickr/timelib) library that is used to power PHP and MongoDB.

## Installation

Prerequisites:

`timelib` depends on `re2c` to be built. You can install it easily on all major platforms:

1. Linux: `apt-get install re2c`
1. Mac: `brew install re2c`
1. Windows: `choco install re2c`
1. From source: [re2c.org](https://re2c.org/)

Now you can install with:

```bash
cargo add timelib
```

## Usage

```rust
let tz = timelib::Timezone::parse("America/Chicago".into()).expect("Error parsing timezone!");
timelib::strtotime("tomorrow".into(), None, &tz);
timelib::strtotime("next tuesday".into(), Some(1654318823), &tz);
```

View the tests for more examples.

## Building

Make sure to check out all submodules.

Initial clone:

```bash
git clone --recurse-submodules https://github.com/westy92/timelib-rust
```

Post-clone:

```bash
git submodule init && git submodule update
```

Make sure to install `re2c` as described above.

You should now be able to run `cargo build` and `cargo test`.

## Updating the submodule version

```bash
git submodule update --remote
```

## Publishing

Remove `--dry-run` to publish for real.

```bash
cargo publish --dry-run
```
