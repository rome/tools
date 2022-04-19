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
            elements: self.elements(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> JsonArrayElementList { support::list(&self.syntax, 1usize) }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
pub struct JsonArrayFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub elements: JsonArrayElementList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonBoolean {
    pub(crate) syntax: SyntaxNode,
}
impl JsonBoolean {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonBooleanFields {
        JsonBooleanFields {
            true_token: self.true_token(),
            false_token: self.false_token(),
        }
    }
    pub fn true_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn false_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
pub struct JsonBooleanFields {
    pub true_token: SyntaxResult<SyntaxToken>,
    pub false_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsonMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonMemberFields {
        JsonMemberFields {
            key_token: self.key_token(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn key_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<JsonValue> { support::required_node(&self.syntax, 2usize) }
}
pub struct JsonMemberFields {
    pub key_token: SyntaxResult<SyntaxToken>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<JsonValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonNull {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNull {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonNullFields {
        JsonNullFields {
            null_token: self.null_token(),
        }
    }
    pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
pub struct JsonNullFields {
    pub null_token: SyntaxResult<SyntaxToken>,
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
            json_member_list: self.json_member_list(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn json_member_list(&self) -> JsonMemberList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
pub struct JsonObjectFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub json_member_list: JsonMemberList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonRoot {
    pub(crate) syntax: SyntaxNode,
}
impl JsonRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonRootFields {
        JsonRootFields {
            json_value: self.json_value(),
        }
    }
    pub fn json_value(&self) -> SyntaxResult<JsonValue> {
        support::required_node(&self.syntax, 0usize)
    }
}
pub struct JsonRootFields {
    pub json_value: SyntaxResult<JsonValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonValueFields {
        JsonValueFields {
            json_string_token: self.json_string_token(),
            json_boolean: self.json_boolean(),
            json_null: self.json_null(),
            json_number_token: self.json_number_token(),
            json_array: self.json_array(),
            json_object: self.json_object(),
        }
    }
    pub fn json_string_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn json_boolean(&self) -> SyntaxResult<JsonBoolean> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn json_null(&self) -> SyntaxResult<JsonNull> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn json_number_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn json_array(&self) -> SyntaxResult<JsonArray> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn json_object(&self) -> SyntaxResult<JsonObject> {
        support::required_node(&self.syntax, 5usize)
    }
}
pub struct JsonValueFields {
    pub json_string_token: SyntaxResult<SyntaxToken>,
    pub json_boolean: SyntaxResult<JsonBoolean>,
    pub json_null: SyntaxResult<JsonNull>,
    pub json_number_token: SyntaxResult<SyntaxToken>,
    pub json_array: SyntaxResult<JsonArray>,
    pub json_object: SyntaxResult<JsonObject>,
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
            .field("elements", &self.elements())
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
impl AstNode<Language> for JsonBoolean {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_BOOLEAN }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonBoolean")
            .field("true_token", &support::DebugSyntaxResult(self.true_token()))
            .field(
                "false_token",
                &support::DebugSyntaxResult(self.false_token()),
            )
            .finish()
    }
}
impl From<JsonBoolean> for SyntaxNode {
    fn from(n: JsonBoolean) -> SyntaxNode { n.syntax }
}
impl From<JsonBoolean> for SyntaxElement {
    fn from(n: JsonBoolean) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonMember {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonMember")
            .field("key_token", &support::DebugSyntaxResult(self.key_token()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<JsonMember> for SyntaxNode {
    fn from(n: JsonMember) -> SyntaxNode { n.syntax }
}
impl From<JsonMember> for SyntaxElement {
    fn from(n: JsonMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonNull {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_NULL }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonNull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonNull")
            .field("null_token", &support::DebugSyntaxResult(self.null_token()))
            .finish()
    }
}
impl From<JsonNull> for SyntaxNode {
    fn from(n: JsonNull) -> SyntaxNode { n.syntax }
}
impl From<JsonNull> for SyntaxElement {
    fn from(n: JsonNull) -> SyntaxElement { n.syntax.into() }
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
            .field("json_member_list", &self.json_member_list())
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
impl AstNode<Language> for JsonRoot {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_ROOT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonRoot")
            .field("json_value", &support::DebugSyntaxResult(self.json_value()))
            .finish()
    }
}
impl From<JsonRoot> for SyntaxNode {
    fn from(n: JsonRoot) -> SyntaxNode { n.syntax }
}
impl From<JsonRoot> for SyntaxElement {
    fn from(n: JsonRoot) -> SyntaxElement { n.syntax.into() }
}
impl AstNode<Language> for JsonValue {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonValue")
            .field(
                "json_string_token",
                &support::DebugSyntaxResult(self.json_string_token()),
            )
            .field(
                "json_boolean",
                &support::DebugSyntaxResult(self.json_boolean()),
            )
            .field("json_null", &support::DebugSyntaxResult(self.json_null()))
            .field(
                "json_number_token",
                &support::DebugSyntaxResult(self.json_number_token()),
            )
            .field("json_array", &support::DebugSyntaxResult(self.json_array()))
            .field(
                "json_object",
                &support::DebugSyntaxResult(self.json_object()),
            )
            .finish()
    }
}
impl From<JsonValue> for SyntaxNode {
    fn from(n: JsonValue) -> SyntaxNode { n.syntax }
}
impl From<JsonValue> for SyntaxElement {
    fn from(n: JsonValue) -> SyntaxElement { n.syntax.into() }
}
impl std::fmt::Display for JsonArray {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonNull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonValue {
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
pub struct JsonArrayElementList {
    syntax_list: SyntaxList,
}
impl JsonArrayElementList {
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
impl AstNode<Language> for JsonArrayElementList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_ARRAY_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsonArrayElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsonArrayElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<Language, JsonValue> for JsonArrayElementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsonArrayElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsonArrayElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsonArrayElementList {
    type Item = SyntaxResult<JsonValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonValue>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsonArrayElementList {
    type Item = SyntaxResult<JsonValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonValue>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsonMemberList {
    syntax_list: SyntaxList,
}
impl JsonMemberList {
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
impl AstNode<Language> for JsonMemberList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_MEMBER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsonMemberList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsonMemberList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<Language, JsonMember> for JsonMemberList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsonMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsonMemberList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsonMemberList {
    type Item = SyntaxResult<JsonMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsonMemberList {
    type Item = SyntaxResult<JsonMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonMember>;
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
