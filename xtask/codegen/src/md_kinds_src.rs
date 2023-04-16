use crate::kinds_src::KindsSrc;

pub const MD_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (":", "COLON"),
        ("`", "BACKTICK"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
        ("#", "H1"),
        ("##", "H2"),
        ("###", "H3"),
        ("####", "H4"),
        ("#####", "H5"),
    ],
    keywords: &[],
    literals: &["MD_STRING_LITERAL"],
    // keep comment token for json 5 extension
    tokens: &[
        "ERROR_TOKEN",
        "NEWLINE",
        "WHITESPACE",
        "IDENT",
        "COMMENT",
        "MULTILINE_COMMENT",
    ],
    nodes: &[
        "MD_ROOT",
        "MD_HEADING",
        "MD_TEXT",
        "MD_ELEMENT_LIST",
        // Bogus nodes
        "MD_BOGUS",
    ],
};
