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
#[cfg(feature = "serde")]
impl Serialize for JsonArray {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonBooleanFields {
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
            key: self.key(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn key(&self) -> SyntaxResult<JsonString> { support::required_node(&self.syntax, 0usize) }
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
    pub key: SyntaxResult<JsonString>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<JsonAnyValue>,
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
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonNull {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonNullFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsonNumber {
    pub(crate) syntax: SyntaxNode,
}
impl JsonNumber {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonNumberFields {
        JsonNumberFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonNumberFields {
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
impl Serialize for JsonObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
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
pub struct JsonString {
    pub(crate) syntax: SyntaxNode,
}
impl JsonString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> JsonStringFields {
        JsonStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for JsonString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct JsonStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum JsonAnyValue {
    JsonArray(JsonArray),
    JsonBoolean(JsonBoolean),
    JsonNull(JsonNull),
    JsonNumber(JsonNumber),
    JsonObject(JsonObject),
    JsonString(JsonString),
    JsonUnknown(JsonUnknown),
}
impl JsonAnyValue {
    pub fn as_json_array(&self) -> Option<&JsonArray> {
        match &self {
            JsonAnyValue::JsonArray(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_boolean(&self) -> Option<&JsonBoolean> {
        match &self {
            JsonAnyValue::JsonBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_null(&self) -> Option<&JsonNull> {
        match &self {
            JsonAnyValue::JsonNull(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_number(&self) -> Option<&JsonNumber> {
        match &self {
            JsonAnyValue::JsonNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_object(&self) -> Option<&JsonObject> {
        match &self {
            JsonAnyValue::JsonObject(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_string(&self) -> Option<&JsonString> {
        match &self {
            JsonAnyValue::JsonString(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_json_unknown(&self) -> Option<&JsonUnknown> {
        match &self {
            JsonAnyValue::JsonUnknown(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for JsonArray {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_ARRAY as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_ARRAY }
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
impl AstNode for JsonBoolean {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_BOOLEAN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_BOOLEAN }
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
impl std::fmt::Debug for JsonBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonBoolean")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
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
            .field("key", &support::DebugSyntaxResult(self.key()))
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
impl AstNode for JsonNull {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_NULL as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_NULL }
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
impl std::fmt::Debug for JsonNull {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonNull")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonNull> for SyntaxNode {
    fn from(n: JsonNull) -> SyntaxNode { n.syntax }
}
impl From<JsonNull> for SyntaxElement {
    fn from(n: JsonNull) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonNumber {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_NUMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_NUMBER }
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
impl std::fmt::Debug for JsonNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonNumber")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonNumber> for SyntaxNode {
    fn from(n: JsonNumber) -> SyntaxNode { n.syntax }
}
impl From<JsonNumber> for SyntaxElement {
    fn from(n: JsonNumber) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsonObject {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_OBJECT as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_OBJECT }
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
impl AstNode for JsonString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_STRING }
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
impl std::fmt::Debug for JsonString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonString")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsonString> for SyntaxNode {
    fn from(n: JsonString) -> SyntaxNode { n.syntax }
}
impl From<JsonString> for SyntaxElement {
    fn from(n: JsonString) -> SyntaxElement { n.syntax.into() }
}
impl From<JsonArray> for JsonAnyValue {
    fn from(node: JsonArray) -> JsonAnyValue { JsonAnyValue::JsonArray(node) }
}
impl From<JsonBoolean> for JsonAnyValue {
    fn from(node: JsonBoolean) -> JsonAnyValue { JsonAnyValue::JsonBoolean(node) }
}
impl From<JsonNull> for JsonAnyValue {
    fn from(node: JsonNull) -> JsonAnyValue { JsonAnyValue::JsonNull(node) }
}
impl From<JsonNumber> for JsonAnyValue {
    fn from(node: JsonNumber) -> JsonAnyValue { JsonAnyValue::JsonNumber(node) }
}
impl From<JsonObject> for JsonAnyValue {
    fn from(node: JsonObject) -> JsonAnyValue { JsonAnyValue::JsonObject(node) }
}
impl From<JsonString> for JsonAnyValue {
    fn from(node: JsonString) -> JsonAnyValue { JsonAnyValue::JsonString(node) }
}
impl From<JsonUnknown> for JsonAnyValue {
    fn from(node: JsonUnknown) -> JsonAnyValue { JsonAnyValue::JsonUnknown(node) }
}
impl AstNode for JsonAnyValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = JsonArray::KIND_SET
        .union(JsonBoolean::KIND_SET)
        .union(JsonNull::KIND_SET)
        .union(JsonNumber::KIND_SET)
        .union(JsonObject::KIND_SET)
        .union(JsonString::KIND_SET)
        .union(JsonUnknown::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            JSON_ARRAY
                | JSON_BOOLEAN
                | JSON_NULL
                | JSON_NUMBER
                | JSON_OBJECT
                | JSON_STRING
                | JSON_UNKNOWN
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JSON_ARRAY => JsonAnyValue::JsonArray(JsonArray { syntax }),
            JSON_BOOLEAN => JsonAnyValue::JsonBoolean(JsonBoolean { syntax }),
            JSON_NULL => JsonAnyValue::JsonNull(JsonNull { syntax }),
            JSON_NUMBER => JsonAnyValue::JsonNumber(JsonNumber { syntax }),
            JSON_OBJECT => JsonAnyValue::JsonObject(JsonObject { syntax }),
            JSON_STRING => JsonAnyValue::JsonString(JsonString { syntax }),
            JSON_UNKNOWN => JsonAnyValue::JsonUnknown(JsonUnknown { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsonAnyValue::JsonArray(it) => &it.syntax,
            JsonAnyValue::JsonBoolean(it) => &it.syntax,
            JsonAnyValue::JsonNull(it) => &it.syntax,
            JsonAnyValue::JsonNumber(it) => &it.syntax,
            JsonAnyValue::JsonObject(it) => &it.syntax,
            JsonAnyValue::JsonString(it) => &it.syntax,
            JsonAnyValue::JsonUnknown(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            JsonAnyValue::JsonArray(it) => it.syntax,
            JsonAnyValue::JsonBoolean(it) => it.syntax,
            JsonAnyValue::JsonNull(it) => it.syntax,
            JsonAnyValue::JsonNumber(it) => it.syntax,
            JsonAnyValue::JsonObject(it) => it.syntax,
            JsonAnyValue::JsonString(it) => it.syntax,
            JsonAnyValue::JsonUnknown(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for JsonAnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonAnyValue::JsonArray(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonBoolean(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonNull(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonNumber(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonObject(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonString(it) => std::fmt::Debug::fmt(it, f),
            JsonAnyValue::JsonUnknown(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsonAnyValue> for SyntaxNode {
    fn from(n: JsonAnyValue) -> SyntaxNode {
        match n {
            JsonAnyValue::JsonArray(it) => it.into(),
            JsonAnyValue::JsonBoolean(it) => it.into(),
            JsonAnyValue::JsonNull(it) => it.into(),
            JsonAnyValue::JsonNumber(it) => it.into(),
            JsonAnyValue::JsonObject(it) => it.into(),
            JsonAnyValue::JsonString(it) => it.into(),
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
impl std::fmt::Display for JsonAnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
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
impl std::fmt::Display for JsonNumber {
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
impl std::fmt::Display for JsonString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
impl AstNode for JsonUnknown {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(JSON_UNKNOWN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == JSON_UNKNOWN }
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
