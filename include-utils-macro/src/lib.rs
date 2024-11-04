//! Internal proc macro for the `include-utils` crate.

#![allow(missing_docs)]

use std::path::{Path, PathBuf};

use itertools::Itertools;
use manyhow::manyhow;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::LitStr;

use crate::include_location::{IncludeLocation, IncludeRange};

mod include_location;

#[manyhow]
#[proc_macro]
pub fn include_str_part(file: LitStr) -> manyhow::Result<TokenStream2> {
    let file = file.value();
    let location = IncludeLocation::parse(&file)?;
    let file_content = read_file(location.path)?;

    let processed_content = process_file(file_content, &location.range, |_content, _name| {
        Err(manyhow::error_message!("Anchors is not supported for the plain string").into())
    })?;

    Ok(quote!(#processed_content))
}

#[manyhow]
#[proc_macro]
pub fn include_md(file: LitStr) -> manyhow::Result<TokenStream2> {
    let file = file.value();
    let location = IncludeLocation::parse(&file)?;
    let file_content = read_file(location.path)?;

    // TODO Use markdown parser to analyze comments.
    let processed_content = process_file(file_content, &location.range, |content, anchor_name| {
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
            .join("\n");

        manyhow::ensure!(
            has_anchor,
            "Included file doesn't contain anchor with name: {anchor_name}"
        );
        Ok(output)
    })?;

    Ok(quote!(#processed_content))
}

fn cargo_manifest_dir() -> manyhow::Result<PathBuf> {
    let dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|err| {
        manyhow::error_message!("Unable to read `CARGO_MANIFEST_DIR` environment variable. {err}")
    })?;
    Ok(dir.into())
}

/// Search for file path relative cargo manifest directory and workspace root directory if `workspace`
/// feature is enabled.
fn search_file(file_path: &Path) -> manyhow::Result<PathBuf> {
    let manifest_dir = cargo_manifest_dir()?;

    let search_paths = [
        manifest_dir.clone(),
        // Searching for a file also relative the workspace root directory.
        #[cfg(feature = "workspace")]
        {
            let metadata = cargo_metadata::MetadataCommand::new()
                .manifest_path(manifest_dir.join("Cargo.toml"))
                .exec()
                .map_err(|err| {
                    manyhow::error_message!("Unable to execute cargo metadata command. {err}")
                })?;

            metadata.workspace_root.into_std_path_buf()
        },
    ];

    for search_path in search_paths {
        let full_path = search_path.join(file_path);

        if full_path.exists() {
            return full_path.canonicalize().map_err(|err| {
                manyhow::error_message!("Unable to canonicalize file path. {err}").into()
            });
        }
    }

    Err(manyhow::error_message!("File path `{}` not found", file_path.display()).into())
}

fn read_file(file_path: impl AsRef<Path>) -> manyhow::Result<String> {
    let full_path = {
        let file_path = file_path.as_ref();
        if file_path.is_absolute() {
            file_path.to_owned()
        } else {
            search_file(file_path)?
        }
    };

    std::fs::read_to_string(full_path)
        .map_err(|err| manyhow::error_message!("Unable to read file. {err}").into())
}

fn process_file<F: FnOnce(String, &str) -> manyhow::Result<String>>(
    content: String,
    range: &IncludeRange<'_>,
    anchor_processor: F,
) -> manyhow::Result<String> {
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
        IncludeRange::Anchor { name } => anchor_processor(content, name)?,
    };

    Ok(content.trim().to_owned())
}
