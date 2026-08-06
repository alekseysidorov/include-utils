# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.2.5] 2026.08.06

- Bump minimum supported Rust version to `1.92`. Update crate dependencies.

## [0.2.4] 2024.12.04

- Fix typos in the documentation.

- Bump minimum supported Rust version to `1.78`.

## [0.2.3] 2024.11.04

- Replace unmaintained `proc-macro-error` crate by the `manyhow` one.

## [0.2.2] 2024.04.18

- Update `homepage` to `repository` in cargo manifest files.

## [0.2.1] 2024.03.27

- Fix lints

## [0.2.0]

- Improved workspace support.

  If the `workspace` feature is enabled, then if the file cannot be found
  relative to the `CARGO_MANIFEST_DIR`, it will be searched relative to the
  cargo workspace root directory.

## [0.1.1]

- Fix lints and `include-utils` crate category.

## [0.1.0]

- First public release
