//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    macros::map_syntax_node,
    CssLanguage as Language, CssSyntaxElement as SyntaxElement,
    CssSyntaxElementChildren as SyntaxElementChildren,
    CssSyntaxKind::{self as SyntaxKind, *},
    CssSyntaxList as SyntaxList, CssSyntaxNode as SyntaxNode, CssSyntaxToken as SyntaxToken,
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
pub struct CssAnyFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssAnyFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAnyFunctionFields {
        CssAnyFunctionFields {
            css_simple_function: self.css_simple_function(),
        }
    }
    pub fn css_simple_function(&self) -> SyntaxResult<CssSimpleFunction> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAnyFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAnyFunctionFields {
    pub css_simple_function: SyntaxResult<CssSimpleFunction>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtKeyframes {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtKeyframes {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtKeyframesFields {
        CssAtKeyframesFields {
            at_token: self.at_token(),
            keyframes_token: self.keyframes_token(),
            name: self.name(),
            css_string: self.css_string(),
            body: self.body(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn keyframes_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn css_string(&self) -> SyntaxResult<CssString> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<CssAtKeyframesBody> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtKeyframes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtKeyframesFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub keyframes_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub css_string: SyntaxResult<CssString>,
    pub body: SyntaxResult<CssAtKeyframesBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtKeyframesBody {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtKeyframesBody {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtKeyframesBodyFields {
        CssAtKeyframesBodyFields {
            l_curly_token: self.l_curly_token(),
            items: self.items(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> CssAtKeyframesItemList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtKeyframesBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtKeyframesBodyFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub items: CssAtKeyframesItemList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMedia {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMedia {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaFields {
        CssAtMediaFields {
            at_token: self.at_token(),
            media_token: self.media_token(),
            query_list: self.query_list(),
            l_curly_token: self.l_curly_token(),
            body: self.body(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn media_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn query_list(&self) -> CssAtMediaQueryList { support::list(&self.syntax, 2usize) }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<CssAnyRule> { support::required_node(&self.syntax, 4usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMedia {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub media_token: SyntaxResult<SyntaxToken>,
    pub query_list: CssAtMediaQueryList,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<CssAnyRule>,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQuery {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQuery {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaQueryFields {
        CssAtMediaQueryFields {
            condition_token: self.condition_token(),
            or_token: self.or_token(),
            only_token: self.only_token(),
            ty: self.ty(),
            consequent: self.consequent(),
        }
    }
    pub fn condition_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn only_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
    pub fn ty(&self) -> SyntaxResult<CssAnyAtMediaQueryType> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn consequent(&self) -> Option<CssAtMediaQueryConsequent> {
        support::node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFields {
    pub condition_token: SyntaxResult<SyntaxToken>,
    pub or_token: SyntaxResult<SyntaxToken>,
    pub only_token: Option<SyntaxToken>,
    pub ty: SyntaxResult<CssAnyAtMediaQueryType>,
    pub consequent: Option<CssAtMediaQueryConsequent>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryConsequent {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryConsequent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaQueryConsequentFields {
        CssAtMediaQueryConsequentFields {
            and_token: self.and_token(),
            condition_token: self.condition_token(),
            ty: self.ty(),
        }
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn ty(&self) -> SyntaxResult<CssAnyAtMediaQueryType> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryConsequent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryConsequentFields {
    pub and_token: SyntaxResult<SyntaxToken>,
    pub condition_token: Option<SyntaxToken>,
    pub ty: SyntaxResult<CssAnyAtMediaQueryType>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeature {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeature {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaQueryFeatureFields {
        CssAtMediaQueryFeatureFields {
            l_paren_token: self.l_paren_token(),
            feature: self.feature(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn feature(&self) -> SyntaxResult<CssAnyAtMediaQueryFeatureType> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeatureFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub feature: SyntaxResult<CssAnyAtMediaQueryFeatureType>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeatureBoolean {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeatureBoolean {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaQueryFeatureBooleanFields {
        CssAtMediaQueryFeatureBooleanFields {
            css_identifier: self.css_identifier(),
        }
    }
    pub fn css_identifier(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeatureBoolean {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeatureBooleanFields {
    pub css_identifier: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeatureCompare {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeatureCompare {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaQueryFeatureCompareFields {
        CssAtMediaQueryFeatureCompareFields {
            name: self.name(),
            range: self.range(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn range(&self) -> SyntaxResult<CssAtMediaQueryRange> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<CssAnyValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeatureCompare {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeatureCompareFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub range: SyntaxResult<CssAtMediaQueryRange>,
    pub value: SyntaxResult<CssAnyValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeaturePlain {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeaturePlain {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaQueryFeaturePlainFields {
        CssAtMediaQueryFeaturePlainFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<CssAnyValue> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeaturePlain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeaturePlainFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssAnyValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryFeatureRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryFeatureRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaQueryFeatureRangeFields {
        CssAtMediaQueryFeatureRangeFields {
            first_value: self.first_value(),
            first_range: self.first_range(),
            name: self.name(),
            second_value: self.second_value(),
            second_range: self.second_range(),
        }
    }
    pub fn first_value(&self) -> SyntaxResult<CssAnyValue> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn first_range(&self) -> SyntaxResult<CssAtMediaQueryRange> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn second_value(&self) -> SyntaxResult<CssAnyValue> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn second_range(&self) -> SyntaxResult<CssAtMediaQueryRange> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryFeatureRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryFeatureRangeFields {
    pub first_value: SyntaxResult<CssAnyValue>,
    pub first_range: SyntaxResult<CssAtMediaQueryRange>,
    pub name: SyntaxResult<CssIdentifier>,
    pub second_value: SyntaxResult<CssAnyValue>,
    pub second_range: SyntaxResult<CssAtMediaQueryRange>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAtMediaQueryRange {
    pub(crate) syntax: SyntaxNode,
}
impl CssAtMediaQueryRange {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAtMediaQueryRangeFields {
        CssAtMediaQueryRangeFields {
            r_angle_token: self.r_angle_token(),
            l_angle_token: self.l_angle_token(),
            greater_than_equal_token: self.greater_than_equal_token(),
            less_than_equal_token: self.less_than_equal_token(),
        }
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn greater_than_equal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn less_than_equal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAtMediaQueryRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAtMediaQueryRangeFields {
    pub r_angle_token: SyntaxResult<SyntaxToken>,
    pub l_angle_token: SyntaxResult<SyntaxToken>,
    pub greater_than_equal_token: SyntaxResult<SyntaxToken>,
    pub less_than_equal_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttribute {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttribute {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAttributeFields {
        CssAttributeFields {
            l_brack_token: self.l_brack_token(),
            attribute_name: self.attribute_name(),
            attribute_meta: self.attribute_meta(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn attribute_name(&self) -> SyntaxResult<CssAttributeName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn attribute_meta(&self) -> Option<CssAttributeMeta> { support::node(&self.syntax, 2usize) }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub attribute_name: SyntaxResult<CssAttributeName>,
    pub attribute_meta: Option<CssAttributeMeta>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeMatcher {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeMatcher {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAttributeMatcherFields {
        CssAttributeMatcherFields {
            matcher_type_token: self.matcher_type_token(),
            exactly_or_hyphen_token: self.exactly_or_hyphen_token(),
            prefix_token: self.prefix_token(),
            suffix_token: self.suffix_token(),
            times_assign_token: self.times_assign_token(),
            eq_token: self.eq_token(),
            matcher_name: self.matcher_name(),
            css_identifier: self.css_identifier(),
        }
    }
    pub fn matcher_type_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn exactly_or_hyphen_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn prefix_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn suffix_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn times_assign_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn matcher_name(&self) -> SyntaxResult<CssString> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn css_identifier(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 7usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeMatcher {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeMatcherFields {
    pub matcher_type_token: SyntaxResult<SyntaxToken>,
    pub exactly_or_hyphen_token: SyntaxResult<SyntaxToken>,
    pub prefix_token: SyntaxResult<SyntaxToken>,
    pub suffix_token: SyntaxResult<SyntaxToken>,
    pub times_assign_token: SyntaxResult<SyntaxToken>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub matcher_name: SyntaxResult<CssString>,
    pub css_identifier: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeMeta {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeMeta {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAttributeMetaFields {
        CssAttributeMetaFields {
            attribute_matcher: self.attribute_matcher(),
            attribute_modifier: self.attribute_modifier(),
        }
    }
    pub fn attribute_matcher(&self) -> Option<CssAttributeMatcher> {
        support::node(&self.syntax, 0usize)
    }
    pub fn attribute_modifier(&self) -> Option<CssAttributeModifier> {
        support::node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeMetaFields {
    pub attribute_matcher: Option<CssAttributeMatcher>,
    pub attribute_modifier: Option<CssAttributeModifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeModifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAttributeModifierFields {
        CssAttributeModifierFields {
            i_token: self.i_token(),
        }
    }
    pub fn i_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeModifierFields {
    pub i_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeName {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAttributeNameFields {
        CssAttributeNameFields {
            css_string: self.css_string(),
        }
    }
    pub fn css_string(&self) -> SyntaxResult<CssString> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeNameFields {
    pub css_string: SyntaxResult<CssString>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssAttributeSelectorPattern {
    pub(crate) syntax: SyntaxNode,
}
impl CssAttributeSelectorPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssAttributeSelectorPatternFields {
        CssAttributeSelectorPatternFields {
            name: self.name(),
            attribute_list: self.attribute_list(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn attribute_list(&self) -> CssAttributeList { support::list(&self.syntax, 1usize) }
}
#[cfg(feature = "serde")]
impl Serialize for CssAttributeSelectorPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssAttributeSelectorPatternFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub attribute_list: CssAttributeList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssBlockFields {
        CssBlockFields {
            l_curly_token: self.l_curly_token(),
            declaration_list: self.declaration_list(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declaration_list(&self) -> CssDeclarationList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBlockFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub declaration_list: CssDeclarationList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssClassSelectorPattern {
    pub(crate) syntax: SyntaxNode,
}
impl CssClassSelectorPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssClassSelectorPatternFields {
        CssClassSelectorPatternFields {
            dot_token: self.dot_token(),
            name: self.name(),
        }
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssClassSelectorPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssClassSelectorPatternFields {
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCombinatorSelectorPattern {
    pub(crate) syntax: SyntaxNode,
}
impl CssCombinatorSelectorPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssCombinatorSelectorPatternFields {
        CssCombinatorSelectorPatternFields {
            left: self.left(),
            combinator_token: self.combinator_token(),
            plus_token: self.plus_token(),
            bitwise_not_token: self.bitwise_not_token(),
            css_space_literal_token: self.css_space_literal_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<CssAnySelectorPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn combinator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn plus_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn bitwise_not_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn css_space_literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn right(&self) -> SyntaxResult<CssAnySelectorPattern> {
        support::required_node(&self.syntax, 5usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCombinatorSelectorPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCombinatorSelectorPatternFields {
    pub left: SyntaxResult<CssAnySelectorPattern>,
    pub combinator_token: SyntaxResult<SyntaxToken>,
    pub plus_token: SyntaxResult<SyntaxToken>,
    pub bitwise_not_token: SyntaxResult<SyntaxToken>,
    pub css_space_literal_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<CssAnySelectorPattern>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssCustomProperty {
    pub(crate) syntax: SyntaxNode,
}
impl CssCustomProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssCustomPropertyFields {
        CssCustomPropertyFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssCustomProperty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssCustomPropertyFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssDeclarationFields {
        CssDeclarationFields {
            name: self.name(),
            css_custom_property: self.css_custom_property(),
            colon_token: self.colon_token(),
            value: self.value(),
            important: self.important(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn css_custom_property(&self) -> SyntaxResult<CssCustomProperty> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn value(&self) -> SyntaxResult<CssAnyValue> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn important(&self) -> Option<CssDeclarationImportant> {
        support::node(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDeclarationFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub css_custom_property: SyntaxResult<CssCustomProperty>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssAnyValue>,
    pub important: Option<CssDeclarationImportant>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDeclarationImportant {
    pub(crate) syntax: SyntaxNode,
}
impl CssDeclarationImportant {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssDeclarationImportantFields {
        CssDeclarationImportantFields {
            excl_token: self.excl_token(),
            important_token: self.important_token(),
        }
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn important_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDeclarationImportant {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDeclarationImportantFields {
    pub excl_token: SyntaxResult<SyntaxToken>,
    pub important_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssDimension {
    pub(crate) syntax: SyntaxNode,
}
impl CssDimension {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssDimensionFields {
        CssDimensionFields {
            value: self.value(),
            unit: self.unit(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> { support::required_node(&self.syntax, 0usize) }
    pub fn unit(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssDimension {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssDimensionFields {
    pub value: SyntaxResult<CssNumber>,
    pub unit: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssIdSelectorPattern {
    pub(crate) syntax: SyntaxNode,
}
impl CssIdSelectorPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssIdSelectorPatternFields {
        CssIdSelectorPatternFields {
            hash_token: self.hash_token(),
            name: self.name(),
        }
    }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssIdSelectorPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssIdSelectorPatternFields {
    pub hash_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl CssIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssIdentifierFields {
        CssIdentifierFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssIdentifierFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesBlock {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssKeyframesBlockFields {
        CssKeyframesBlockFields {
            selectors: self.selectors(),
            l_curly_token: self.l_curly_token(),
            declarations: self.declarations(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn selectors(&self) -> CssKeyframesSelectorList { support::list(&self.syntax, 0usize) }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn declarations(&self) -> CssDeclarationList { support::list(&self.syntax, 2usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesBlock {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesBlockFields {
    pub selectors: CssKeyframesSelectorList,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub declarations: CssDeclarationList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssKeyframesSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssKeyframesSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssKeyframesSelectorFields {
        CssKeyframesSelectorFields {
            from_token: self.from_token(),
            to_token: self.to_token(),
            css_percentage: self.css_percentage(),
        }
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn to_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn css_percentage(&self) -> SyntaxResult<CssPercentage> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssKeyframesSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssKeyframesSelectorFields {
    pub from_token: SyntaxResult<SyntaxToken>,
    pub to_token: SyntaxResult<SyntaxToken>,
    pub css_percentage: SyntaxResult<CssPercentage>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssNumber {
    pub(crate) syntax: SyntaxNode,
}
impl CssNumber {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssNumberFields {
        CssNumberFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssNumberFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssParameter {
    pub(crate) syntax: SyntaxNode,
}
impl CssParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssParameterFields {
        CssParameterFields {
            css_any_value: self.css_any_value(),
        }
    }
    pub fn css_any_value(&self) -> SyntaxResult<CssAnyValue> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssParameterFields {
    pub css_any_value: SyntaxResult<CssAnyValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPercentage {
    pub(crate) syntax: SyntaxNode,
}
impl CssPercentage {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssPercentageFields {
        CssPercentageFields {
            value: self.value(),
            reminder_token: self.reminder_token(),
        }
    }
    pub fn value(&self) -> SyntaxResult<CssNumber> { support::required_node(&self.syntax, 0usize) }
    pub fn reminder_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPercentage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPercentageFields {
    pub value: SyntaxResult<CssNumber>,
    pub reminder_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassSelectorPattern {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassSelectorPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssPseudoClassSelectorPatternFields {
        CssPseudoClassSelectorPatternFields {
            colon_token: self.colon_token(),
            name: self.name(),
            parameters: self.parameters(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn parameters(&self) -> Option<CssPseudoClassSelectorPatternParameters> {
        support::node(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassSelectorPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassSelectorPatternFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<CssIdentifier>,
    pub parameters: Option<CssPseudoClassSelectorPatternParameters>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssPseudoClassSelectorPatternParameters {
    pub(crate) syntax: SyntaxNode,
}
impl CssPseudoClassSelectorPatternParameters {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssPseudoClassSelectorPatternParametersFields {
        CssPseudoClassSelectorPatternParametersFields {
            l_paren_token: self.l_paren_token(),
            parameter: self.parameter(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn parameter(&self) -> SyntaxResult<CssAnyValue> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssPseudoClassSelectorPatternParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssPseudoClassSelectorPatternParametersFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub parameter: SyntaxResult<CssAnyValue>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRatio {
    pub(crate) syntax: SyntaxNode,
}
impl CssRatio {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssRatioFields {
        CssRatioFields {
            numerator: self.numerator(),
            denominator: self.denominator(),
        }
    }
    pub fn numerator(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn denominator(&self) -> SyntaxResult<CssNumber> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssRatio {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRatioFields {
    pub numerator: SyntaxResult<CssNumber>,
    pub denominator: SyntaxResult<CssNumber>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssRule {
    pub(crate) syntax: SyntaxNode,
}
impl CssRule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssRuleFields {
        CssRuleFields {
            prelude: self.prelude(),
            block: self.block(),
        }
    }
    pub fn prelude(&self) -> CssSelectorList { support::list(&self.syntax, 0usize) }
    pub fn block(&self) -> SyntaxResult<CssBlock> { support::required_node(&self.syntax, 1usize) }
}
#[cfg(feature = "serde")]
impl Serialize for CssRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssRuleFields {
    pub prelude: CssSelectorList,
    pub block: SyntaxResult<CssBlock>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSelector {
    pub(crate) syntax: SyntaxNode,
}
impl CssSelector {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssSelectorFields {
        CssSelectorFields {
            pattern_list: self.pattern_list(),
        }
    }
    pub fn pattern_list(&self) -> CssAnySelectorPatternList { support::list(&self.syntax, 0usize) }
}
#[cfg(feature = "serde")]
impl Serialize for CssSelector {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSelectorFields {
    pub pattern_list: CssAnySelectorPatternList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssSimpleFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssSimpleFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssSimpleFunctionFields {
        CssSimpleFunctionFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn items(&self) -> CssParameterList { support::list(&self.syntax, 2usize) }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssSimpleFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssSimpleFunctionFields {
    pub name: SyntaxResult<CssIdentifier>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: CssParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssString {
    pub(crate) syntax: SyntaxNode,
}
impl CssString {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssStringFields {
        CssStringFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssStringFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssTypeSelectorPattern {
    pub(crate) syntax: SyntaxNode,
}
impl CssTypeSelectorPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssTypeSelectorPatternFields {
        CssTypeSelectorPatternFields {
            ident: self.ident(),
        }
    }
    pub fn ident(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssTypeSelectorPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssTypeSelectorPatternFields {
    pub ident: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssUniversalSelectorPattern {
    pub(crate) syntax: SyntaxNode,
}
impl CssUniversalSelectorPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssUniversalSelectorPatternFields {
        CssUniversalSelectorPatternFields {
            star_token: self.star_token(),
        }
    }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssUniversalSelectorPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssUniversalSelectorPatternFields {
    pub star_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssVarFunction {
    pub(crate) syntax: SyntaxNode,
}
impl CssVarFunction {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssVarFunctionFields {
        CssVarFunctionFields {
            var_token: self.var_token(),
            l_paren_token: self.l_paren_token(),
            property: self.property(),
            value: self.value(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn var_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn property(&self) -> SyntaxResult<CssCustomProperty> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn value(&self) -> Option<CssVarFunctionValue> { support::node(&self.syntax, 3usize) }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssVarFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssVarFunctionFields {
    pub var_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub property: SyntaxResult<CssCustomProperty>,
    pub value: Option<CssVarFunctionValue>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CssVarFunctionValue {
    pub(crate) syntax: SyntaxNode,
}
impl CssVarFunctionValue {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_fields(&self) -> CssVarFunctionValueFields {
        CssVarFunctionValueFields {
            comma_token: self.comma_token(),
            value: self.value(),
        }
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<CssIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[cfg(feature = "serde")]
impl Serialize for CssVarFunctionValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssVarFunctionValueFields {
    pub comma_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<CssIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssAnyAtMediaQueryFeatureType {
    CssAtMediaQueryFeatureBoolean(CssAtMediaQueryFeatureBoolean),
    CssAtMediaQueryFeatureCompare(CssAtMediaQueryFeatureCompare),
    CssAtMediaQueryFeaturePlain(CssAtMediaQueryFeaturePlain),
    CssAtMediaQueryFeatureRange(CssAtMediaQueryFeatureRange),
}
impl CssAnyAtMediaQueryFeatureType {
    pub fn as_css_at_media_query_feature_boolean(&self) -> Option<&CssAtMediaQueryFeatureBoolean> {
        match &self {
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_at_media_query_feature_compare(&self) -> Option<&CssAtMediaQueryFeatureCompare> {
        match &self {
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_at_media_query_feature_plain(&self) -> Option<&CssAtMediaQueryFeaturePlain> {
        match &self {
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_at_media_query_feature_range(&self) -> Option<&CssAtMediaQueryFeatureRange> {
        match &self {
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssAnyAtMediaQueryType {
    CssAtMediaQueryFeature(CssAtMediaQueryFeature),
    CssIdentifier(CssIdentifier),
}
impl CssAnyAtMediaQueryType {
    pub fn as_css_at_media_query_feature(&self) -> Option<&CssAtMediaQueryFeature> {
        match &self {
            CssAnyAtMediaQueryType::CssAtMediaQueryFeature(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            CssAnyAtMediaQueryType::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssAnyAtRule {
    CssAtKeyframes(CssAtKeyframes),
    CssAtMedia(CssAtMedia),
}
impl CssAnyAtRule {
    pub fn as_css_at_keyframes(&self) -> Option<&CssAtKeyframes> {
        match &self {
            CssAnyAtRule::CssAtKeyframes(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_at_media(&self) -> Option<&CssAtMedia> {
        match &self {
            CssAnyAtRule::CssAtMedia(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssAnyRule {
    CssAnyAtRule(CssAnyAtRule),
    CssRule(CssRule),
}
impl CssAnyRule {
    pub fn as_css_any_at_rule(&self) -> Option<&CssAnyAtRule> {
        match &self {
            CssAnyRule::CssAnyAtRule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_rule(&self) -> Option<&CssRule> {
        match &self {
            CssAnyRule::CssRule(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssAnySelectorPattern {
    CssAttributeSelectorPattern(CssAttributeSelectorPattern),
    CssClassSelectorPattern(CssClassSelectorPattern),
    CssCombinatorSelectorPattern(CssCombinatorSelectorPattern),
    CssIdSelectorPattern(CssIdSelectorPattern),
    CssPseudoClassSelectorPattern(CssPseudoClassSelectorPattern),
    CssTypeSelectorPattern(CssTypeSelectorPattern),
    CssUniversalSelectorPattern(CssUniversalSelectorPattern),
}
impl CssAnySelectorPattern {
    pub fn as_css_attribute_selector_pattern(&self) -> Option<&CssAttributeSelectorPattern> {
        match &self {
            CssAnySelectorPattern::CssAttributeSelectorPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_class_selector_pattern(&self) -> Option<&CssClassSelectorPattern> {
        match &self {
            CssAnySelectorPattern::CssClassSelectorPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_combinator_selector_pattern(&self) -> Option<&CssCombinatorSelectorPattern> {
        match &self {
            CssAnySelectorPattern::CssCombinatorSelectorPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_id_selector_pattern(&self) -> Option<&CssIdSelectorPattern> {
        match &self {
            CssAnySelectorPattern::CssIdSelectorPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_pseudo_class_selector_pattern(&self) -> Option<&CssPseudoClassSelectorPattern> {
        match &self {
            CssAnySelectorPattern::CssPseudoClassSelectorPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_type_selector_pattern(&self) -> Option<&CssTypeSelectorPattern> {
        match &self {
            CssAnySelectorPattern::CssTypeSelectorPattern(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_universal_selector_pattern(&self) -> Option<&CssUniversalSelectorPattern> {
        match &self {
            CssAnySelectorPattern::CssUniversalSelectorPattern(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum CssAnyValue {
    CssAnyFunction(CssAnyFunction),
    CssCustomProperty(CssCustomProperty),
    CssDimension(CssDimension),
    CssIdentifier(CssIdentifier),
    CssNumber(CssNumber),
    CssRatio(CssRatio),
    CssString(CssString),
}
impl CssAnyValue {
    pub fn as_css_any_function(&self) -> Option<&CssAnyFunction> {
        match &self {
            CssAnyValue::CssAnyFunction(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_custom_property(&self) -> Option<&CssCustomProperty> {
        match &self {
            CssAnyValue::CssCustomProperty(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_dimension(&self) -> Option<&CssDimension> {
        match &self {
            CssAnyValue::CssDimension(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_identifier(&self) -> Option<&CssIdentifier> {
        match &self {
            CssAnyValue::CssIdentifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_number(&self) -> Option<&CssNumber> {
        match &self {
            CssAnyValue::CssNumber(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_ratio(&self) -> Option<&CssRatio> {
        match &self {
            CssAnyValue::CssRatio(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_css_string(&self) -> Option<&CssString> {
        match &self {
            CssAnyValue::CssString(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for CssAnyFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ANY_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ANY_FUNCTION }
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
impl std::fmt::Debug for CssAnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAnyFunction")
            .field(
                "css_simple_function",
                &support::DebugSyntaxResult(self.css_simple_function()),
            )
            .finish()
    }
}
impl From<CssAnyFunction> for SyntaxNode {
    fn from(n: CssAnyFunction) -> SyntaxNode { n.syntax }
}
impl From<CssAnyFunction> for SyntaxElement {
    fn from(n: CssAnyFunction) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtKeyframes {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_KEYFRAMES as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_KEYFRAMES }
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
impl std::fmt::Debug for CssAtKeyframes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtKeyframes")
            .field("at_token", &support::DebugSyntaxResult(self.at_token()))
            .field(
                "keyframes_token",
                &support::DebugSyntaxResult(self.keyframes_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("css_string", &support::DebugSyntaxResult(self.css_string()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<CssAtKeyframes> for SyntaxNode {
    fn from(n: CssAtKeyframes) -> SyntaxNode { n.syntax }
}
impl From<CssAtKeyframes> for SyntaxElement {
    fn from(n: CssAtKeyframes) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtKeyframesBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_KEYFRAMES_BODY as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_KEYFRAMES_BODY }
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
impl std::fmt::Debug for CssAtKeyframesBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtKeyframesBody")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("items", &self.items())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssAtKeyframesBody> for SyntaxNode {
    fn from(n: CssAtKeyframesBody) -> SyntaxNode { n.syntax }
}
impl From<CssAtKeyframesBody> for SyntaxElement {
    fn from(n: CssAtKeyframesBody) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMedia {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA }
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
impl std::fmt::Debug for CssAtMedia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMedia")
            .field("at_token", &support::DebugSyntaxResult(self.at_token()))
            .field(
                "media_token",
                &support::DebugSyntaxResult(self.media_token()),
            )
            .field("query_list", &self.query_list())
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssAtMedia> for SyntaxNode {
    fn from(n: CssAtMedia) -> SyntaxNode { n.syntax }
}
impl From<CssAtMedia> for SyntaxElement {
    fn from(n: CssAtMedia) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMediaQuery {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY }
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
impl std::fmt::Debug for CssAtMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQuery")
            .field(
                "condition_token",
                &support::DebugSyntaxResult(self.condition_token()),
            )
            .field("or_token", &support::DebugSyntaxResult(self.or_token()))
            .field(
                "only_token",
                &support::DebugOptionalElement(self.only_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .field(
                "consequent",
                &support::DebugOptionalElement(self.consequent()),
            )
            .finish()
    }
}
impl From<CssAtMediaQuery> for SyntaxNode {
    fn from(n: CssAtMediaQuery) -> SyntaxNode { n.syntax }
}
impl From<CssAtMediaQuery> for SyntaxElement {
    fn from(n: CssAtMediaQuery) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMediaQueryConsequent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_CONSEQUENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY_CONSEQUENT }
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
impl std::fmt::Debug for CssAtMediaQueryConsequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryConsequent")
            .field("and_token", &support::DebugSyntaxResult(self.and_token()))
            .field(
                "condition_token",
                &support::DebugOptionalElement(self.condition_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<CssAtMediaQueryConsequent> for SyntaxNode {
    fn from(n: CssAtMediaQueryConsequent) -> SyntaxNode { n.syntax }
}
impl From<CssAtMediaQueryConsequent> for SyntaxElement {
    fn from(n: CssAtMediaQueryConsequent) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMediaQueryFeature {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY_FEATURE }
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
impl std::fmt::Debug for CssAtMediaQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeature")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("feature", &support::DebugSyntaxResult(self.feature()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssAtMediaQueryFeature> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeature) -> SyntaxNode { n.syntax }
}
impl From<CssAtMediaQueryFeature> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeature) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMediaQueryFeatureBoolean {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN }
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
impl std::fmt::Debug for CssAtMediaQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeatureBoolean")
            .field(
                "css_identifier",
                &support::DebugSyntaxResult(self.css_identifier()),
            )
            .finish()
    }
}
impl From<CssAtMediaQueryFeatureBoolean> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeatureBoolean) -> SyntaxNode { n.syntax }
}
impl From<CssAtMediaQueryFeatureBoolean> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeatureBoolean) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMediaQueryFeatureCompare {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE_COMPARE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY_FEATURE_COMPARE }
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
impl std::fmt::Debug for CssAtMediaQueryFeatureCompare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeatureCompare")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("range", &support::DebugSyntaxResult(self.range()))
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssAtMediaQueryFeatureCompare> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeatureCompare) -> SyntaxNode { n.syntax }
}
impl From<CssAtMediaQueryFeatureCompare> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeatureCompare) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMediaQueryFeaturePlain {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE_PLAIN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY_FEATURE_PLAIN }
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
impl std::fmt::Debug for CssAtMediaQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeaturePlain")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssAtMediaQueryFeaturePlain> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeaturePlain) -> SyntaxNode { n.syntax }
}
impl From<CssAtMediaQueryFeaturePlain> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeaturePlain) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMediaQueryFeatureRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_FEATURE_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY_FEATURE_RANGE }
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
impl std::fmt::Debug for CssAtMediaQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryFeatureRange")
            .field(
                "first_value",
                &support::DebugSyntaxResult(self.first_value()),
            )
            .field(
                "first_range",
                &support::DebugSyntaxResult(self.first_range()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "second_value",
                &support::DebugSyntaxResult(self.second_value()),
            )
            .field(
                "second_range",
                &support::DebugSyntaxResult(self.second_range()),
            )
            .finish()
    }
}
impl From<CssAtMediaQueryFeatureRange> for SyntaxNode {
    fn from(n: CssAtMediaQueryFeatureRange) -> SyntaxNode { n.syntax }
}
impl From<CssAtMediaQueryFeatureRange> for SyntaxElement {
    fn from(n: CssAtMediaQueryFeatureRange) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAtMediaQueryRange {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_RANGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY_RANGE }
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
impl std::fmt::Debug for CssAtMediaQueryRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAtMediaQueryRange")
            .field(
                "r_angle_token",
                &support::DebugSyntaxResult(self.r_angle_token()),
            )
            .field(
                "l_angle_token",
                &support::DebugSyntaxResult(self.l_angle_token()),
            )
            .field(
                "greater_than_equal_token",
                &support::DebugSyntaxResult(self.greater_than_equal_token()),
            )
            .field(
                "less_than_equal_token",
                &support::DebugSyntaxResult(self.less_than_equal_token()),
            )
            .finish()
    }
}
impl From<CssAtMediaQueryRange> for SyntaxNode {
    fn from(n: CssAtMediaQueryRange) -> SyntaxNode { n.syntax }
}
impl From<CssAtMediaQueryRange> for SyntaxElement {
    fn from(n: CssAtMediaQueryRange) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ATTRIBUTE }
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
impl std::fmt::Debug for CssAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttribute")
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field(
                "attribute_name",
                &support::DebugSyntaxResult(self.attribute_name()),
            )
            .field(
                "attribute_meta",
                &support::DebugOptionalElement(self.attribute_meta()),
            )
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<CssAttribute> for SyntaxNode {
    fn from(n: CssAttribute) -> SyntaxNode { n.syntax }
}
impl From<CssAttribute> for SyntaxElement {
    fn from(n: CssAttribute) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAttributeMatcher {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_MATCHER as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ATTRIBUTE_MATCHER }
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
impl std::fmt::Debug for CssAttributeMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeMatcher")
            .field(
                "matcher_type_token",
                &support::DebugSyntaxResult(self.matcher_type_token()),
            )
            .field(
                "exactly_or_hyphen_token",
                &support::DebugSyntaxResult(self.exactly_or_hyphen_token()),
            )
            .field(
                "prefix_token",
                &support::DebugSyntaxResult(self.prefix_token()),
            )
            .field(
                "suffix_token",
                &support::DebugSyntaxResult(self.suffix_token()),
            )
            .field(
                "times_assign_token",
                &support::DebugSyntaxResult(self.times_assign_token()),
            )
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field(
                "matcher_name",
                &support::DebugSyntaxResult(self.matcher_name()),
            )
            .field(
                "css_identifier",
                &support::DebugSyntaxResult(self.css_identifier()),
            )
            .finish()
    }
}
impl From<CssAttributeMatcher> for SyntaxNode {
    fn from(n: CssAttributeMatcher) -> SyntaxNode { n.syntax }
}
impl From<CssAttributeMatcher> for SyntaxElement {
    fn from(n: CssAttributeMatcher) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAttributeMeta {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_META as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ATTRIBUTE_META }
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
impl std::fmt::Debug for CssAttributeMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeMeta")
            .field(
                "attribute_matcher",
                &support::DebugOptionalElement(self.attribute_matcher()),
            )
            .field(
                "attribute_modifier",
                &support::DebugOptionalElement(self.attribute_modifier()),
            )
            .finish()
    }
}
impl From<CssAttributeMeta> for SyntaxNode {
    fn from(n: CssAttributeMeta) -> SyntaxNode { n.syntax }
}
impl From<CssAttributeMeta> for SyntaxElement {
    fn from(n: CssAttributeMeta) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAttributeModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ATTRIBUTE_MODIFIER }
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
impl std::fmt::Debug for CssAttributeModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeModifier")
            .field("i_token", &support::DebugSyntaxResult(self.i_token()))
            .finish()
    }
}
impl From<CssAttributeModifier> for SyntaxNode {
    fn from(n: CssAttributeModifier) -> SyntaxNode { n.syntax }
}
impl From<CssAttributeModifier> for SyntaxElement {
    fn from(n: CssAttributeModifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAttributeName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ATTRIBUTE_NAME }
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
impl std::fmt::Debug for CssAttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeName")
            .field("css_string", &support::DebugSyntaxResult(self.css_string()))
            .finish()
    }
}
impl From<CssAttributeName> for SyntaxNode {
    fn from(n: CssAttributeName) -> SyntaxNode { n.syntax }
}
impl From<CssAttributeName> for SyntaxElement {
    fn from(n: CssAttributeName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssAttributeSelectorPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_SELECTOR_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ATTRIBUTE_SELECTOR_PATTERN }
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
impl std::fmt::Debug for CssAttributeSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssAttributeSelectorPattern")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("attribute_list", &self.attribute_list())
            .finish()
    }
}
impl From<CssAttributeSelectorPattern> for SyntaxNode {
    fn from(n: CssAttributeSelectorPattern) -> SyntaxNode { n.syntax }
}
impl From<CssAttributeSelectorPattern> for SyntaxElement {
    fn from(n: CssAttributeSelectorPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_BLOCK }
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
impl std::fmt::Debug for CssBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBlock")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("declaration_list", &self.declaration_list())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssBlock> for SyntaxNode {
    fn from(n: CssBlock) -> SyntaxNode { n.syntax }
}
impl From<CssBlock> for SyntaxElement {
    fn from(n: CssBlock) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssClassSelectorPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CLASS_SELECTOR_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_CLASS_SELECTOR_PATTERN }
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
impl std::fmt::Debug for CssClassSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssClassSelectorPattern")
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssClassSelectorPattern> for SyntaxNode {
    fn from(n: CssClassSelectorPattern) -> SyntaxNode { n.syntax }
}
impl From<CssClassSelectorPattern> for SyntaxElement {
    fn from(n: CssClassSelectorPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssCombinatorSelectorPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_COMBINATOR_SELECTOR_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_COMBINATOR_SELECTOR_PATTERN }
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
impl std::fmt::Debug for CssCombinatorSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCombinatorSelectorPattern")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "combinator_token",
                &support::DebugSyntaxResult(self.combinator_token()),
            )
            .field("plus_token", &support::DebugSyntaxResult(self.plus_token()))
            .field(
                "bitwise_not_token",
                &support::DebugSyntaxResult(self.bitwise_not_token()),
            )
            .field(
                "css_space_literal_token",
                &support::DebugSyntaxResult(self.css_space_literal_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<CssCombinatorSelectorPattern> for SyntaxNode {
    fn from(n: CssCombinatorSelectorPattern) -> SyntaxNode { n.syntax }
}
impl From<CssCombinatorSelectorPattern> for SyntaxElement {
    fn from(n: CssCombinatorSelectorPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssCustomProperty {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_CUSTOM_PROPERTY as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_CUSTOM_PROPERTY }
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
impl std::fmt::Debug for CssCustomProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssCustomProperty")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssCustomProperty> for SyntaxNode {
    fn from(n: CssCustomProperty) -> SyntaxNode { n.syntax }
}
impl From<CssCustomProperty> for SyntaxElement {
    fn from(n: CssCustomProperty) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_DECLARATION }
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
impl std::fmt::Debug for CssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDeclaration")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "css_custom_property",
                &support::DebugSyntaxResult(self.css_custom_property()),
            )
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field(
                "important",
                &support::DebugOptionalElement(self.important()),
            )
            .finish()
    }
}
impl From<CssDeclaration> for SyntaxNode {
    fn from(n: CssDeclaration) -> SyntaxNode { n.syntax }
}
impl From<CssDeclaration> for SyntaxElement {
    fn from(n: CssDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssDeclarationImportant {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_IMPORTANT as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_DECLARATION_IMPORTANT }
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
impl std::fmt::Debug for CssDeclarationImportant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDeclarationImportant")
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .field(
                "important_token",
                &support::DebugSyntaxResult(self.important_token()),
            )
            .finish()
    }
}
impl From<CssDeclarationImportant> for SyntaxNode {
    fn from(n: CssDeclarationImportant) -> SyntaxNode { n.syntax }
}
impl From<CssDeclarationImportant> for SyntaxElement {
    fn from(n: CssDeclarationImportant) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssDimension {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DIMENSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_DIMENSION }
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
impl std::fmt::Debug for CssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDimension")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field("unit", &support::DebugSyntaxResult(self.unit()))
            .finish()
    }
}
impl From<CssDimension> for SyntaxNode {
    fn from(n: CssDimension) -> SyntaxNode { n.syntax }
}
impl From<CssDimension> for SyntaxElement {
    fn from(n: CssDimension) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssIdSelectorPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ID_SELECTOR_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ID_SELECTOR_PATTERN }
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
impl std::fmt::Debug for CssIdSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssIdSelectorPattern")
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<CssIdSelectorPattern> for SyntaxNode {
    fn from(n: CssIdSelectorPattern) -> SyntaxNode { n.syntax }
}
impl From<CssIdSelectorPattern> for SyntaxElement {
    fn from(n: CssIdSelectorPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_IDENTIFIER }
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
impl std::fmt::Debug for CssIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssIdentifier")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssIdentifier> for SyntaxNode {
    fn from(n: CssIdentifier) -> SyntaxNode { n.syntax }
}
impl From<CssIdentifier> for SyntaxElement {
    fn from(n: CssIdentifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssKeyframesBlock {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_BLOCK as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_KEYFRAMES_BLOCK }
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
impl std::fmt::Debug for CssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesBlock")
            .field("selectors", &self.selectors())
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("declarations", &self.declarations())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<CssKeyframesBlock> for SyntaxNode {
    fn from(n: CssKeyframesBlock) -> SyntaxNode { n.syntax }
}
impl From<CssKeyframesBlock> for SyntaxElement {
    fn from(n: CssKeyframesBlock) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssKeyframesSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_KEYFRAMES_SELECTOR }
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
impl std::fmt::Debug for CssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssKeyframesSelector")
            .field("from_token", &support::DebugSyntaxResult(self.from_token()))
            .field("to_token", &support::DebugSyntaxResult(self.to_token()))
            .field(
                "css_percentage",
                &support::DebugSyntaxResult(self.css_percentage()),
            )
            .finish()
    }
}
impl From<CssKeyframesSelector> for SyntaxNode {
    fn from(n: CssKeyframesSelector) -> SyntaxNode { n.syntax }
}
impl From<CssKeyframesSelector> for SyntaxElement {
    fn from(n: CssKeyframesSelector) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssNumber {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_NUMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_NUMBER }
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
impl std::fmt::Debug for CssNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssNumber")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssNumber> for SyntaxNode {
    fn from(n: CssNumber) -> SyntaxNode { n.syntax }
}
impl From<CssNumber> for SyntaxElement {
    fn from(n: CssNumber) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_PARAMETER }
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
impl std::fmt::Debug for CssParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssParameter")
            .field(
                "css_any_value",
                &support::DebugSyntaxResult(self.css_any_value()),
            )
            .finish()
    }
}
impl From<CssParameter> for SyntaxNode {
    fn from(n: CssParameter) -> SyntaxNode { n.syntax }
}
impl From<CssParameter> for SyntaxElement {
    fn from(n: CssParameter) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssPercentage {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PERCENTAGE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_PERCENTAGE }
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
impl std::fmt::Debug for CssPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPercentage")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .field(
                "reminder_token",
                &support::DebugSyntaxResult(self.reminder_token()),
            )
            .finish()
    }
}
impl From<CssPercentage> for SyntaxNode {
    fn from(n: CssPercentage) -> SyntaxNode { n.syntax }
}
impl From<CssPercentage> for SyntaxElement {
    fn from(n: CssPercentage) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssPseudoClassSelectorPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PSEUDO_CLASS_SELECTOR_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_PSEUDO_CLASS_SELECTOR_PATTERN }
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
impl std::fmt::Debug for CssPseudoClassSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassSelectorPattern")
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "parameters",
                &support::DebugOptionalElement(self.parameters()),
            )
            .finish()
    }
}
impl From<CssPseudoClassSelectorPattern> for SyntaxNode {
    fn from(n: CssPseudoClassSelectorPattern) -> SyntaxNode { n.syntax }
}
impl From<CssPseudoClassSelectorPattern> for SyntaxElement {
    fn from(n: CssPseudoClassSelectorPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssPseudoClassSelectorPatternParameters {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(
        CSS_PSEUDO_CLASS_SELECTOR_PATTERN_PARAMETERS as u16,
    ));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_PSEUDO_CLASS_SELECTOR_PATTERN_PARAMETERS }
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
impl std::fmt::Debug for CssPseudoClassSelectorPatternParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssPseudoClassSelectorPatternParameters")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("parameter", &support::DebugSyntaxResult(self.parameter()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssPseudoClassSelectorPatternParameters> for SyntaxNode {
    fn from(n: CssPseudoClassSelectorPatternParameters) -> SyntaxNode { n.syntax }
}
impl From<CssPseudoClassSelectorPatternParameters> for SyntaxElement {
    fn from(n: CssPseudoClassSelectorPatternParameters) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssRatio {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RATIO as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_RATIO }
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
impl std::fmt::Debug for CssRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRatio")
            .field("numerator", &support::DebugSyntaxResult(self.numerator()))
            .field(
                "denominator",
                &support::DebugSyntaxResult(self.denominator()),
            )
            .finish()
    }
}
impl From<CssRatio> for SyntaxNode {
    fn from(n: CssRatio) -> SyntaxNode { n.syntax }
}
impl From<CssRatio> for SyntaxElement {
    fn from(n: CssRatio) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_RULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_RULE }
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
impl std::fmt::Debug for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssRule")
            .field("prelude", &self.prelude())
            .field("block", &support::DebugSyntaxResult(self.block()))
            .finish()
    }
}
impl From<CssRule> for SyntaxNode {
    fn from(n: CssRule) -> SyntaxNode { n.syntax }
}
impl From<CssRule> for SyntaxElement {
    fn from(n: CssRule) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssSelector {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SELECTOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_SELECTOR }
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
impl std::fmt::Debug for CssSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSelector")
            .field("pattern_list", &self.pattern_list())
            .finish()
    }
}
impl From<CssSelector> for SyntaxNode {
    fn from(n: CssSelector) -> SyntaxNode { n.syntax }
}
impl From<CssSelector> for SyntaxElement {
    fn from(n: CssSelector) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssSimpleFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SIMPLE_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_SIMPLE_FUNCTION }
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
impl std::fmt::Debug for CssSimpleFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssSimpleFunction")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("items", &self.items())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssSimpleFunction> for SyntaxNode {
    fn from(n: CssSimpleFunction) -> SyntaxNode { n.syntax }
}
impl From<CssSimpleFunction> for SyntaxElement {
    fn from(n: CssSimpleFunction) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssString {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_STRING as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_STRING }
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
impl std::fmt::Debug for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssString")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<CssString> for SyntaxNode {
    fn from(n: CssString) -> SyntaxNode { n.syntax }
}
impl From<CssString> for SyntaxElement {
    fn from(n: CssString) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssTypeSelectorPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_TYPE_SELECTOR_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_TYPE_SELECTOR_PATTERN }
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
impl std::fmt::Debug for CssTypeSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssTypeSelectorPattern")
            .field("ident", &support::DebugSyntaxResult(self.ident()))
            .finish()
    }
}
impl From<CssTypeSelectorPattern> for SyntaxNode {
    fn from(n: CssTypeSelectorPattern) -> SyntaxNode { n.syntax }
}
impl From<CssTypeSelectorPattern> for SyntaxElement {
    fn from(n: CssTypeSelectorPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssUniversalSelectorPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_UNIVERSAL_SELECTOR_PATTERN as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_UNIVERSAL_SELECTOR_PATTERN }
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
impl std::fmt::Debug for CssUniversalSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssUniversalSelectorPattern")
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .finish()
    }
}
impl From<CssUniversalSelectorPattern> for SyntaxNode {
    fn from(n: CssUniversalSelectorPattern) -> SyntaxNode { n.syntax }
}
impl From<CssUniversalSelectorPattern> for SyntaxElement {
    fn from(n: CssUniversalSelectorPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssVarFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VAR_FUNCTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_VAR_FUNCTION }
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
impl std::fmt::Debug for CssVarFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssVarFunction")
            .field("var_token", &support::DebugSyntaxResult(self.var_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("property", &support::DebugSyntaxResult(self.property()))
            .field("value", &support::DebugOptionalElement(self.value()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<CssVarFunction> for SyntaxNode {
    fn from(n: CssVarFunction) -> SyntaxNode { n.syntax }
}
impl From<CssVarFunction> for SyntaxElement {
    fn from(n: CssVarFunction) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for CssVarFunctionValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_VAR_FUNCTION_VALUE as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_VAR_FUNCTION_VALUE }
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
impl std::fmt::Debug for CssVarFunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssVarFunctionValue")
            .field(
                "comma_token",
                &support::DebugSyntaxResult(self.comma_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<CssVarFunctionValue> for SyntaxNode {
    fn from(n: CssVarFunctionValue) -> SyntaxNode { n.syntax }
}
impl From<CssVarFunctionValue> for SyntaxElement {
    fn from(n: CssVarFunctionValue) -> SyntaxElement { n.syntax.into() }
}
impl From<CssAtMediaQueryFeatureBoolean> for CssAnyAtMediaQueryFeatureType {
    fn from(node: CssAtMediaQueryFeatureBoolean) -> CssAnyAtMediaQueryFeatureType {
        CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(node)
    }
}
impl From<CssAtMediaQueryFeatureCompare> for CssAnyAtMediaQueryFeatureType {
    fn from(node: CssAtMediaQueryFeatureCompare) -> CssAnyAtMediaQueryFeatureType {
        CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(node)
    }
}
impl From<CssAtMediaQueryFeaturePlain> for CssAnyAtMediaQueryFeatureType {
    fn from(node: CssAtMediaQueryFeaturePlain) -> CssAnyAtMediaQueryFeatureType {
        CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(node)
    }
}
impl From<CssAtMediaQueryFeatureRange> for CssAnyAtMediaQueryFeatureType {
    fn from(node: CssAtMediaQueryFeatureRange) -> CssAnyAtMediaQueryFeatureType {
        CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(node)
    }
}
impl AstNode for CssAnyAtMediaQueryFeatureType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtMediaQueryFeatureBoolean::KIND_SET
        .union(CssAtMediaQueryFeatureCompare::KIND_SET)
        .union(CssAtMediaQueryFeaturePlain::KIND_SET)
        .union(CssAtMediaQueryFeatureRange::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN
                | CSS_AT_MEDIA_QUERY_FEATURE_COMPARE
                | CSS_AT_MEDIA_QUERY_FEATURE_PLAIN
                | CSS_AT_MEDIA_QUERY_FEATURE_RANGE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN => {
                CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(
                    CssAtMediaQueryFeatureBoolean { syntax },
                )
            }
            CSS_AT_MEDIA_QUERY_FEATURE_COMPARE => {
                CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(
                    CssAtMediaQueryFeatureCompare { syntax },
                )
            }
            CSS_AT_MEDIA_QUERY_FEATURE_PLAIN => {
                CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(
                    CssAtMediaQueryFeaturePlain { syntax },
                )
            }
            CSS_AT_MEDIA_QUERY_FEATURE_RANGE => {
                CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(
                    CssAtMediaQueryFeatureRange { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(it) => &it.syntax,
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(it) => &it.syntax,
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(it) => &it.syntax,
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(it) => it.syntax,
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(it) => it.syntax,
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(it) => it.syntax,
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for CssAnyAtMediaQueryFeatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<CssAnyAtMediaQueryFeatureType> for SyntaxNode {
    fn from(n: CssAnyAtMediaQueryFeatureType) -> SyntaxNode {
        match n {
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureBoolean(it) => it.into(),
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureCompare(it) => it.into(),
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeaturePlain(it) => it.into(),
            CssAnyAtMediaQueryFeatureType::CssAtMediaQueryFeatureRange(it) => it.into(),
        }
    }
}
impl From<CssAnyAtMediaQueryFeatureType> for SyntaxElement {
    fn from(n: CssAnyAtMediaQueryFeatureType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtMediaQueryFeature> for CssAnyAtMediaQueryType {
    fn from(node: CssAtMediaQueryFeature) -> CssAnyAtMediaQueryType {
        CssAnyAtMediaQueryType::CssAtMediaQueryFeature(node)
    }
}
impl From<CssIdentifier> for CssAnyAtMediaQueryType {
    fn from(node: CssIdentifier) -> CssAnyAtMediaQueryType {
        CssAnyAtMediaQueryType::CssIdentifier(node)
    }
}
impl AstNode for CssAnyAtMediaQueryType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        CssAtMediaQueryFeature::KIND_SET.union(CssIdentifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, CSS_AT_MEDIA_QUERY_FEATURE | CSS_IDENTIFIER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_MEDIA_QUERY_FEATURE => {
                CssAnyAtMediaQueryType::CssAtMediaQueryFeature(CssAtMediaQueryFeature { syntax })
            }
            CSS_IDENTIFIER => CssAnyAtMediaQueryType::CssIdentifier(CssIdentifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssAnyAtMediaQueryType::CssAtMediaQueryFeature(it) => &it.syntax,
            CssAnyAtMediaQueryType::CssIdentifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssAnyAtMediaQueryType::CssAtMediaQueryFeature(it) => it.syntax,
            CssAnyAtMediaQueryType::CssIdentifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for CssAnyAtMediaQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssAnyAtMediaQueryType::CssAtMediaQueryFeature(it) => std::fmt::Debug::fmt(it, f),
            CssAnyAtMediaQueryType::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssAnyAtMediaQueryType> for SyntaxNode {
    fn from(n: CssAnyAtMediaQueryType) -> SyntaxNode {
        match n {
            CssAnyAtMediaQueryType::CssAtMediaQueryFeature(it) => it.into(),
            CssAnyAtMediaQueryType::CssIdentifier(it) => it.into(),
        }
    }
}
impl From<CssAnyAtMediaQueryType> for SyntaxElement {
    fn from(n: CssAnyAtMediaQueryType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAtKeyframes> for CssAnyAtRule {
    fn from(node: CssAtKeyframes) -> CssAnyAtRule { CssAnyAtRule::CssAtKeyframes(node) }
}
impl From<CssAtMedia> for CssAnyAtRule {
    fn from(node: CssAtMedia) -> CssAnyAtRule { CssAnyAtRule::CssAtMedia(node) }
}
impl AstNode for CssAnyAtRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAtKeyframes::KIND_SET.union(CssAtMedia::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, CSS_AT_KEYFRAMES | CSS_AT_MEDIA) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_AT_KEYFRAMES => CssAnyAtRule::CssAtKeyframes(CssAtKeyframes { syntax }),
            CSS_AT_MEDIA => CssAnyAtRule::CssAtMedia(CssAtMedia { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssAnyAtRule::CssAtKeyframes(it) => &it.syntax,
            CssAnyAtRule::CssAtMedia(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssAnyAtRule::CssAtKeyframes(it) => it.syntax,
            CssAnyAtRule::CssAtMedia(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for CssAnyAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssAnyAtRule::CssAtKeyframes(it) => std::fmt::Debug::fmt(it, f),
            CssAnyAtRule::CssAtMedia(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssAnyAtRule> for SyntaxNode {
    fn from(n: CssAnyAtRule) -> SyntaxNode {
        match n {
            CssAnyAtRule::CssAtKeyframes(it) => it.into(),
            CssAnyAtRule::CssAtMedia(it) => it.into(),
        }
    }
}
impl From<CssAnyAtRule> for SyntaxElement {
    fn from(n: CssAnyAtRule) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssRule> for CssAnyRule {
    fn from(node: CssRule) -> CssAnyRule { CssAnyRule::CssRule(node) }
}
impl AstNode for CssAnyRule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAnyAtRule::KIND_SET.union(CssRule::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CSS_RULE => true,
            k if CssAnyAtRule::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_RULE => CssAnyRule::CssRule(CssRule { syntax }),
            _ => {
                if let Some(css_any_at_rule) = CssAnyAtRule::cast(syntax) {
                    return Some(CssAnyRule::CssAnyAtRule(css_any_at_rule));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssAnyRule::CssRule(it) => &it.syntax,
            CssAnyRule::CssAnyAtRule(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssAnyRule::CssRule(it) => it.syntax,
            CssAnyRule::CssAnyAtRule(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for CssAnyRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssAnyRule::CssAnyAtRule(it) => std::fmt::Debug::fmt(it, f),
            CssAnyRule::CssRule(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssAnyRule> for SyntaxNode {
    fn from(n: CssAnyRule) -> SyntaxNode {
        match n {
            CssAnyRule::CssAnyAtRule(it) => it.into(),
            CssAnyRule::CssRule(it) => it.into(),
        }
    }
}
impl From<CssAnyRule> for SyntaxElement {
    fn from(n: CssAnyRule) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAttributeSelectorPattern> for CssAnySelectorPattern {
    fn from(node: CssAttributeSelectorPattern) -> CssAnySelectorPattern {
        CssAnySelectorPattern::CssAttributeSelectorPattern(node)
    }
}
impl From<CssClassSelectorPattern> for CssAnySelectorPattern {
    fn from(node: CssClassSelectorPattern) -> CssAnySelectorPattern {
        CssAnySelectorPattern::CssClassSelectorPattern(node)
    }
}
impl From<CssCombinatorSelectorPattern> for CssAnySelectorPattern {
    fn from(node: CssCombinatorSelectorPattern) -> CssAnySelectorPattern {
        CssAnySelectorPattern::CssCombinatorSelectorPattern(node)
    }
}
impl From<CssIdSelectorPattern> for CssAnySelectorPattern {
    fn from(node: CssIdSelectorPattern) -> CssAnySelectorPattern {
        CssAnySelectorPattern::CssIdSelectorPattern(node)
    }
}
impl From<CssPseudoClassSelectorPattern> for CssAnySelectorPattern {
    fn from(node: CssPseudoClassSelectorPattern) -> CssAnySelectorPattern {
        CssAnySelectorPattern::CssPseudoClassSelectorPattern(node)
    }
}
impl From<CssTypeSelectorPattern> for CssAnySelectorPattern {
    fn from(node: CssTypeSelectorPattern) -> CssAnySelectorPattern {
        CssAnySelectorPattern::CssTypeSelectorPattern(node)
    }
}
impl From<CssUniversalSelectorPattern> for CssAnySelectorPattern {
    fn from(node: CssUniversalSelectorPattern) -> CssAnySelectorPattern {
        CssAnySelectorPattern::CssUniversalSelectorPattern(node)
    }
}
impl AstNode for CssAnySelectorPattern {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAttributeSelectorPattern::KIND_SET
        .union(CssClassSelectorPattern::KIND_SET)
        .union(CssCombinatorSelectorPattern::KIND_SET)
        .union(CssIdSelectorPattern::KIND_SET)
        .union(CssPseudoClassSelectorPattern::KIND_SET)
        .union(CssTypeSelectorPattern::KIND_SET)
        .union(CssUniversalSelectorPattern::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_ATTRIBUTE_SELECTOR_PATTERN
                | CSS_CLASS_SELECTOR_PATTERN
                | CSS_COMBINATOR_SELECTOR_PATTERN
                | CSS_ID_SELECTOR_PATTERN
                | CSS_PSEUDO_CLASS_SELECTOR_PATTERN
                | CSS_TYPE_SELECTOR_PATTERN
                | CSS_UNIVERSAL_SELECTOR_PATTERN
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ATTRIBUTE_SELECTOR_PATTERN => {
                CssAnySelectorPattern::CssAttributeSelectorPattern(CssAttributeSelectorPattern {
                    syntax,
                })
            }
            CSS_CLASS_SELECTOR_PATTERN => {
                CssAnySelectorPattern::CssClassSelectorPattern(CssClassSelectorPattern { syntax })
            }
            CSS_COMBINATOR_SELECTOR_PATTERN => {
                CssAnySelectorPattern::CssCombinatorSelectorPattern(CssCombinatorSelectorPattern {
                    syntax,
                })
            }
            CSS_ID_SELECTOR_PATTERN => {
                CssAnySelectorPattern::CssIdSelectorPattern(CssIdSelectorPattern { syntax })
            }
            CSS_PSEUDO_CLASS_SELECTOR_PATTERN => {
                CssAnySelectorPattern::CssPseudoClassSelectorPattern(
                    CssPseudoClassSelectorPattern { syntax },
                )
            }
            CSS_TYPE_SELECTOR_PATTERN => {
                CssAnySelectorPattern::CssTypeSelectorPattern(CssTypeSelectorPattern { syntax })
            }
            CSS_UNIVERSAL_SELECTOR_PATTERN => {
                CssAnySelectorPattern::CssUniversalSelectorPattern(CssUniversalSelectorPattern {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssAnySelectorPattern::CssAttributeSelectorPattern(it) => &it.syntax,
            CssAnySelectorPattern::CssClassSelectorPattern(it) => &it.syntax,
            CssAnySelectorPattern::CssCombinatorSelectorPattern(it) => &it.syntax,
            CssAnySelectorPattern::CssIdSelectorPattern(it) => &it.syntax,
            CssAnySelectorPattern::CssPseudoClassSelectorPattern(it) => &it.syntax,
            CssAnySelectorPattern::CssTypeSelectorPattern(it) => &it.syntax,
            CssAnySelectorPattern::CssUniversalSelectorPattern(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssAnySelectorPattern::CssAttributeSelectorPattern(it) => it.syntax,
            CssAnySelectorPattern::CssClassSelectorPattern(it) => it.syntax,
            CssAnySelectorPattern::CssCombinatorSelectorPattern(it) => it.syntax,
            CssAnySelectorPattern::CssIdSelectorPattern(it) => it.syntax,
            CssAnySelectorPattern::CssPseudoClassSelectorPattern(it) => it.syntax,
            CssAnySelectorPattern::CssTypeSelectorPattern(it) => it.syntax,
            CssAnySelectorPattern::CssUniversalSelectorPattern(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for CssAnySelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssAnySelectorPattern::CssAttributeSelectorPattern(it) => std::fmt::Debug::fmt(it, f),
            CssAnySelectorPattern::CssClassSelectorPattern(it) => std::fmt::Debug::fmt(it, f),
            CssAnySelectorPattern::CssCombinatorSelectorPattern(it) => std::fmt::Debug::fmt(it, f),
            CssAnySelectorPattern::CssIdSelectorPattern(it) => std::fmt::Debug::fmt(it, f),
            CssAnySelectorPattern::CssPseudoClassSelectorPattern(it) => std::fmt::Debug::fmt(it, f),
            CssAnySelectorPattern::CssTypeSelectorPattern(it) => std::fmt::Debug::fmt(it, f),
            CssAnySelectorPattern::CssUniversalSelectorPattern(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssAnySelectorPattern> for SyntaxNode {
    fn from(n: CssAnySelectorPattern) -> SyntaxNode {
        match n {
            CssAnySelectorPattern::CssAttributeSelectorPattern(it) => it.into(),
            CssAnySelectorPattern::CssClassSelectorPattern(it) => it.into(),
            CssAnySelectorPattern::CssCombinatorSelectorPattern(it) => it.into(),
            CssAnySelectorPattern::CssIdSelectorPattern(it) => it.into(),
            CssAnySelectorPattern::CssPseudoClassSelectorPattern(it) => it.into(),
            CssAnySelectorPattern::CssTypeSelectorPattern(it) => it.into(),
            CssAnySelectorPattern::CssUniversalSelectorPattern(it) => it.into(),
        }
    }
}
impl From<CssAnySelectorPattern> for SyntaxElement {
    fn from(n: CssAnySelectorPattern) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<CssAnyFunction> for CssAnyValue {
    fn from(node: CssAnyFunction) -> CssAnyValue { CssAnyValue::CssAnyFunction(node) }
}
impl From<CssCustomProperty> for CssAnyValue {
    fn from(node: CssCustomProperty) -> CssAnyValue { CssAnyValue::CssCustomProperty(node) }
}
impl From<CssDimension> for CssAnyValue {
    fn from(node: CssDimension) -> CssAnyValue { CssAnyValue::CssDimension(node) }
}
impl From<CssIdentifier> for CssAnyValue {
    fn from(node: CssIdentifier) -> CssAnyValue { CssAnyValue::CssIdentifier(node) }
}
impl From<CssNumber> for CssAnyValue {
    fn from(node: CssNumber) -> CssAnyValue { CssAnyValue::CssNumber(node) }
}
impl From<CssRatio> for CssAnyValue {
    fn from(node: CssRatio) -> CssAnyValue { CssAnyValue::CssRatio(node) }
}
impl From<CssString> for CssAnyValue {
    fn from(node: CssString) -> CssAnyValue { CssAnyValue::CssString(node) }
}
impl AstNode for CssAnyValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = CssAnyFunction::KIND_SET
        .union(CssCustomProperty::KIND_SET)
        .union(CssDimension::KIND_SET)
        .union(CssIdentifier::KIND_SET)
        .union(CssNumber::KIND_SET)
        .union(CssRatio::KIND_SET)
        .union(CssString::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            CSS_ANY_FUNCTION
                | CSS_CUSTOM_PROPERTY
                | CSS_DIMENSION
                | CSS_IDENTIFIER
                | CSS_NUMBER
                | CSS_RATIO
                | CSS_STRING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CSS_ANY_FUNCTION => CssAnyValue::CssAnyFunction(CssAnyFunction { syntax }),
            CSS_CUSTOM_PROPERTY => CssAnyValue::CssCustomProperty(CssCustomProperty { syntax }),
            CSS_DIMENSION => CssAnyValue::CssDimension(CssDimension { syntax }),
            CSS_IDENTIFIER => CssAnyValue::CssIdentifier(CssIdentifier { syntax }),
            CSS_NUMBER => CssAnyValue::CssNumber(CssNumber { syntax }),
            CSS_RATIO => CssAnyValue::CssRatio(CssRatio { syntax }),
            CSS_STRING => CssAnyValue::CssString(CssString { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            CssAnyValue::CssAnyFunction(it) => &it.syntax,
            CssAnyValue::CssCustomProperty(it) => &it.syntax,
            CssAnyValue::CssDimension(it) => &it.syntax,
            CssAnyValue::CssIdentifier(it) => &it.syntax,
            CssAnyValue::CssNumber(it) => &it.syntax,
            CssAnyValue::CssRatio(it) => &it.syntax,
            CssAnyValue::CssString(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            CssAnyValue::CssAnyFunction(it) => it.syntax,
            CssAnyValue::CssCustomProperty(it) => it.syntax,
            CssAnyValue::CssDimension(it) => it.syntax,
            CssAnyValue::CssIdentifier(it) => it.syntax,
            CssAnyValue::CssNumber(it) => it.syntax,
            CssAnyValue::CssRatio(it) => it.syntax,
            CssAnyValue::CssString(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for CssAnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CssAnyValue::CssAnyFunction(it) => std::fmt::Debug::fmt(it, f),
            CssAnyValue::CssCustomProperty(it) => std::fmt::Debug::fmt(it, f),
            CssAnyValue::CssDimension(it) => std::fmt::Debug::fmt(it, f),
            CssAnyValue::CssIdentifier(it) => std::fmt::Debug::fmt(it, f),
            CssAnyValue::CssNumber(it) => std::fmt::Debug::fmt(it, f),
            CssAnyValue::CssRatio(it) => std::fmt::Debug::fmt(it, f),
            CssAnyValue::CssString(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<CssAnyValue> for SyntaxNode {
    fn from(n: CssAnyValue) -> SyntaxNode {
        match n {
            CssAnyValue::CssAnyFunction(it) => it.into(),
            CssAnyValue::CssCustomProperty(it) => it.into(),
            CssAnyValue::CssDimension(it) => it.into(),
            CssAnyValue::CssIdentifier(it) => it.into(),
            CssAnyValue::CssNumber(it) => it.into(),
            CssAnyValue::CssRatio(it) => it.into(),
            CssAnyValue::CssString(it) => it.into(),
        }
    }
}
impl From<CssAnyValue> for SyntaxElement {
    fn from(n: CssAnyValue) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for CssAnyAtMediaQueryFeatureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAnyAtMediaQueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAnyAtRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAnyRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAnySelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtKeyframes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtKeyframesBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMedia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryConsequent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeatureBoolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeatureCompare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeaturePlain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryFeatureRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAtMediaQueryRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssAttributeSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssClassSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCombinatorSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssCustomProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDeclarationImportant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssIdSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssKeyframesSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssPseudoClassSelectorPatternParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssSimpleFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssTypeSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssUniversalSelectorPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssVarFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CssVarFunctionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct CssBogus {
    syntax: SyntaxNode,
}
impl CssBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for CssBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_BOGUS }
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
impl std::fmt::Debug for CssBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<CssBogus> for SyntaxNode {
    fn from(n: CssBogus) -> SyntaxNode { n.syntax }
}
impl From<CssBogus> for SyntaxElement {
    fn from(n: CssBogus) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssAnySelectorPatternList {
    syntax_list: SyntaxList,
}
impl CssAnySelectorPatternList {
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
impl AstNode for CssAnySelectorPatternList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ANY_SELECTOR_PATTERN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ANY_SELECTOR_PATTERN_LIST }
    fn cast(syntax: SyntaxNode) -> Option<CssAnySelectorPatternList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssAnySelectorPatternList {
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
impl Serialize for CssAnySelectorPatternList {
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
impl AstNodeList for CssAnySelectorPatternList {
    type Language = Language;
    type Node = CssAnySelectorPattern;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssAnySelectorPatternList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssAnySelectorPatternList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssAnySelectorPatternList {
    type Item = CssAnySelectorPattern;
    type IntoIter = AstNodeListIterator<Language, CssAnySelectorPattern>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for CssAnySelectorPatternList {
    type Item = CssAnySelectorPattern;
    type IntoIter = AstNodeListIterator<Language, CssAnySelectorPattern>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssAtKeyframesItemList {
    syntax_list: SyntaxList,
}
impl CssAtKeyframesItemList {
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
impl AstNode for CssAtKeyframesItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_KEYFRAMES_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_KEYFRAMES_ITEM_LIST }
    fn cast(syntax: SyntaxNode) -> Option<CssAtKeyframesItemList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssAtKeyframesItemList {
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
impl Serialize for CssAtKeyframesItemList {
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
impl AstNodeList for CssAtKeyframesItemList {
    type Language = Language;
    type Node = CssKeyframesBlock;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssAtKeyframesItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssAtKeyframesItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssAtKeyframesItemList {
    type Item = CssKeyframesBlock;
    type IntoIter = AstNodeListIterator<Language, CssKeyframesBlock>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for CssAtKeyframesItemList {
    type Item = CssKeyframesBlock;
    type IntoIter = AstNodeListIterator<Language, CssKeyframesBlock>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssAtMediaQueryList {
    syntax_list: SyntaxList,
}
impl CssAtMediaQueryList {
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
impl AstNode for CssAtMediaQueryList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_AT_MEDIA_QUERY_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_AT_MEDIA_QUERY_LIST }
    fn cast(syntax: SyntaxNode) -> Option<CssAtMediaQueryList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssAtMediaQueryList {
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
impl Serialize for CssAtMediaQueryList {
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
impl AstSeparatedList for CssAtMediaQueryList {
    type Language = Language;
    type Node = CssAtMediaQuery;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssAtMediaQueryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssAtMediaQueryList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssAtMediaQueryList {
    type Item = SyntaxResult<CssAtMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssAtMediaQuery>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &CssAtMediaQueryList {
    type Item = SyntaxResult<CssAtMediaQuery>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssAtMediaQuery>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssAttributeList {
    syntax_list: SyntaxList,
}
impl CssAttributeList {
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
impl AstNode for CssAttributeList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ATTRIBUTE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ATTRIBUTE_LIST }
    fn cast(syntax: SyntaxNode) -> Option<CssAttributeList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssAttributeList {
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
impl Serialize for CssAttributeList {
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
impl AstNodeList for CssAttributeList {
    type Language = Language;
    type Node = CssAttribute;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssAttributeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssAttributeList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssAttributeList {
    type Item = CssAttribute;
    type IntoIter = AstNodeListIterator<Language, CssAttribute>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for CssAttributeList {
    type Item = CssAttribute;
    type IntoIter = AstNodeListIterator<Language, CssAttribute>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssDeclarationList {
    syntax_list: SyntaxList,
}
impl CssDeclarationList {
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
impl AstNode for CssDeclarationList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_DECLARATION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_DECLARATION_LIST }
    fn cast(syntax: SyntaxNode) -> Option<CssDeclarationList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssDeclarationList {
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
impl Serialize for CssDeclarationList {
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
impl AstNodeList for CssDeclarationList {
    type Language = Language;
    type Node = CssDeclaration;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssDeclarationList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssDeclarationList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssDeclarationList {
    type Item = CssDeclaration;
    type IntoIter = AstNodeListIterator<Language, CssDeclaration>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for CssDeclarationList {
    type Item = CssDeclaration;
    type IntoIter = AstNodeListIterator<Language, CssDeclaration>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssKeyframesSelectorList {
    syntax_list: SyntaxList,
}
impl CssKeyframesSelectorList {
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
impl AstNode for CssKeyframesSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_KEYFRAMES_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_KEYFRAMES_SELECTOR_LIST }
    fn cast(syntax: SyntaxNode) -> Option<CssKeyframesSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssKeyframesSelectorList {
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
impl Serialize for CssKeyframesSelectorList {
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
impl AstSeparatedList for CssKeyframesSelectorList {
    type Language = Language;
    type Node = CssKeyframesSelector;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssKeyframesSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssKeyframesSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssKeyframesSelectorList {
    type Item = SyntaxResult<CssKeyframesSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssKeyframesSelector>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &CssKeyframesSelectorList {
    type Item = SyntaxResult<CssKeyframesSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssKeyframesSelector>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssParameterList {
    syntax_list: SyntaxList,
}
impl CssParameterList {
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
impl AstNode for CssParameterList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_PARAMETER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_PARAMETER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<CssParameterList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssParameterList {
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
impl Serialize for CssParameterList {
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
impl AstNodeList for CssParameterList {
    type Language = Language;
    type Node = CssParameter;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssParameterList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssParameterList {
    type Item = CssParameter;
    type IntoIter = AstNodeListIterator<Language, CssParameter>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for CssParameterList {
    type Item = CssParameter;
    type IntoIter = AstNodeListIterator<Language, CssParameter>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssRoot {
    syntax_list: SyntaxList,
}
impl CssRoot {
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
impl AstNode for CssRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_ROOT }
    fn cast(syntax: SyntaxNode) -> Option<CssRoot> {
        if Self::can_cast(syntax.kind()) {
            Some(CssRoot {
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
impl Serialize for CssRoot {
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
impl AstNodeList for CssRoot {
    type Language = Language;
    type Node = CssAnyRule;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssRoot ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &CssRoot {
    type Item = CssAnyRule;
    type IntoIter = AstNodeListIterator<Language, CssAnyRule>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for CssRoot {
    type Item = CssAnyRule;
    type IntoIter = AstNodeListIterator<Language, CssAnyRule>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CssSelectorList {
    syntax_list: SyntaxList,
}
impl CssSelectorList {
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
impl AstNode for CssSelectorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(CSS_SELECTOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool { kind == CSS_SELECTOR_LIST }
    fn cast(syntax: SyntaxNode) -> Option<CssSelectorList> {
        if Self::can_cast(syntax.kind()) {
            Some(CssSelectorList {
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
impl Serialize for CssSelectorList {
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
impl AstSeparatedList for CssSelectorList {
    type Language = Language;
    type Node = CssSelector;
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
    fn into_syntax_list(self) -> SyntaxList { self.syntax_list }
}
impl Debug for CssSelectorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("CssSelectorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for CssSelectorList {
    type Item = SyntaxResult<CssSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssSelector>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &CssSelectorList {
    type Item = SyntaxResult<CssSelector>;
    type IntoIter = AstSeparatedListNodesIterator<Language, CssSelector>;
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
