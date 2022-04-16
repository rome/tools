use crate::kinds_src::KindsSrc;

pub const JSON_KINDS_SRC: KindsSrc = KindsSrc {
    punct: &[
        (",", "COMMA"),
        ("{", "L_CURLY"),
        ("}", "R_CURLY"),
        ("[", "L_BRACK"),
        ("]", "R_BRACK"),
    ],
    keywords: &[],
    literals: &[
        "JSON_STRING_LITERAL",
        "JSON_NUMBER_LITERAL",
        "JSON_BOOL_LITERAL",
        "JSON_NULL_LITERAL",
    ],
    tokens: &["WHITESPACE"],
    nodes: &[
        "JSON_UNKNOWN",
        "JSON_DOCUMENT",
        "JSON_OBJECT",
        "JSON_OBJECT_VALUE_LIST",
        "JSON_OBJECT_VALUE",
        "JSON_ARRAY",
        "JSON_ARRAY_VALUE_LIST",
        "JSON_ANY_VALUE",
        "JSON_LITERAL_EXPRESSION",
        "JSON_STRING_LITERAL_EXPRESSION",
        "JSON_NUMBER_LITERAL_EXPRESSION",
        "JSON_BOOLEAN_LITERAL_EXPRESSION",
        "JSON_NULL_LITERAL_EXPRESSION",
    ],
};
