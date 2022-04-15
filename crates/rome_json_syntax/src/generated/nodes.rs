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
pub struct JsonArrayStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsonArrayStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonArrayStatementFields {
        JsonArrayStatementFields {
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
pub struct JsonArrayStatementFields {
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
pub struct JsonObjectStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsonObjectStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonObjectStatementFields {
        JsonObjectStatementFields {
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
pub struct JsonObjectStatementFields {
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
            json_data_value: self.json_data_value(),
        }
    }
    pub fn json_string_literal_expression(&self) -> SyntaxResult<JsonStringLiteralExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn json_data_value(&self) -> SyntaxResult<JsonDataValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
pub struct JsonObjectValueFields {
    pub json_string_literal_expression: SyntaxResult<JsonStringLiteralExpression>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub json_data_value: SyntaxResult<JsonDataValue>,
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
pub enum JsonDataLiteralExpression {
    JsonBooleanLiteralExpression(JsonBooleanLiteralExpression),
    JsonNullLiteralExpression(JsonNullLiteralExpression),
    JsonNumberLiteralExpression(JsonNumberLiteralExpression),
    JsonStringLiteralExpression(JsonStringLiteralExpression),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsonDataValue {
    JsonArrayStatement(JsonArrayStatement),
    JsonDataLiteralExpression(JsonDataLiteralExpression),
    JsonObjectStatement(JsonObjectStatement),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsonRoot {
    JsonArrayStatement(JsonArrayStatement),
    JsonObjectStatement(JsonObjectStatement),
}
impl AstNode<Language> for JsonArrayStatement {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_ARRAY_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonArrayStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonArrayStatement")
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
impl From<JsonArrayStatement> for SyntaxNode {
    fn from(n: JsonArrayStatement) -> SyntaxNode { n.syntax }
}
impl From<JsonArrayStatement> for SyntaxElement {
    fn from(n: JsonArrayStatement) -> SyntaxElement { n.syntax.into() }
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
impl AstNode<Language> for JsonObjectStatement {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_OBJECT_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonObjectStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonObjectStatement")
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
impl From<JsonObjectStatement> for SyntaxNode {
    fn from(n: JsonObjectStatement) -> SyntaxNode { n.syntax }
}
impl From<JsonObjectStatement> for SyntaxElement {
    fn from(n: JsonObjectStatement) -> SyntaxElement { n.syntax.into() }
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
                "json_data_value",
                &support::DebugSyntaxResult(self.json_data_value()),
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
impl From<JsonBooleanLiteralExpression> for JsonDataLiteralExpression {
    fn from(node: JsonBooleanLiteralExpression) -> JsonDataLiteralExpression {
        JsonDataLiteralExpression::JsonBooleanLiteralExpression(node)
    }
}
impl From<JsonNullLiteralExpression> for JsonDataLiteralExpression {
    fn from(node: JsonNullLiteralExpression) -> JsonDataLiteralExpression {
        JsonDataLiteralExpression::JsonNullLiteralExpression(node)
    }
}
impl From<JsonNumberLiteralExpression> for JsonDataLiteralExpression {
    fn from(node: JsonNumberLiteralExpression) -> JsonDataLiteralExpression {
        JsonDataLiteralExpression::JsonNumberLiteralExpression(node)
    }
}
impl From<JsonStringLiteralExpression> for JsonDataLiteralExpression {
    fn from(node: JsonStringLiteralExpression) -> JsonDataLiteralExpression {
        JsonDataLiteralExpression::JsonStringLiteralExpression(node)
    }
}
impl AstNode<Language> for JsonDataLiteralExpression {
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
                JsonDataLiteralExpression::JsonBooleanLiteralExpression(
                    JsonBooleanLiteralExpression { syntax },
                )
            }
            JSON_NULL_LITERAL_EXPRESSION => {
                JsonDataLiteralExpression::JsonNullLiteralExpression(JsonNullLiteralExpression {
                    syntax,
                })
            }
            JSON_NUMBER_LITERAL_EXPRESSION => {
                JsonDataLiteralExpression::JsonNumberLiteralExpression(
                    JsonNumberLiteralExpression { syntax },
                )
            }
            JSON_STRING_LITERAL_EXPRESSION => {
                JsonDataLiteralExpression::JsonStringLiteralExpression(
                    JsonStringLiteralExpression { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsonDataLiteralExpression::JsonBooleanLiteralExpression(it) => &it.syntax,
            JsonDataLiteralExpression::JsonNullLiteralExpression(it) => &it.syntax,
            JsonDataLiteralExpression::JsonNumberLiteralExpression(it) => &it.syntax,
            JsonDataLiteralExpression::JsonStringLiteralExpression(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsonDataLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonDataLiteralExpression::JsonBooleanLiteralExpression(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsonDataLiteralExpression::JsonNullLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsonDataLiteralExpression::JsonNumberLiteralExpression(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsonDataLiteralExpression::JsonStringLiteralExpression(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<JsonDataLiteralExpression> for SyntaxNode {
    fn from(n: JsonDataLiteralExpression) -> SyntaxNode {
        match n {
            JsonDataLiteralExpression::JsonBooleanLiteralExpression(it) => it.into(),
            JsonDataLiteralExpression::JsonNullLiteralExpression(it) => it.into(),
            JsonDataLiteralExpression::JsonNumberLiteralExpression(it) => it.into(),
            JsonDataLiteralExpression::JsonStringLiteralExpression(it) => it.into(),
        }
    }
}
impl From<JsonDataLiteralExpression> for SyntaxElement {
    fn from(n: JsonDataLiteralExpression) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsonArrayStatement> for JsonDataValue {
    fn from(node: JsonArrayStatement) -> JsonDataValue { JsonDataValue::JsonArrayStatement(node) }
}
impl From<JsonObjectStatement> for JsonDataValue {
    fn from(node: JsonObjectStatement) -> JsonDataValue { JsonDataValue::JsonObjectStatement(node) }
}
impl AstNode<Language> for JsonDataValue {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            JSON_ARRAY_STATEMENT | JSON_OBJECT_STATEMENT => true,
            k if JsonDataLiteralExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JSON_ARRAY_STATEMENT => {
                JsonDataValue::JsonArrayStatement(JsonArrayStatement { syntax })
            }
            JSON_OBJECT_STATEMENT => {
                JsonDataValue::JsonObjectStatement(JsonObjectStatement { syntax })
            }
            _ => {
                if let Some(json_data_literal_expression) = JsonDataLiteralExpression::cast(syntax)
                {
                    return Some(JsonDataValue::JsonDataLiteralExpression(
                        json_data_literal_expression,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsonDataValue::JsonArrayStatement(it) => &it.syntax,
            JsonDataValue::JsonObjectStatement(it) => &it.syntax,
            JsonDataValue::JsonDataLiteralExpression(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsonDataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonDataValue::JsonArrayStatement(it) => std::fmt::Debug::fmt(it, f),
            JsonDataValue::JsonDataLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsonDataValue::JsonObjectStatement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsonDataValue> for SyntaxNode {
    fn from(n: JsonDataValue) -> SyntaxNode {
        match n {
            JsonDataValue::JsonArrayStatement(it) => it.into(),
            JsonDataValue::JsonDataLiteralExpression(it) => it.into(),
            JsonDataValue::JsonObjectStatement(it) => it.into(),
        }
    }
}
impl From<JsonDataValue> for SyntaxElement {
    fn from(n: JsonDataValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsonArrayStatement> for JsonRoot {
    fn from(node: JsonArrayStatement) -> JsonRoot { JsonRoot::JsonArrayStatement(node) }
}
impl From<JsonObjectStatement> for JsonRoot {
    fn from(node: JsonObjectStatement) -> JsonRoot { JsonRoot::JsonObjectStatement(node) }
}
impl AstNode<Language> for JsonRoot {
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, JSON_ARRAY_STATEMENT | JSON_OBJECT_STATEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JSON_ARRAY_STATEMENT => JsonRoot::JsonArrayStatement(JsonArrayStatement { syntax }),
            JSON_OBJECT_STATEMENT => JsonRoot::JsonObjectStatement(JsonObjectStatement { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsonRoot::JsonArrayStatement(it) => &it.syntax,
            JsonRoot::JsonObjectStatement(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonRoot::JsonArrayStatement(it) => std::fmt::Debug::fmt(it, f),
            JsonRoot::JsonObjectStatement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsonRoot> for SyntaxNode {
    fn from(n: JsonRoot) -> SyntaxNode {
        match n {
            JsonRoot::JsonArrayStatement(it) => it.into(),
            JsonRoot::JsonObjectStatement(it) => it.into(),
        }
    }
}
impl From<JsonRoot> for SyntaxElement {
    fn from(n: JsonRoot) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for JsonDataLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonDataValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonArrayStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonBooleanLiteralExpression {
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
impl std::fmt::Display for JsonObjectStatement {
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
impl AstSeparatedList<Language, JsonDataValue> for JsonArrayValueList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsonArrayValueList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsonArrayValueList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsonArrayValueList {
    type Item = SyntaxResult<JsonDataValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonDataValue>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsonArrayValueList {
    type Item = SyntaxResult<JsonDataValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonDataValue>;
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
