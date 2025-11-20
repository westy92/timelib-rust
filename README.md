# Timelib for Rust

[![Crates.io](https://img.shields.io/crates/v/timelib)](https://crates.io/crates/timelib)
[![Build Status](https://github.com/westy92/timelib-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/westy92/timelib-rust/actions/workflows/ci.yml)
[![docs.rs](https://img.shields.io/docsrs/timelib)](https://docs.rs/timelib)
[![Code Coverage](https://codecov.io/gh/westy92/timelib-rust/branch/main/graph/badge.svg)](https://codecov.io/gh/westy92/timelib-rust)
[![Funding Status](https://img.shields.io/github/sponsors/westy92)](https://github.com/sponsors/westy92)

Timelib for Rust is a small Rust wrapper around the [timelib](https://github.com/derickr/timelib) library that is used to power PHP and MongoDB.

## Installation

You can install with:

```bash
cargo add timelib
```

## Usage

```rust
let tz = timelib::Timezone::parse("America/Chicago").expect("Error parsing timezone!");
timelib::strtotime("tomorrow", None, &tz);
timelib::strtotime("next tuesday", Some(1654318823), &tz);
```

View the tests for more examples.

## Alpine Linux / musl Support

This library works out-of-the-box on Alpine Linux (musl libc). The build system automatically detects musl targets and:

- Uses pregenerated FFI bindings (avoiding bindgen issues with Alpine's statically-linked Rust)
- Adjusts compiler flags for musl compatibility
- Statically compiles all C code into the binary (no external shared libraries needed besides `libm`)

On Alpine Linux:

```bash
apk add musl-dev
cargo build
cargo test
```

## Optional Features

The generated `re2c` outputs are bundled and automatically used. If you wish to generate these files yourself, do the following:

1. Install `re2c`. You can install it easily on all major platforms:
    - Linux: `apt-get install re2c`
    - Mac: `brew install re2c`
    - Windows: `choco install re2c`
    - From source: [re2c.org](https://re2c.org/)
1. Enable the `re2c` feature:
    - `timelib = { version = "0.3", features = ["re2c"] }`

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

You should now be able to run `cargo build` and `cargo test`.

If using the `re2c` feature, make sure to install `re2c` as described above. i.e. `cargo test --features re2c`.

## Updating the submodule version

```bash
git submodule update --remote
```

Make sure to regenerate the re2c outputs and bindings, then copy them to `pregenerated/`.

```bash
# Clean and build to generate fresh bindings
cargo clean
cargo build

# Copy generated bindings from build output
cp target/debug/build/timelib-*/out/bindings.rs pregenerated/bindings.rs

# Regenerate re2c outputs
cd ext/timelib/
make parse_date.c parse_iso_intervals.c
cp parse_date.c ../../pregenerated/
cp parse_iso_intervals.c ../../pregenerated/
```

## Publishing

Remove `--dry-run` to publish for real.

```bash
cargo publish --dry-run
```
