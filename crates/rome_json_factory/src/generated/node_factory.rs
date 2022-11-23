//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use rome_json_syntax::{
    JsonSyntaxElement as SyntaxElement, JsonSyntaxNode as SyntaxNode,
    JsonSyntaxToken as SyntaxToken, *,
};
use rome_rowan::AstNode;
pub fn json_array(l_brack_token: SyntaxToken, r_brack_token: SyntaxToken) -> JsonArrayBuilder {
    JsonArrayBuilder {
        l_brack_token,
        r_brack_token,
        elements: None,
    }
}
pub struct JsonArrayBuilder {
    l_brack_token: SyntaxToken,
    r_brack_token: SyntaxToken,
    elements: Option<JsonArrayElementList>,
}
impl JsonArrayBuilder {
    pub fn with_elements(mut self, elements: JsonArrayElementList) -> Self {
        self.elements = Some(elements);
        self
    }
    pub fn build(self) -> JsonArray {
        JsonArray::unwrap_cast(SyntaxNode::new_detached(
            JsonSyntaxKind::JSON_ARRAY,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                self.elements
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
            ],
        ))
    }
}
pub fn json_boolean(value_token: SyntaxToken) -> JsonBoolean {
    JsonBoolean::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_BOOLEAN,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn json_member(key: JsonString, colon_token: SyntaxToken, value: JsonAnyValue) -> JsonMember {
    JsonMember::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_MEMBER,
        [
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn json_null(value_token: SyntaxToken) -> JsonNull {
    JsonNull::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_NULL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn json_number(value_token: SyntaxToken) -> JsonNumber {
    JsonNumber::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_NUMBER,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn json_object(l_curly_token: SyntaxToken, r_curly_token: SyntaxToken) -> JsonObjectBuilder {
    JsonObjectBuilder {
        l_curly_token,
        r_curly_token,
        json_member_list: None,
    }
}
pub struct JsonObjectBuilder {
    l_curly_token: SyntaxToken,
    r_curly_token: SyntaxToken,
    json_member_list: Option<JsonMemberList>,
}
impl JsonObjectBuilder {
    pub fn with_json_member_list(mut self, json_member_list: JsonMemberList) -> Self {
        self.json_member_list = Some(json_member_list);
        self
    }
    pub fn build(self) -> JsonObject {
        JsonObject::unwrap_cast(SyntaxNode::new_detached(
            JsonSyntaxKind::JSON_OBJECT,
            [
                Some(SyntaxElement::Token(self.l_curly_token)),
                self.json_member_list
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_curly_token)),
            ],
        ))
    }
}
pub fn json_root(value: JsonAnyValue, eof_token: SyntaxToken) -> JsonRoot {
    JsonRoot::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_ROOT,
        [
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn json_string(value_token: SyntaxToken) -> JsonString {
    JsonString::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn json_array_element_list<I, S>(items: I, separators: S) -> JsonArrayElementList
where
    I: IntoIterator<Item = JsonAnyValue>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsonSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsonArrayElementList::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_ARRAY_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn json_member_list<I, S>(items: I, separators: S) -> JsonMemberList
where
    I: IntoIterator<Item = JsonMember>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = JsonSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    JsonMemberList::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_MEMBER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn json_unknown<I>(slots: I) -> JsonUnknown
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    JsonUnknown::unwrap_cast(SyntaxNode::new_detached(
        JsonSyntaxKind::JSON_UNKNOWN,
        slots,
    ))
}
