use include_utils_macro::{include_str_part, include_md};

#[test]
fn test_include_str_line() {
    assert_eq!(
        include_str_part!("tests/data/markdown_with_header.md:3"),
        "Some needless line\n"
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
fn test_ainclude_str_end() {
    assert_eq!(
        include_str_part!("tests/data/markdown_with_header.md:16:"),
        "4. He played soccer with his friends in the park.\n5. The dog barked loudly at the mailman."
    );
}


#[test]
fn test_include_md_anchor_all() {
    assert_eq!(
        include_md!("tests/data/anchor.md:all"),
        "all text\nconclusion"
    )
}

#[test]
fn test_include_md_anchor_conclusion() {
    assert_eq!(
        include_md!("tests/data/anchor.md:conclusion"),
        "conclusion"
    )
}
