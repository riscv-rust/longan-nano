[![crates.io](https://img.shields.io/crates/d/longan-nano.svg)](https://crates.io/crates/longan-nano)
[![crates.io](https://img.shields.io/crates/v/longan-nano.svg)](https://crates.io/crates/longan-nano)
[![Build Status](https://travis-ci.org/riscv-rust/longan-nano.svg?branch=master)](https://travis-ci.org/riscv-rust/longan-nano)

# `longan-nano`

> Board support crate for the Longan Nano board

## [Documentation](https://docs.rs/crate/longan-nano)

## Getting started

### Installing dependencies

- Rust 1.36 or a newer toolchain. e.g. `rustup default stable`

- `rust-std` components (pre-compiled `core` crate) for the RISC-V target. Run:

``` console
rustup target add riscv32imac-unknown-none-elf
```

- RISC-V toolchain ([e.g. from SiFive](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz))

- [openocd for GD32VF103](https://github.com/riscv-mcu/riscv-openocd)

### Running the examples

Start openocd:
```sh
/path/to/openocd -f sipeed-jtag.cfg -f openocd.cfg
```

Run one of the examples:
```sh
cargo run --example blinky
```
or
```sh
cargo run --release --example display
```

## License

Copyright 2018-2019 [RISC-V team][team]

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [RISC-V team][team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-risc-v-team
