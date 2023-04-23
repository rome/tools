//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    MdLanguage as Language, MdSyntaxElement as SyntaxElement,
    MdSyntaxElementChildren as SyntaxElementChildren,
    MdSyntaxKind::{self as SyntaxKind, *},
    MdSyntaxList as SyntaxList, MdSyntaxNode as SyntaxNode, MdSyntaxToken as SyntaxToken,
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
pub struct MdHeading {
    pub(crate) syntax: SyntaxNode,
}
impl MdHeading {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> MdHeadingFields {
        MdHeadingFields {
            heading_level: self.heading_level(),
            value: self.value(),
        }
    }
    pub fn heading_level(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> Option<MdText> { support::node(&self.syntax, 1usize) }
}
#[cfg(feature = "serde")]
impl Serialize for MdHeading {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MdHeadingFields {
    pub heading_level: SyntaxResult<SyntaxToken>,
    pub value: Option<MdText>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdRoot {
    pub(crate) syntax: SyntaxNode,
}
impl MdRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> MdRootFields {
        MdRootFields {
            value: self.value(),
            eof_token: self.eof_token(),
        }
    }
    pub fn value(&self) -> MdElementList { support::list(&self.syntax, 0usize) }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for MdRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MdRootFields {
    pub value: MdElementList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MdText {
    pub(crate) syntax: SyntaxNode,
}
impl MdText {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> MdTextFields {
        MdTextFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for MdText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MdTextFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum AnyMdElement {
    MdHeading(MdHeading),
    MdText(MdText),
}
impl AnyMdElement {
    pub fn as_md_heading(&self) -> Option<&MdHeading> {
        match &self {
            AnyMdElement::MdHeading(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_md_text(&self) -> Option<&MdText> {
        match &self {
            AnyMdElement::MdText(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for MdHeading {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_HEADING as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == MD_HEADING }
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
impl std::fmt::Debug for MdHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MdHeading")
            .field(
                "heading_level",
                &support::DebugSyntaxResult(self.heading_level()),
            )
            .field("value", &support::DebugOptionalElement(self.value()))
            .finish()
    }
}
impl From<MdHeading> for SyntaxNode {
    fn from(n: MdHeading) -> SyntaxNode { n.syntax }
}
impl From<MdHeading> for SyntaxElement {
    fn from(n: MdHeading) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for MdRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == MD_ROOT }
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
impl std::fmt::Debug for MdRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MdRoot")
            .field("value", &self.value())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<MdRoot> for SyntaxNode {
    fn from(n: MdRoot) -> SyntaxNode { n.syntax }
}
impl From<MdRoot> for SyntaxElement {
    fn from(n: MdRoot) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for MdText {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_TEXT as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == MD_TEXT }
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
impl std::fmt::Debug for MdText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MdText")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<MdText> for SyntaxNode {
    fn from(n: MdText) -> SyntaxNode { n.syntax }
}
impl From<MdText> for SyntaxElement {
    fn from(n: MdText) -> SyntaxElement { n.syntax.into() }
}
impl From<MdHeading> for AnyMdElement {
    fn from(node: MdHeading) -> AnyMdElement { AnyMdElement::MdHeading(node) }
}
impl From<MdText> for AnyMdElement {
    fn from(node: MdText) -> AnyMdElement { AnyMdElement::MdText(node) }
}
impl AstNode for AnyMdElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MdHeading::KIND_SET.union(MdText::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, MD_HEADING | MD_TEXT) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            MD_HEADING => AnyMdElement::MdHeading(MdHeading { syntax }),
            MD_TEXT => AnyMdElement::MdText(MdText { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AnyMdElement::MdHeading(it) => &it.syntax,
            AnyMdElement::MdText(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            AnyMdElement::MdHeading(it) => it.syntax,
            AnyMdElement::MdText(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMdElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnyMdElement::MdHeading(it) => std::fmt::Debug::fmt(it, f),
            AnyMdElement::MdText(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMdElement> for SyntaxNode {
    fn from(n: AnyMdElement) -> SyntaxNode {
        match n {
            AnyMdElement::MdHeading(it) => it.into(),
            AnyMdElement::MdText(it) => it.into(),
        }
    }
}
impl From<AnyMdElement> for SyntaxElement {
    fn from(n: AnyMdElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyMdElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdHeading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MdText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct MdBogus {
    syntax: SyntaxNode,
}
impl MdBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for MdBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == MD_BOGUS }
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
impl std::fmt::Debug for MdBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MdBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MdBogus> for SyntaxNode {
    fn from(n: MdBogus) -> SyntaxNode { n.syntax }
}
impl From<MdBogus> for SyntaxElement {
    fn from(n: MdBogus) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MdElementList {
    syntax_list: SyntaxList,
}
impl MdElementList {
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
impl AstNode for MdElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(MD_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == MD_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<MdElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(MdElementList {
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
impl Serialize for MdElementList {
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
impl AstNodeList for MdElementList {
    type Language = Language;
    type Node = AnyMdElement;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for MdElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MdElementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MdElementList {
    type Item = AnyMdElement;
    type IntoIter = AstNodeListIterator<Language, AnyMdElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for MdElementList {
    type Item = AnyMdElement;
    type IntoIter = AstNodeListIterator<Language, AnyMdElement>;
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
