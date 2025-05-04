# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.3.1] - 2025-04-30

### Added

- Implemented common `std` library traits for all public types (#5)

### Changed

- Made the fields of `ferrishot_knus::ast::Integer` and `ferrishot_knus::ast::Decimal` public (#1)
- Changed `parse_*` functions to take `file_name: impl AsRef<str>` to match `miette` (#18)

### Fixed
- Upgraded to `miette` v7.6.0, fixing several graphical bugs when reporting errors (#3)
- Improved macro hygiene to avoid clashing with imported `Result` aliases (#19)

## [3.2.0] - 2024-10-24

The beginning of time — this version is identical to [`knuffel` v3.2.0](https://crates.io/crates/knuffel/3.2.0).

[unreleased]: https://github.com/TheLostLambda/ferrishot_knus/compare/v3.3.1...HEAD
[3.3.1]: https://github.com/TheLostLambda/ferrishot_knus/compare/v3.2.0...v3.3.1
[3.2.0]: https://github.com/TheLostLambda/ferrishot_knus/releases/tag/v3.2.0
