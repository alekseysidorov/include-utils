//! # Overview
//!
#![doc = crate::include_md!("../README.md:description")]
//!
//! # Partial include modes
//! 
//! We supports exactly the same modes of partial includes as the referred in the [mdbook].
//! 
//! ```rust
//! #![doc = include_utils::include_str_part!("tests/data/sample.md:2")]
//! #![doc = include_utils::include_str_part!("tests/data/sample.md::10")]
//! #![doc = include_utils::include_str_part!("tests/data/sample.md:2:")]
//! #![doc = include_utils::include_str_part!("tests/data/sample.md:2:10")]
//! ```
//! 
//! The first line includes the second line from the file sample.md. 
//! The second one includes all lines up to the line 10, i.e. the lines 
//! from 11 till the end of file are omitted. 
//! The third command includes all file lines from the 2, i.e. the first line is omitted.
//! this last one includes lines 2 to 10.
//! 
//! To avoid breaking your doc when modifying included files, you can include a specific section
//! using anchors instead of line numbers. An anchor is a pair of matching comment lines.
//! The line beginning an anchor must match the pattern `ANCHOR: anchor_name` and
//! similarly the ending line must match the pattern: `ANCHOR_END: anchor_name`.
//! 
//! ```markdown
//! <!-- ANCHOR: anchor_name -->
//! An example of anchored section in markdown file.
//! <!-- ANCHOR_END: anchor_name -->
//! ```
//! 
//! [mdbook]: https://rust-lang.github.io/mdBook/format/mdbook.html#including-portions-of-a-file

/// Includes a markdown file as a string.
/// 
/// See [module][self] documentation.
pub use include_utils_macro::include_md;
/// Includes a UTF-8 encoded file as a string.
/// 
/// _**Note!** Anchors is not supported by this macro, use specific `include_md` macro to 
/// include markdown file section._
/// 
/// See [module][self] documentation.
pub use include_utils_macro::include_str_part;
