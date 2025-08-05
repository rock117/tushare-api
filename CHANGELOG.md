# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.1]

### Fixed
- Fixed cargo doc warnings by wrapping generic type parameters in backticks in documentation comments
- Resolved unclosed HTML tag warnings for `Vec<T>`, `TushareEntityList<T>`, and `TryFrom<TushareResponse>` in doc comments
- Improved docs.rs compatibility by following Rustdoc HTML validation requirements

## [1.1.0]

### Added
- Custom date format support with `#[tushare(date_format = "...")]` attribute
- `FromTushareValueWithFormat` trait for custom date parsing
- Support for field-level date format specification in procedural macros
- Comprehensive examples for custom date formats (European, US, German, Chinese, etc.)
- Enhanced date parsing with 11+ supported formats including YYYYMMDD, DD/MM/YYYY, etc.
- Generic pagination container `TushareEntityList<T>`
- Procedural macro system with `#[derive(DeriveFromTushareData)]`
- Field skipping with `#[tushare(skip)]`
- Automatic struct generation and conversion

- Comprehensive third-party type support through feature flags
- Support for `rust_decimal::Decimal` with high-precision arithmetic
- Support for `bigdecimal::BigDecimal` with arbitrary precision
- Support for `chrono` date/time types (NaiveDate, NaiveDateTime, DateTime<Utc>)
- Support for `uuid::Uuid` type
- `all_types` convenience feature flag for enabling all third-party types
- Conditional trait implementations with `#[cfg(feature = "...")]`
- Zero-cost abstractions when features are disabled

## Contributing

Please read our contributing guidelines and ensure all changes are documented in this changelog.

## Support

- 📖 [Documentation](https://docs.rs/tushare-api)
- 🐛 [Issue Tracker](https://github.com/rock117/tushare-api/issues)
- 💬 [Discussions](https://github.com/rock117/tushare-api/discussions)
