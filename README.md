# microbit

[![microbit on crates.io][cratesio-image]][cratesio]
[![microbit on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/microbit.svg
[cratesio]: https://crates.io/crates/microbit
[docsrs-image]: https://docs.rs/microbit/badge.svg
[docsrs]: https://docs.rs/microbit

_microbit_ contains everything required to get started with the use of Rust to create firmwares for the fabulous [BBC micro:bit][] microcontroller board. This little board has everything and a kitchen sink built-in, even a capable debugging interface, so all that one needs to get going with programming this device is:

* A BBC micro:bit board (at the moment v1 only)
* A computer (macOS and Linux work perfectly, [Windows tested as well](http://flames-of-code.netlify.com/blog/rust-microbit-windows/))
* A bit of open source software

Some very preliminary examples of using this crate can be found [here in this repo][examples] or [here on my blog][myblog].

A guide to embedded development with Rust on the _microbit_ using this crate can be found in the [MicroRust book][microrust].

The [BBC micro:bit][] and this crate is compatible with the fantastic tooling
provided by the [knurling] project. If you haven't done so already, installing
`probe-run` and playing around with the examples is highly recommended:
```bash
# cargo install probe-run
# cargo run --release --example led_blocking
   Compiling microbit v0.8.0
    Finished release [optimized + debuginfo] target(s) in 15.39s
     Running `probe-run --chip nRF51822_xxAA target/thumbv6m-none-eabi/release/examples/led_blocking`
  (HOST) INFO  flashing program (1.95 KiB)
  (HOST) INFO  success!
RTT logs not available; blocking until the device halts..
```

[BBC micro:bit]: https://microbit.org
[cortex-m]:(https://github.com/japaric/cortex-m)
[cortex-m-rt]:(https://github.com/japaric/cortex-m-rt)
[examples]: https://github.com/therealprof/microbit/tree/master/examples
[myblog]: https://www.eggers-club.de/blog/2018/05/31/rust-on-the-microbit-101-part-1
[microrust]: https://droogmic.github.io/microrust/
[knurling]: https://knurling.ferrous-systems.com/

## License

[0-clause BSD license](LICENSE-0BSD.txt).
