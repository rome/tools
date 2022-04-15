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
        "JSON_ROOT",
        "JSON_OBJECT_STATEMENT",  //
        "JSON_OBJECT_VALUE_LIST", //
        "JSON_OBJECT_VALUE",      //
        "JSON_ARRAY_STATEMENT",   //
        "JSON_ARRAY_VALUE_LIST",  //
        "JSON_DATA_VALUE",
        "JSON_DATA_LITERAL_EXPRESSION",
        "JSON_STRING_LITERAL_EXPRESSION",  //
        "JSON_NUMBER_LITERAL_EXPRESSION",  //
        "JSON_BOOLEAN_LITERAL_EXPRESSION", //
        "JSON_NULL_LITERAL_EXPRESSION",    //
    ],
};
