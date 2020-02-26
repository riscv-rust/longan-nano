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

If you have a GD32VF103C**B** chip on your board, edit `.cargo/config` and replace
`memory-c8.x` with `memory-cb.x`.

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
cargo run --release --example ferris --features lcd
```

### Using RV-LINK for Flashing and Debugging

[RV-LINK](https://gitee.com/zoomdy/RV-LINK) is a Chinese firmware, similar to
[Black Magic Probe (BMP)](https://github.com/blacksphere/blackmagic/wiki). It
supports the Longan Nano, allowing to use one Longan Nano board as a debug
probe for another one. It can be built & flashed via
[PlatformIO](https://platformio.org/). Check out the latest version to ensure
that the code compiles:

```
> git clone https://gitee.com/zoomdy/RV-LINK
> cd RV-LINK
> git tag 
v0.0.1
v0.1
v0.2 # <- seems to be the latest tag, so let's check this out
> git checkout tags/v0.2 
```

PlatformIO allows building & flashing of firmware with a single command. To do
so, the board needs to be in bootloader mode (DFU mode). The board boots to
bootloader mode if the bootloader button is pressed while powering it up (e.g.
by plugging it in). However, it is also possible to enter bootloader mode
without un- and replugging the board: press the boot button, press the reset
button, release the reset button and finally release the boot button while the
board is plugged in.

```
> pio run -t upload # put the board in bootloader mode before
```

Once RV-LINK is flashed to your probe, connect the eight debug pins on the
probe with the debug pins on the debug target. Ensure that you connect the pins
according to this table:

| Probe Pin | Target Pin |
| ---       | ---        |
| JTDO      | JTDO       |
| JTDI      | JTDI       |
| JTCK      | JTCK       |
| JTMS      | JTMS       |
| 3V3       | 3V3        |
| GND       | GND        |

After you connected the debug probe to your host, a new serial port shows up.
You can connect GDB to this serial port as an `extended-remote`. For
predictable behavior when multiple serial devices are present (and hence
`/dev/ttyACM0` is not necessarily the RV-LINK device),
[udev](https://www.freedesktop.org/wiki/Software/systemd/) offers id symlinks.
However you may also use `/dev/ttyACM0` or even `COMx` if you run Windows.

```
> gdb
(gdb) target extended-remote /dev/serial/by-id/usb-RV-LINK_Longan_Nano_GD32XXX-3.0.0-7z8x9yer-if00
```

To flash the firmware, execute `load` in GDB:

```
> gdb target/remote/debug/demo
(gdb) target extended-remote /dev/ttyACM0
(gdb) monitor reset halt
(gdb) load
(gdb) monitor reset
```

To improve your workflow, you can put the aforementioned GDB commands in
a `debug.gdb` file and add these lines to `.cargo/config`:

```
[target.riscv32imac-unknown-none-elf]
runner = 'gdb -command=debug.gdb'
```

This way `cargo run --target riscv32imac-unknown-none-elf` will automatically
launch GDB, flash your firmware on the target and provide you with a full debug
environment.

You can infer the current status of the board by observing the blinking pattern
of the green LED:

| Blink Behavior                             | Probe Status                                           |
| ---                                        | ---                                                    |
| Short pulse, on for 100ms, off for 900ms   | GDB is not connected                                   |
| Slow flashing, on for 500ms, off for 500ms | GDB is connected and the debugged MCU is in halt state |
| Fast blinking, on for 100ms, off for 100ms | GDB is connected, and the debugged MCU is running      |
| Long pulse, on for 900ms, off for 100ms    | RV-LINK has failed. Retry after resetting RV-LINK      |

## License

Copyright 2019-2020 [RISC-V team][team]

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
