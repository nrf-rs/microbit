# microbit

_microbit_ contains everything required to get started using Rust to create firmwares for the fabulous
[BBC micro:bit](https://microbit.org) microcontroller board. This little board has everything and a kitchen sink built-in,
even a capable debugging interface.

## Getting started

All you need to start programming this device is:

* A BBC micro:bit board
* A computer (macOS and Linux work perfectly, [Windows tested as well](http://flames-of-code.netlify.com/blog/rust-microbit-windows/))
* A bit of open source software

### Know your version

The micro:bit comes in different versions. There is a separate crate for each major board version. See the table below to identify
which crate you need to use.

| Crate                          | Board version | Board image | Docs | crates.io | target |
| ------------------------------ | ------------- | ----------- | ---- | --------- | ------ |
| [`microbit`](./microbit)       | V1 | [<img src="https://github.com/microbit-foundation/microbit-svg/raw/master/microbit-drawing-back-1-5.png" width="124px" height="100px">](https://github.com/microbit-foundation/microbit-svg/blob/master/microbit-drawing-back-1-5.png) | [![docs.rs](https://docs.rs/microbit/badge.svg)](https://docs.rs/microbit) | [![crates.io](https://img.shields.io/crates/d/microbit.svg)](https://crates.io/crates/microbit) | `thumbv6m-none-eabi` |
| [`microbit-v2`](./microbit-v2) | V2 | [<img src="https://github.com/microbit-foundation/microbit-svg/raw/master/microbit-drawing-back-2.png" width="124px" height="100px">](https://github.com/microbit-foundation/microbit-svg/blob/master/microbit-drawing-back-2.png) | [![docs.rs](https://docs.rs/microbit-v2/badge.svg)](https://docs.rs/microbit-v2) | [![crates.io](https://img.shields.io/crates/d/microbit-v2.svg)](https://crates.io/crates/microbit-v2) | `thumbv7em-none-eabihf` |

### Install dependencies

The examples make use of some of the fantastic tooling from the [knurling](https://knurling.ferrous-systems.com/) project.
In order to run the examples you need to install [`probe-run`](https://github.com/knurling-rs/probe-run#installation)
and [`flip-link`](https://github.com/knurling-rs/flip-link#installation).

```bash
> cargo install probe-run flip-link
```

### Run an example

The first thing to try is one of the [examples](./examples) in this repository. Plug in your micro:bit and
run one of the commands below.

*For micro:bit V1*
```bash
> cargo run --release --manifest-path ./examples/display-blocking/Cargo.toml --features v1 --target thumbv6m-none-eabi
```

*For micro:bit V2*
```bash
> cargo run --release --manifest-path ./examples/display-blocking/Cargo.toml --features v2 --target thumbv7em-none-eabihf
```

You should see a lot of build output, the orange LED on the back of the micro:bit should flash quickly and
a message should appear on the LED display.

Congratulations! You've flashed your first rust program onto your micro:bit!

## Further reading

A guide to embedded development with Rust on the _microbit_ using this crate can be found in the [MicroRust book](https://droogmic.github.io/microrust/).

Other useful resources:
- [micro:bit developer community](https://tech.microbit.org)
- [micro:bit hardware overview](https://tech.microbit.org/hardware/)
- [nrf-hal](https://github.com/nrf-rs/nrf-hal#readme) the hardware abstraction layer (HAL) this repository is based on

## License

[0-clause BSD license](LICENSE-0BSD.txt).
