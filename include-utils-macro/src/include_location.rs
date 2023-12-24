use proc_macro_error::{abort_call_site, ResultExt};

use crate::err_to_diagnostic;

/// Include location part of the given file path.
#[derive(Debug, PartialEq, Eq)]
pub struct IncludeLocation<'a> {
    /// File path itself.
    pub path: &'a str,
    /// Range of file lines to include.
    pub range: IncludeRange<'a>,
}

/// Include range specification follows the mdbook include portion of file chapter.
///
/// https://rust-lang.github.io/mdBook/format/mdbook.html#including-portions-of-a-file
#[derive(Debug, PartialEq, Eq)]
pub enum IncludeRange<'a> {
    /// Include the entire file content as is.
    Full,
    /// Include the specific file part.
    Range {
        from: Option<usize>,
        to: Option<usize>,
    },
    /// Include part of file between anchor begin and end.
    Anchor { name: &'a str },
}

impl<'a> IncludeRange<'a> {
    fn line(num: usize) -> Self {
        Self::Range {
            from: Some(num),
            to: Some(num + 1),
        }
    }

    fn left(num: usize) -> Self {
        Self::Range {
            from: None,
            to: Some(num),
        }
    }

    fn right(num: usize) -> Self {
        Self::Range {
            from: Some(num),
            to: None,
        }
    }

    fn range(from: usize, to: usize) -> Self {
        Self::Range {
            from: Some(from),
            to: Some(to),
        }
    }
}

impl<'a> IncludeLocation<'a> {
    pub fn parse(s: &'a str) -> Self {
        let parts = s.split(':').collect::<Vec<_>>();

        match &parts[..] {
            // file.md
            [path] => Self {
                path,
                range: IncludeRange::Full,
            },
            // file.md:5 | file.md:component
            [path, line] => {
                let range = if let Ok(num) = line.parse() {
                    IncludeRange::line(num)
                } else {
                    IncludeRange::Anchor { name: line }
                };

                Self { path, range }
            }
            // file.md::2
            [path, first, second] if first.is_empty() => {
                let to = second
                    .parse()
                    .map_err(err_to_diagnostic)
                    .expect_or_abort("unable to parse 'to' include range component");
                Self {
                    path,
                    range: IncludeRange::left(to),
                }
            }

            // file.md:2:
            [path, first, second] if second.is_empty() => {
                let from = first
                    .parse()
                    .map_err(err_to_diagnostic)
                    .expect_or_abort("unable to parse 'from' include range component");
                Self {
                    path,
                    range: IncludeRange::right(from),
                }
            }

            // file.md:2:10
            [path, first, second] => {
                let from = first
                    .parse()
                    .map_err(err_to_diagnostic)
                    .expect_or_abort("unable to parse 'from' include range component");
                let to = second
                    .parse()
                    .map_err(err_to_diagnostic)
                    .expect_or_abort("unable to parse 'to' include range component");

                Self {
                    path,
                    range: IncludeRange::range(from, to),
                }
            }

            _ => abort_call_site!("unknown file include range layout"),
        }
    }
}

#[test]
fn test_parse_include_location() {
    let test_cases = [
        (
            "file.rs",
            IncludeLocation {
                path: "file.rs",
                range: IncludeRange::Full,
            },
        ),
        (
            "file.rs:2",
            IncludeLocation {
                path: "file.rs",
                range: IncludeRange::Range {
                    from: Some(2),
                    to: Some(3),
                },
            },
        ),
        (
            "file.rs::10",
            IncludeLocation {
                path: "file.rs",
                range: IncludeRange::Range {
                    from: None,
                    to: Some(10),
                },
            },
        ),
        (
            "file.rs:2:",
            IncludeLocation {
                path: "file.rs",
                range: IncludeRange::Range {
                    from: Some(2),
                    to: None,
                },
            },
        ),
        (
            "file.rs:2:10",
            IncludeLocation {
                path: "file.rs",
                range: IncludeRange::Range {
                    from: Some(2),
                    to: Some(10),
                },
            },
        ),
        (
            "file.rs:component",
            IncludeLocation {
                path: "file.rs",
                range: IncludeRange::Anchor { name: "component" },
            },
        ),
    ];

    for (path, expected) in test_cases {
        let actual = IncludeLocation::parse(path);
        assert_eq!(actual, expected);
    }
}
