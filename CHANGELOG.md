# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.9.0] - 2021-04-29

### Added

- Add `microbit::gpio` module with pins mapped to micro:bit names
- Refactor `microbit::display` and `microbit::led` to accept `gpio::Pins`
- Make probe-run the default runner
- Rewrite `serial_port` as a macro

### Fixed

- Fix rustdoc warnings
- Upgrade nrf51-hal to 0.12.1

[Unreleased]: https://github.com/therealprof/microbit/compare/v0.9.0...HEAD
[0.9.0]: https://github.com/therealprof/microbit/compare/v0.8.0...v0.9.0
