# Include utils

A more powerful replacement for the standard `inlcude_str` macros.

<!-- ANCHOR: description -->

Often you only need a specific part of the file, e.g. relevant lines for an
example, or section of README.md. This crate provides macros that can include
only part of a file, similar to the [mdbook] specific feature.

Imagine that you want to include "usage" section from your repository readme
file to the crate documentation. But you do not want to see in crate
documentation some parts of readme file, like header, badges, etc. With the
[`include_str`] macro you can only include the entire file content.

But with the [`include_md`] macro you can include only a specific section of the
file.

## Notes

- Unlike the built-in macro, this macros uses the `CARGO_MANIFEST_DIR` as the
  current directory instead of the directory from which macro is called.

- If the `workspace` feature is enabled, then if the file cannot be found
  relative to the `CARGO_MANIFEST_DIR`, it will be searhed relative to the cargo
  workspace root directory. It may be useful if you want to store your
  documentation in the single directory outside the crates. In this case you
  have to copy included directory to each crate before you publish it to the
  crates registry.

## Usage

```rust
//! # Crate overview
//! 
#![doc = include_utils::include_md!("README.md:description")]
//!
//! ## Other section
```

[mdbook]: https://rust-lang.github.io/mdBook/format/mdbook.html#including-portions-of-a-file

<!-- ANCHOR_END: description -->

[`include_str`]: https://doc.rust-lang.org/stable/std/macro.include_str.html
