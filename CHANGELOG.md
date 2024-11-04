# Changelog

## Unreleased

- Replace unmaintained `proc-macro-error` crate by the `manyhow` one.

## 0.2.2 18.04.2024

- Update `homepage` to `repository` in cargo manifest files.

## 0.2.1 27.03.2024

- Fix lints

## 0.2.0

- Improved workspace support.

  If the `workspace` feature is enabled, then if the file cannot be found
  relative to the `CARGO_MANIFEST_DIR`, it will be searched relative to the
  cargo workspace root directory.

## 0.1.1

- Fix lints and `inlcude-utils` crate category.

## 0.1.0

First public release
