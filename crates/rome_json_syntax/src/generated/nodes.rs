//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    JsonLanguage as Language, JsonSyntaxElement as SyntaxElement,
    JsonSyntaxElementChildren as SyntaxElementChildren,
    JsonSyntaxKind::{self as SyntaxKind, *},
    JsonSyntaxList as SyntaxList, JsonSyntaxNode as SyntaxNode, JsonSyntaxToken as SyntaxToken,
};
use rome_rowan::{
    support, AstNode, AstNodeList, AstNodeListIterator, AstSeparatedList,
    AstSeparatedListNodesIterator, SyntaxResult,
};
use std::fmt::{Debug, Formatter};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonArray {
    pub(crate) syntax: SyntaxNode,
}
impl JsonArray {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonArrayFields {
        JsonArrayFields {
            l_brack_token: self.l_brack_token(),
            json_array_value_list: self.json_array_value_list(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn json_array_value_list(&self) -> JsonArrayValueList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
pub struct JsonArrayFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub json_array_value_list: JsonArrayValueList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonBooleanLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsonBooleanLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonBooleanLiteralExpressionFields {
        JsonBooleanLiteralExpressionFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
pub struct JsonBooleanLiteralExpressionFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonDocument {
    pub(crate) syntax: SyntaxNode,
}
impl JsonDocument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonDocumentFields {
        JsonDocumentFields {
            value: self.value(),
            json_unknown: self.json_unknown(),
            eof_token: self.eof_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<JsonAnyValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn json_unknown(&self) -> SyntaxResult<JsonUnknown> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
pub struct JsonDocumentFields {
    pub value: SyntaxResult<JsonAnyValue>,
    pub json_unknown: SyntaxResult<JsonUnknown>,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonNullLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNullLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonNullLiteralExpressionFields {
        JsonNullLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
pub struct JsonNullLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonNumberLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNumberLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonNumberLiteralExpressionFields {
        JsonNumberLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
pub struct JsonNumberLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonObject {
    pub(crate) syntax: SyntaxNode,
}
impl JsonObject {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonObjectFields {
        JsonObjectFields {
            l_curly_token: self.l_curly_token(),
            json_object_value_list: self.json_object_value_list(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn json_object_value_list(&self) -> JsonObjectValueList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
pub struct JsonObjectFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub json_object_value_list: JsonObjectValueList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonObjectValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonObjectValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonObjectValueFields {
        JsonObjectValueFields {
            json_string_literal_expression: self.json_string_literal_expression(),
            colon_token: self.colon_token(),
            json_any_value: self.json_any_value(),
        }
    }
    pub fn json_string_literal_expression(&self) -> SyntaxResult<JsonStringLiteralExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn json_any_value(&self) -> SyntaxResult<JsonAnyValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
pub struct JsonObjectValueFields {
    pub json_string_literal_expression: SyntaxResult<JsonStringLiteralExpression>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub json_any_value: SyntaxResult<JsonAnyValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonStringLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsonStringLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonStringLiteralExpressionFields {
        JsonStringLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
pub struct JsonStringLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsonAnyValue {
    JsonArray(JsonArray),
    JsonLiteralExpression(JsonLiteralExpression),
    JsonObject(JsonObject),
    JsonUnknown(JsonUnknown),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsonLiteralExpression {
    JsonBooleanLiteralExpression(JsonBooleanLiteralExpression),
    JsonNullLiteralExpression(JsonNullLiteralExpression),
    JsonNumberLiteralExpression(JsonNumberLiteralExpression),
    JsonStringLiteralExpression(JsonStringLiteralExpression),
}
impl AstNode<Language> for JsonArray {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_ARRAY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonArray")
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("json_array_value_list", &self.json_array_value_list())
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<JsonArray> for SyntaxNode {
    fn from(n: JsonArray) -> SyntaxNode { n.syntax }
}
impl From<JsonArray> for SyntaxElement {
    fn from(n: JsonArray) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonBooleanLiteralExpression {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_BOOLEAN_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonBooleanLiteralExpression")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<JsonBooleanLiteralExpression> for SyntaxNode {
    fn from(n: JsonBooleanLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsonBooleanLiteralExpression> for SyntaxElement {
    fn from(n: JsonBooleanLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonDocument {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_DOCUMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonDocument")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field(
                "json_unknown",
                &support::DebugSyntaxResult(self.json_unknown()),
            )
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<JsonDocument> for SyntaxNode {
    fn from(n: JsonDocument) -> SyntaxNode { n.syntax }
}
impl From<JsonDocument> for SyntaxElement {
    fn from(n: JsonDocument) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonNullLiteralExpression {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_NULL_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonNullLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonNullLiteralExpression> for SyntaxNode {
    fn from(n: JsonNullLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsonNullLiteralExpression> for SyntaxElement {
    fn from(n: JsonNullLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonNumberLiteralExpression {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_NUMBER_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonNumberLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonNumberLiteralExpression> for SyntaxNode {
    fn from(n: JsonNumberLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsonNumberLiteralExpression> for SyntaxElement {
    fn from(n: JsonNumberLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonObject {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_OBJECT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonObject")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("json_object_value_list", &self.json_object_value_list())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsonObject> for SyntaxNode {
    fn from(n: JsonObject) -> SyntaxNode { n.syntax }
}
impl From<JsonObject> for SyntaxElement {
    fn from(n: JsonObject) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonObjectValue {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_OBJECT_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonObjectValue")
            .field(
                "json_string_literal_expression",
                &support::DebugSyntaxResult(self.json_string_literal_expression()),
            )
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field(
                "json_any_value",
                &support::DebugSyntaxResult(self.json_any_value()),
            )
            .finish()
    }
}
impl From<JsonObjectValue> for SyntaxNode {
    fn from(n: JsonObjectValue) -> SyntaxNode { n.syntax }
}
impl From<JsonObjectValue> for SyntaxElement {
    fn from(n: JsonObjectValue) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonStringLiteralExpression {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_STRING_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonStringLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonStringLiteralExpression> for SyntaxNode {
    fn from(n: JsonStringLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsonStringLiteralExpression> for SyntaxElement {
    fn from(n: JsonStringLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl From<JsonArray> for JsonAnyValue {
    fn from(node: JsonArray) -> JsonAnyValue { JsonAnyValue::JsonArray(node) }
}
impl From<JsonObject> for JsonAnyValue {
    fn from(node: JsonObject) -> JsonAnyValue { JsonAnyValue::JsonObject(node) }
}
impl From<JsonUnknown> for JsonAnyValue {
    fn from(node: JsonUnknown) -> JsonAnyValue { JsonAnyValue::JsonUnknown(node) }
}
impl AstNode<Language> for JsonAnyValue {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            JSON_ARRAY | JSON_OBJECT | JSON_UNKNOWN => true,
            k if JsonLiteralExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JSON_ARRAY => JsonAnyValue::JsonArray(JsonArray { syntax }),
            JSON_OBJECT => JsonAnyValue::JsonObject(JsonObject { syntax }),
            JSON_UNKNOWN => JsonAnyValue::JsonUnknown(JsonUnknown { syntax }),
            _ => {
                if let Some(json_literal_expression) = JsonLiteralExpression::cast(syntax) {
                    return Some(JsonAnyValue::JsonLiteralExpression(json_literal_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsonAnyValue::JsonArray(it) => &it.syntax,
            JsonAnyValue::JsonObject(it) => &it.syntax,
            JsonAnyValue::JsonUnknown(it) => &it.syntax,
            JsonAnyValue::JsonLiteralExpression(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsonAnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonAnyValue::JsonArray(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonObject(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonUnknown(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsonAnyValue> for SyntaxNode {
    fn from(n: JsonAnyValue) -> SyntaxNode {
        match n {
            JsonAnyValue::JsonArray(it) => it.into(),
            JsonAnyValue::JsonLiteralExpression(it) => it.into(),
            JsonAnyValue::JsonObject(it) => it.into(),
            JsonAnyValue::JsonUnknown(it) => it.into(),
        }
    }
}
impl From<JsonAnyValue> for SyntaxElement {
    fn from(n: JsonAnyValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsonBooleanLiteralExpression> for JsonLiteralExpression {
    fn from(node: JsonBooleanLiteralExpression) -> JsonLiteralExpression {
        JsonLiteralExpression::JsonBooleanLiteralExpression(node)
    }
}
impl From<JsonNullLiteralExpression> for JsonLiteralExpression {
    fn from(node: JsonNullLiteralExpression) -> JsonLiteralExpression {
        JsonLiteralExpression::JsonNullLiteralExpression(node)
    }
}
impl From<JsonNumberLiteralExpression> for JsonLiteralExpression {
    fn from(node: JsonNumberLiteralExpression) -> JsonLiteralExpression {
        JsonLiteralExpression::JsonNumberLiteralExpression(node)
    }
}
impl From<JsonStringLiteralExpression> for JsonLiteralExpression {
    fn from(node: JsonStringLiteralExpression) -> JsonLiteralExpression {
        JsonLiteralExpression::JsonStringLiteralExpression(node)
    }
}
impl AstNode<Language> for JsonLiteralExpression {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            JSON_BOOLEAN_LITERAL_EXPRESSION
                | JSON_NULL_LITERAL_EXPRESSION
                | JSON_NUMBER_LITERAL_EXPRESSION
                | JSON_STRING_LITERAL_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JSON_BOOLEAN_LITERAL_EXPRESSION => {
                JsonLiteralExpression::JsonBooleanLiteralExpression(JsonBooleanLiteralExpression {
                    syntax,
                })
            }
            JSON_NULL_LITERAL_EXPRESSION => {
                JsonLiteralExpression::JsonNullLiteralExpression(JsonNullLiteralExpression {
                    syntax,
                })
            }
            JSON_NUMBER_LITERAL_EXPRESSION => {
                JsonLiteralExpression::JsonNumberLiteralExpression(JsonNumberLiteralExpression {
                    syntax,
                })
            }
            JSON_STRING_LITERAL_EXPRESSION => {
                JsonLiteralExpression::JsonStringLiteralExpression(JsonStringLiteralExpression {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsonLiteralExpression::JsonBooleanLiteralExpression(it) => &it.syntax,
            JsonLiteralExpression::JsonNullLiteralExpression(it) => &it.syntax,
            JsonLiteralExpression::JsonNumberLiteralExpression(it) => &it.syntax,
            JsonLiteralExpression::JsonStringLiteralExpression(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsonLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonLiteralExpression::JsonBooleanLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsonLiteralExpression::JsonNullLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsonLiteralExpression::JsonNumberLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsonLiteralExpression::JsonStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsonLiteralExpression> for SyntaxNode {
    fn from(n: JsonLiteralExpression) -> SyntaxNode {
        match n {
            JsonLiteralExpression::JsonBooleanLiteralExpression(it) => it.into(),
            JsonLiteralExpression::JsonNullLiteralExpression(it) => it.into(),
            JsonLiteralExpression::JsonNumberLiteralExpression(it) => it.into(),
            JsonLiteralExpression::JsonStringLiteralExpression(it) => it.into(),
        }
    }
}
impl From<JsonLiteralExpression> for SyntaxElement {
    fn from(n: JsonLiteralExpression) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for JsonAnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonUnknown {
    syntax: SyntaxNode,
}
impl JsonUnknown {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode<Language> for JsonUnknown {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_UNKNOWN }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonUnknown")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsonUnknown> for SyntaxNode {
    fn from(n: JsonUnknown) -> SyntaxNode { n.syntax }
}
impl From<JsonUnknown> for SyntaxElement {
    fn from(n: JsonUnknown) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsonArrayValueList {
    syntax_list: SyntaxList,
}
impl JsonArrayValueList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode<Language> for JsonArrayValueList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_ARRAY_VALUE_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsonArrayValueList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsonArrayValueList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<Language, JsonAnyValue> for JsonArrayValueList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsonArrayValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsonArrayValueList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsonArrayValueList {
    type Item = SyntaxResult<JsonAnyValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonAnyValue>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsonArrayValueList {
    type Item = SyntaxResult<JsonAnyValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonAnyValue>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsonObjectValueList {
    syntax_list: SyntaxList,
}
impl JsonObjectValueList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode<Language> for JsonObjectValueList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_OBJECT_VALUE_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsonObjectValueList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsonObjectValueList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<Language, JsonObjectValue> for JsonObjectValueList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsonObjectValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsonObjectValueList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsonObjectValueList {
    type Item = SyntaxResult<JsonObjectValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonObjectValue>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsonObjectValueList {
    type Item = SyntaxResult<JsonObjectValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonObjectValue>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
