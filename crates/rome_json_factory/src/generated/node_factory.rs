//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use rome_json_syntax::{
    JsonSyntaxElement as SyntaxElement, JsonSyntaxNode as SyntaxNode,
    JsonSyntaxToken as SyntaxToken, *,
};
use rome_rowan::AstNode;
pub fn json_array(
    l_brack_token: SyntaxToken,
    elements: JsonArrayElementList,
    r_brack_token: SyntaxToken,
) -> JsonArray {
    JsonArray::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_ARRAY,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn json_boolean(value_token_token: SyntaxToken) -> JsonBoolean {
    JsonBoolean::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_BOOLEAN,
        [Some(SyntaxElement::Token(value_token_token))],
    ))
}
pub fn json_member(key: JsonString, colon_token: SyntaxToken, value: JsonValue) -> JsonMember {
    JsonMember::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_MEMBER,
        [
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn json_null(null_token: SyntaxToken) -> JsonNull {
    JsonNull::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_NULL,
        [Some(SyntaxElement::Token(null_token))],
    ))
}
pub fn json_number(json_number_literal_token: SyntaxToken) -> JsonNumber {
    JsonNumber::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_NUMBER,
        [Some(SyntaxElement::Token(json_number_literal_token))],
    ))
}
pub fn json_object(
    l_curly_token: SyntaxToken,
    json_member_list: JsonMemberList,
    r_curly_token: SyntaxToken,
) -> JsonObject {
    JsonObject::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_OBJECT,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(json_member_list.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn json_root(json_value: JsonValue, eof_token: SyntaxToken) -> JsonRoot {
    JsonRoot::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_ROOT,
        [
            Some(SyntaxElement::Node(json_value.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn json_string(json_string_literal_token: SyntaxToken) -> JsonString {
    JsonString::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_STRING,
        [Some(SyntaxElement::Token(json_string_literal_token))],
    ))
}
pub fn json_array_element_list<I>(items: I) -> JsonArrayElementList
where
    I: IntoIterator<Item = (JsonValue, Option<JsonSyntaxToken>)>,
    I::IntoIter: ExactSizeIterator,
{
    let items = items.into_iter();
    let length = items.len() * 2;
    let mut iter = items.flat_map(|(item, separator)| {
        [
            Some(item.into_syntax().into()),
            separator.map(|token| token.into()),
        ]
    });
    JsonArrayElementList::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_ARRAY_ELEMENT_LIST,
        (0..length).map(|_| iter.next().unwrap()),
    ))
}
pub fn json_member_list<I>(items: I) -> JsonMemberList
where
    I: IntoIterator<Item = (JsonMember, Option<JsonSyntaxToken>)>,
    I::IntoIter: ExactSizeIterator,
{
    let items = items.into_iter();
    let length = items.len() * 2;
    let mut iter = items.flat_map(|(item, separator)| {
        [
            Some(item.into_syntax().into()),
            separator.map(|token| token.into()),
        ]
    });
    JsonMemberList::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_MEMBER_LIST,
        (0..length).map(|_| iter.next().unwrap()),
    ))
}
