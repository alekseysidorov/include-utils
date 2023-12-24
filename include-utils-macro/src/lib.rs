//! Internal proc macro for the `include-utils` crate.

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::{proc_macro_error, Diagnostic, Level, ResultExt};
use quote::quote;
use syn::{parse_macro_input, LitStr};

use crate::include_location::{IncludeLocation, IncludeRange};

mod include_location;

#[proc_macro]
#[proc_macro_error]
pub fn include_str_part(input: TokenStream) -> TokenStream {
    let file = {
        let file: LitStr = parse_macro_input!(input);
        file.value()
    };

    let location = IncludeLocation::parse(&file);
    let file_content = read_file(location.path);

    let processed_content = process_file(file_content, location.range, |_content, _name| {
        proc_macro_error::abort_call_site!("Anchors is not supported for the plain string");
    });

    quote!(#processed_content).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn include_md(input: TokenStream) -> TokenStream {
    let file = {
        let file: LitStr = parse_macro_input!(input);
        file.value()
    };

    let location = IncludeLocation::parse(&file);
    let file_content = read_file(location.path);

    // TODO Use markdown parser to analyze comments.
    let processed_content = process_file(file_content, location.range, |content, anchor_name| {
        let anchor_begin = format!("<!-- ANCHOR: {anchor_name}");
        let anchor_end = format!("<!-- ANCHOR_END: {anchor_name}");

        let mut has_anchor = false;

        let output = content
            .lines()
            .skip_while(|line| !line.starts_with(&anchor_begin))
            .skip(1)
            .take_while(|line| {
                let is_end = line.starts_with(&anchor_end);
                if is_end {
                    has_anchor = true;
                }
                !is_end
            })
            .filter(|line| !line.starts_with("<!-- ANCHOR"))
            .join("\n");

        if !has_anchor {
            proc_macro_error::abort_call_site!(
                "Included file doesn't contain anchor with name: {}",
                anchor_name
            );
        }

        output
    });

    quote!(#processed_content).into()
}

fn err_to_diagnostic(message: impl Display) -> Diagnostic {
    Diagnostic::new(Level::Error, message.to_string())
}

fn crate_directory() -> PathBuf {
    std::env::var("CARGO_MANIFEST_DIR")
        .map_err(err_to_diagnostic)
        .unwrap_or_abort()
        .into()
}

fn read_file(file_path: impl AsRef<Path>) -> String {
    let full_path = {
        let file_path = file_path.as_ref();
        let path = if !file_path.is_absolute() {
            crate_directory().join(file_path)
        } else {
            file_path.to_owned()
        };

        path.canonicalize()
            .map_err(|err| format!("path `{path:?}`: {err}"))
            .map_err(err_to_diagnostic)
            .unwrap_or_abort()
    };

    std::fs::read_to_string(full_path)
        .map_err(err_to_diagnostic)
        .unwrap_or_abort()
}

fn process_file<F: FnOnce(String, &str) -> String>(
    content: String,
    range: IncludeRange<'_>,
    anchor_processor: F,
) -> String {
    let content = match range {
        // Just copy the entire file content.
        IncludeRange::Full => content,
        IncludeRange::Range { from, to } => {
            // To avoid confuses we just count line numbers from the one instead of zero.
            let from = from.unwrap_or_default().saturating_sub(1);
            // Just skip the file lines before the `from` line.
            let mut lines = content.lines().skip(from);
            if let Some(to) = to {
                // In this case we have an explicit end of file inclusion.
                // So we have to take `N` lines, where `N = to - from`.
                let n = to - from;
                lines.take(n).join("\n")
            } else {
                // Just include the whole file tail.
                lines.join("\n")
            }
        }
        IncludeRange::Anchor { name } => anchor_processor(content, name),
    };
    content.trim().to_owned()
}