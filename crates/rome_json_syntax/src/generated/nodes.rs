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
use rome_rowan::{support, AstNode, RawSyntaxKind, SyntaxKindSet, SyntaxResult};
#[allow(unused)]
use rome_rowan::{
    AstNodeList, AstNodeListIterator, AstSeparatedList, AstSeparatedListNodesIterator,
};
#[cfg(feature = "serde")]
use serde::ser::SerializeSeq;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonArrayValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonArrayValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonArrayValueFields {
        JsonArrayValueFields {
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
#[cfg(feature = "serde")]
impl Serialize for JsonArrayValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonArrayValueFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub elements: JsonArrayElementList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonBooleanValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonBooleanValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonBooleanValueFields {
        JsonBooleanValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonBooleanValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonBooleanValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
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
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<JsonMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<JsonAnyValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonMemberFields {
    pub name: SyntaxResult<JsonMemberName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<JsonAnyValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl JsonMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonMemberNameFields {
        JsonMemberNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonMemberName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonMemberNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonNullValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNullValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonNullValueFields {
        JsonNullValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonNullValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonNullValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonNumberValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNumberValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonNumberValueFields {
        JsonNumberValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonNumberValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonNumberValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
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
#[cfg(feature = "serde")]
impl Serialize for JsonObjectValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonObjectValueFields {
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
            value: self.value(),
            eof_token: self.eof_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<JsonAnyValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonRootFields {
    pub value: SyntaxResult<JsonAnyValue>,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonStringValue {
    pub(crate) syntax: SyntaxNode,
}
impl JsonStringValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonStringValueFields {
        JsonStringValueFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonStringValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonStringValueFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum JsonAnyValue {
    JsonArrayValue(JsonArrayValue),
    JsonBogusValue(JsonBogusValue),
    JsonBooleanValue(JsonBooleanValue),
    JsonNullValue(JsonNullValue),
    JsonNumberValue(JsonNumberValue),
    JsonObjectValue(JsonObjectValue),
    JsonStringValue(JsonStringValue),
}
impl JsonAnyValue {
    pub fn as_json_array_value(&self) -> Option<&JsonArrayValue> {
        match &self {
            JsonAnyValue::JsonArrayValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_bogus_value(&self) -> Option<&JsonBogusValue> {
        match &self {
            JsonAnyValue::JsonBogusValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_boolean_value(&self) -> Option<&JsonBooleanValue> {
        match &self {
            JsonAnyValue::JsonBooleanValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_null_value(&self) -> Option<&JsonNullValue> {
        match &self {
            JsonAnyValue::JsonNullValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_number_value(&self) -> Option<&JsonNumberValue> {
        match &self {
            JsonAnyValue::JsonNumberValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_object_value(&self) -> Option<&JsonObjectValue> {
        match &self {
            JsonAnyValue::JsonObjectValue(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_string_value(&self) -> Option<&JsonStringValue> {
        match &self {
            JsonAnyValue::JsonStringValue(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for JsonArrayValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_ARRAY_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_ARRAY_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonArrayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonArrayValue")
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
impl From<JsonArrayValue> for SyntaxNode {
    fn from(n: JsonArrayValue) -> SyntaxNode { n.syntax }
}
impl From<JsonArrayValue> for SyntaxElement {
    fn from(n: JsonArrayValue) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonBooleanValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_BOOLEAN_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_BOOLEAN_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonBooleanValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonBooleanValue> for SyntaxNode {
    fn from(n: JsonBooleanValue) -> SyntaxNode { n.syntax }
}
impl From<JsonBooleanValue> for SyntaxElement {
    fn from(n: JsonBooleanValue) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonMember")
            .field("name", &support::DebugSyntaxResult(self.name()))
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
impl AstNode for JsonMemberName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_MEMBER_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_MEMBER_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonMemberName")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonMemberName> for SyntaxNode {
    fn from(n: JsonMemberName) -> SyntaxNode { n.syntax }
}
impl From<JsonMemberName> for SyntaxElement {
    fn from(n: JsonMemberName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonNullValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_NULL_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_NULL_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonNullValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonNullValue> for SyntaxNode {
    fn from(n: JsonNullValue) -> SyntaxNode { n.syntax }
}
impl From<JsonNullValue> for SyntaxElement {
    fn from(n: JsonNullValue) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonNumberValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_NUMBER_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_NUMBER_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonNumberValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonNumberValue> for SyntaxNode {
    fn from(n: JsonNumberValue) -> SyntaxNode { n.syntax }
}
impl From<JsonNumberValue> for SyntaxElement {
    fn from(n: JsonNumberValue) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonObjectValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_OBJECT_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_OBJECT_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonObjectValue")
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
impl From<JsonObjectValue> for SyntaxNode {
    fn from(n: JsonObjectValue) -> SyntaxNode { n.syntax }
}
impl From<JsonObjectValue> for SyntaxElement {
    fn from(n: JsonObjectValue) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_ROOT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonRoot")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<JsonRoot> for SyntaxNode {
    fn from(n: JsonRoot) -> SyntaxNode { n.syntax }
}
impl From<JsonRoot> for SyntaxElement {
    fn from(n: JsonRoot) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonStringValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_STRING_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_STRING_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonStringValue")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonStringValue> for SyntaxNode {
    fn from(n: JsonStringValue) -> SyntaxNode { n.syntax }
}
impl From<JsonStringValue> for SyntaxElement {
    fn from(n: JsonStringValue) -> SyntaxElement { n.syntax.into() }
}
impl From<JsonArrayValue> for JsonAnyValue {
    fn from(node: JsonArrayValue) -> JsonAnyValue { JsonAnyValue::JsonArrayValue(node) }
}
impl From<JsonBogusValue> for JsonAnyValue {
    fn from(node: JsonBogusValue) -> JsonAnyValue { JsonAnyValue::JsonBogusValue(node) }
}
impl From<JsonBooleanValue> for JsonAnyValue {
    fn from(node: JsonBooleanValue) -> JsonAnyValue { JsonAnyValue::JsonBooleanValue(node) }
}
impl From<JsonNullValue> for JsonAnyValue {
    fn from(node: JsonNullValue) -> JsonAnyValue { JsonAnyValue::JsonNullValue(node) }
}
impl From<JsonNumberValue> for JsonAnyValue {
    fn from(node: JsonNumberValue) -> JsonAnyValue { JsonAnyValue::JsonNumberValue(node) }
}
impl From<JsonObjectValue> for JsonAnyValue {
    fn from(node: JsonObjectValue) -> JsonAnyValue { JsonAnyValue::JsonObjectValue(node) }
}
impl From<JsonStringValue> for JsonAnyValue {
    fn from(node: JsonStringValue) -> JsonAnyValue { JsonAnyValue::JsonStringValue(node) }
}
impl AstNode for JsonAnyValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = JsonArrayValue::KIND_SET
        .union(JsonBogusValue::KIND_SET)
        .union(JsonBooleanValue::KIND_SET)
        .union(JsonNullValue::KIND_SET)
        .union(JsonNumberValue::KIND_SET)
        .union(JsonObjectValue::KIND_SET)
        .union(JsonStringValue::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            JSON_ARRAY_VALUE
                | JSON_BOGUS_VALUE
                | JSON_BOOLEAN_VALUE
                | JSON_NULL_VALUE
                | JSON_NUMBER_VALUE
                | JSON_OBJECT_VALUE
                | JSON_STRING_VALUE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JSON_ARRAY_VALUE => JsonAnyValue::JsonArrayValue(JsonArrayValue { syntax }),
            JSON_BOGUS_VALUE => JsonAnyValue::JsonBogusValue(JsonBogusValue { syntax }),
            JSON_BOOLEAN_VALUE => JsonAnyValue::JsonBooleanValue(JsonBooleanValue { syntax }),
            JSON_NULL_VALUE => JsonAnyValue::JsonNullValue(JsonNullValue { syntax }),
            JSON_NUMBER_VALUE => JsonAnyValue::JsonNumberValue(JsonNumberValue { syntax }),
            JSON_OBJECT_VALUE => JsonAnyValue::JsonObjectValue(JsonObjectValue { syntax }),
            JSON_STRING_VALUE => JsonAnyValue::JsonStringValue(JsonStringValue { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsonAnyValue::JsonArrayValue(it) => &it.syntax,
            JsonAnyValue::JsonBogusValue(it) => &it.syntax,
            JsonAnyValue::JsonBooleanValue(it) => &it.syntax,
            JsonAnyValue::JsonNullValue(it) => &it.syntax,
            JsonAnyValue::JsonNumberValue(it) => &it.syntax,
            JsonAnyValue::JsonObjectValue(it) => &it.syntax,
            JsonAnyValue::JsonStringValue(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            JsonAnyValue::JsonArrayValue(it) => it.syntax,
            JsonAnyValue::JsonBogusValue(it) => it.syntax,
            JsonAnyValue::JsonBooleanValue(it) => it.syntax,
            JsonAnyValue::JsonNullValue(it) => it.syntax,
            JsonAnyValue::JsonNumberValue(it) => it.syntax,
            JsonAnyValue::JsonObjectValue(it) => it.syntax,
            JsonAnyValue::JsonStringValue(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for JsonAnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonAnyValue::JsonArrayValue(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonBogusValue(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonBooleanValue(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonNullValue(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonNumberValue(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonObjectValue(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonStringValue(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsonAnyValue> for SyntaxNode {
    fn from(n: JsonAnyValue) -> SyntaxNode {
        match n {
            JsonAnyValue::JsonArrayValue(it) => it.into(),
            JsonAnyValue::JsonBogusValue(it) => it.into(),
            JsonAnyValue::JsonBooleanValue(it) => it.into(),
            JsonAnyValue::JsonNullValue(it) => it.into(),
            JsonAnyValue::JsonNumberValue(it) => it.into(),
            JsonAnyValue::JsonObjectValue(it) => it.into(),
            JsonAnyValue::JsonStringValue(it) => it.into(),
        }
    }
}
impl From<JsonAnyValue> for SyntaxElement {
    fn from(n: JsonAnyValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for JsonAnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonArrayValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonBooleanValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonNullValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonNumberValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonObjectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsonStringValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonBogus {
    syntax: SyntaxNode,
}
impl JsonBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsonBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_BOGUS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsonBogus> for SyntaxNode {
    fn from(n: JsonBogus) -> SyntaxNode { n.syntax }
}
impl From<JsonBogus> for SyntaxElement {
    fn from(n: JsonBogus) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonBogusValue {
    syntax: SyntaxNode,
}
impl JsonBogusValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsonBogusValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_BOGUS_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_BOGUS_VALUE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
    fn into_syntax(self) -> SyntaxNode { self.syntax }
}
impl std::fmt::Debug for JsonBogusValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonBogusValue")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsonBogusValue> for SyntaxNode {
    fn from(n: JsonBogusValue) -> SyntaxNode { n.syntax }
}
impl From<JsonBogusValue> for SyntaxElement {
    fn from(n: JsonBogusValue) -> SyntaxElement { n.syntax.into() }
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
impl AstNode for JsonArrayElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_ARRAY_ELEMENT_LIST as u16));
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
    fn into_syntax(self) -> SyntaxNode { self.syntax_list.into_node() }
}
#[cfg(feature = "serde")]
impl Serialize for JsonArrayElementList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for JsonArrayElementList {
    type Language = Language;
    type Node = JsonAnyValue;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for JsonArrayElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsonArrayElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsonArrayElementList {
    type Item = SyntaxResult<JsonAnyValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonAnyValue>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsonArrayElementList {
    type Item = SyntaxResult<JsonAnyValue>;
    type IntoIter = AstSeparatedListNodesIterator<Language, JsonAnyValue>;
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
impl AstNode for JsonMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_MEMBER_LIST as u16));
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
    fn into_syntax(self) -> SyntaxNode { self.syntax_list.into_node() }
}
#[cfg(feature = "serde")]
impl Serialize for JsonMemberList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for JsonMemberList {
    type Language = Language;
    type Node = JsonMember;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
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
