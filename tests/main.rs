//! Tests for include-utils-macro

use include_utils_macro::{include_md, include_str_part};

#[test]
fn test_include_str_line() {
    assert_eq!(
        include_str_part!("tests/data/markdown_with_header.md:3"),
        "Some needless line"
    );
}

#[test]
fn test_include_str_part() {
    assert_eq!(
        include_str_part!("tests/data/markdown_with_header.md:5:9"),
        "```rust\nfn main() {\n    \n}\n```"
    );
}

#[test]
fn test_include_str_begin() {
    assert_eq!(
        include_str_part!("tests/data/markdown_with_header.md::3"),
        "# Some needless header\n\nSome needless line"
    );
}

#[test]
fn test_include_str_end() {
    assert_eq!(
        include_str_part!("tests/data/markdown_with_header.md:16:"),
        "4. He played soccer with his friends in the park.\n5. The dog barked loudly at the mailman."
    );
}

#[test]
fn test_include_md_anchor_all() {
    assert_eq!(
        include_md!("tests/data/anchor.md:all"),
        "all text\n\n<!-- ANCHOR: conclusion -->\n\nconclusion\n\n<!-- ANCHOR_END: conclusion -->"
    );
}

#[test]
fn test_include_md_anchor_conclusion() {
    assert_eq!(include_md!("tests/data/anchor.md:conclusion"), "conclusion");
}
