use crate::kinds_src::KindsSrc;

pub const JSON_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (":", "COLON"),
        (",", "COMMA"),
        ("(", "L_PAREN"),
        (")", "R_PAREN"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
    ],
    keywords: &["null", "true", "false"],
    literals: &["JSON_STRING_LITERAL", "JSON_NUMBER_LITERAL"],
    // keep comment token for json 5 extension
    tokens: &["ERROR_TOKEN", "NEWLINE", "WHITESPACE", "COMMENT"],
    nodes: &[
        "JSON_ROOT",
        "JSON_VALUE",
        "JSON_NUMBER",
        "JSON_STRING",
        "JSON_BOOLEAN",
        "JSON_NULL",
        "JSON_ARRAY",
        "JSON_OBJECT",
        "JSON_MEMBER_LIST",
        "JSON_MEMBER",
        "JSON_ARRAY_ELEMENT_LIST",
        // unknown nodes
        "JSON_UNKNOWN",
    ],
};
