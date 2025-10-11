# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

(no changes)

## [0.16.0] - 2025-10-11

- Remove dead link to microbit Rust on Windows blog post in README.
- Bumped MSRV to 1.81.0.
- Add metadata for docs.rs
- Bumped dependencies in `examples/` to latest versions.
- Update HAL crates to 0.19.0.

## [0.15.1] - 2024-08-05

- Set MSRV for `microbit` and `microbit-v2` crates.
- Bumped MSRV to 1.79.0.
- Make various `I2C` `Pin` fields `pub`

## [0.15.0] - 2024-06-19

- Update HAL crates to 0.18.0.
- Update installation instructions for `probe-rs`

## [0.14.0] - 2024-04-18

- Fix: non-blocking display on micro:bit V2 could spuriously light LEDs briefly
- Fix the `blocking::Display::set_refresh_rate` calculation for the micro:bit V2
- Double the non-blocking display refresh frequency for the micro:bit V2
- Fix faulty doc test in `blocking.rs`
- Update the non-blocking display documentation to better explain when methods
  should be called from within a critical section
- Bump example dependencies to latest versions to fix build
- Update examples to use RTIC 1.0
- Add ADC for micro:bit V1
- Add analog example
- Increase minimum supported Rust version to 1.73
- Added support for the real time counters RTC1 and RTC2
- Add common types and structs for the edge connector pads and pins
- Add common ADC types and initialization for ADC and SAADC
- Common ADC example
- Add support for PPI
- Servo example using TIMER, GPIOTE and PPI
- (NFC) GitHub CI changes
- Feature: Exposed all remaining peripherals for both boards.
- Update HAL crates to 0.17.1.
- Update to `embedded-hal` 1.0.
- Update magnetometer example to use `lsm303agr` 1.0.
- Update debug tooling from probe-run to probe-rs

## [0.13.0] - 2022-05-24

- Drop ble-beacon example (since rubble is now archived as unmaintained)
- Bump `defmt` versions to 0.3
- Increase minimum supported Rust version to 1.57
- Add display-text-rtic example
- Make `Board::new(p, cp)` public and fix RTIC example
- Fix display-nonblocking example
- Fix timer for LED-display (GreyscaleImage with a empty Row did not work)
- Add SAADC and microphone_pins for micro:bit V2.
- Add microphone example

## [0.12.0] - 2021-11-10

### Changed

- Update dependencies nrf51-hal and nrf52833-hal to 0.14.0
- Added TEMP field to board
- Fixed Issue where columns 2,3 and 4 of the nonblocking display were swapped

## [0.11.0] - 2021-09-13

### Added

- Added support for the thumbv7em-none-eabi target for microbit:v2 (same as
  thumbv7em-none-eabihf but without hardware floating point support)

### Changed

- Rearrange LED display modules under the same root module and change their
  APIs to be more aligned with each other.
- Add BLE Beacon demo.
- Add a simple speaker demo for micro:bit V2.
- Add Board struct following the pattern used in other nrf board support crates.
- Add magnetometer example.
- LEDs on the micro:bit V1 are now turned off per default
- UART(E) is now exposed in the same way as I2C

## [0.10.1] - 2021-05-25

Republished without changes to fix missing README.md in crates.io.

## [0.10.0] - 2021-05-13

### Added

- Add support for micro:bit V2. This is a significant change that splits
  this repository into multiple crates.

## [0.9.0] - 2021-04-29

### Added

- Add `microbit::gpio` module with pins mapped to micro:bit names
- Refactor `microbit::display` and `microbit::led` to accept `gpio::Pins`
- Make probe-run the default runner
- Rewrite `serial_port` as a macro

### Fixed

- Fix rustdoc warnings
- Upgrade nrf51-hal to 0.12.1

[Unreleased]: https://github.com/nrf-rs/microbit/compare/v0.16.0...HEAD
[0.16.0]: https://github.com/nrf-rs/microbit/compare/v0.15.1...v0.16.0
[0.15.1]: https://github.com/nrf-rs/microbit/compare/v0.15.0...v0.15.1
[0.15.0]: https://github.com/nrf-rs/microbit/compare/v0.14.0...v0.15.0
[0.14.0]: https://github.com/nrf-rs/microbit/compare/v0.13.0...v0.14.0
[0.13.0]: https://github.com/nrf-rs/microbit/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/nrf-rs/microbit/compare/v0.11.0...v0.12.0
[0.11.0]: https://github.com/nrf-rs/microbit/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/nrf-rs/microbit/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/nrf-rs/microbit/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/nrf-rs/microbit/compare/v0.8.0...v0.9.0
