# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Update dependencies nrf51-hal and nrf52833-hal to 0.14.0
- Added TEMP field to board

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

[Unreleased]: https://github.com/nrf-rs/microbit/compare/v0.11.0...HEAD
[0.11.0]: https://github.com/nrf-rs/microbit/compare/v0.10.1...v0.11.0
[0.10.1]: https://github.com/nrf-rs/microbit/compare/v0.10.0...v0.10.1
[0.10.0]: https://github.com/nrf-rs/microbit/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/nrf-rs/microbit/compare/v0.8.0...v0.9.0
