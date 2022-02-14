//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
    ast::*,
    JsSyntaxKind::{self, *},
    SyntaxNode, SyntaxResult, SyntaxToken,
};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImportMeta {
    pub(crate) syntax: SyntaxNode,
}
impl ImportMeta {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn meta_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentPattern {
    pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> JsArrayAssignmentPatternElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentPatternRestElement {
    pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentPatternRestElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayBindingPattern {
    pub(crate) syntax: SyntaxNode,
}
impl JsArrayBindingPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> JsArrayBindingPatternElementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayBindingPatternRestElement {
    pub(crate) syntax: SyntaxNode,
}
impl JsArrayBindingPatternRestElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsArrayExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> JsArrayElementList { support::list(&self.syntax, 1usize) }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayHole {
    pub(crate) syntax: SyntaxNode,
}
impl JsArrayHole {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrowFunctionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsArrowFunctionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 1usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsAnyArrowFunctionParameters> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 3usize)
    }
    pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn body(&self) -> SyntaxResult<JsAnyFunctionBody> {
        support::required_node(&self.syntax, 5usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAssignmentExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsAssignmentExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn left(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAssignmentWithDefault {
    pub(crate) syntax: SyntaxNode,
}
impl JsAssignmentWithDefault {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn default(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAwaitExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsAwaitExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn await_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBigIntLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsBigIntLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBinaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsBinaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn left(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBindingPatternWithDefault {
    pub(crate) syntax: SyntaxNode,
}
impl JsBindingPatternWithDefault {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn default(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBlockStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsBlockStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn statements(&self) -> JsStatementList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBooleanLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsBooleanLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBreakStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsBreakStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn break_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCallArguments {
    pub(crate) syntax: SyntaxNode,
}
impl JsCallArguments {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn args(&self) -> JsCallArgumentList { support::list(&self.syntax, 1usize) }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCallExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsCallExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn callee(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn optional_chain_token_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn type_arguments(&self) -> Option<TsTypeArguments> { support::node(&self.syntax, 2usize) }
    pub fn arguments(&self) -> SyntaxResult<JsCallArguments> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCaseClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsCaseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn case_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn test(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn consequent(&self) -> JsStatementList { support::list(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCatchClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsCatchClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn catch_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declaration(&self) -> Option<JsCatchDeclaration> { support::node(&self.syntax, 1usize) }
    pub fn body(&self) -> SyntaxResult<JsBlockStatement> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCatchDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl JsCatchDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn binding(&self) -> SyntaxResult<JsAnyBindingPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsClassDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl JsClassDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn id(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax, 1usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 2usize)
    }
    pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::node(&self.syntax, 3usize) }
    pub fn implements_clause(&self) -> Option<TsImplementsClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn members(&self) -> JsClassMemberList { support::list(&self.syntax, 6usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsClassExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsClassExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn id(&self) -> Option<JsAnyBinding> { support::node(&self.syntax, 1usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 2usize)
    }
    pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::node(&self.syntax, 3usize) }
    pub fn implements_clause(&self) -> Option<TsImplementsClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn members(&self) -> JsClassMemberList { support::list(&self.syntax, 6usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn object(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn object(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn optional_chain_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn member(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConditionalExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsConditionalExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn test(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn consequent(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn alternate(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConstructorClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsConstructorClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn access_modifier(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn name(&self) -> SyntaxResult<JsLiteralMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsConstructorParameters> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConstructorParameters {
    pub(crate) syntax: SyntaxNode,
}
impl JsConstructorParameters {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn parameters(&self) -> JsConstructorParameterList { support::list(&self.syntax, 1usize) }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsContinueStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsContinueStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn continue_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDebuggerStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsDebuggerStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn debugger_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDefaultClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsDefaultClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn default_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn consequent(&self) -> JsStatementList { support::list(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDefaultImportSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl JsDefaultImportSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn trailing_comma_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDirective {
    pub(crate) syntax: SyntaxNode,
}
impl JsDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDoWhileStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsDoWhileStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn do_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn test(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 6usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn alternate(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsEmptyClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsEmptyClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsEmptyStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsEmptyStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExport {
    pub(crate) syntax: SyntaxNode,
}
impl JsExport {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn export_clause(&self) -> SyntaxResult<JsAnyExportClause> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportAsClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportAsClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn exported_name(&self) -> SyntaxResult<JsLiteralExportName> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportDefaultClassClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportDefaultClassClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn default_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn id(&self) -> Option<JsAnyBinding> { support::node(&self.syntax, 2usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 3usize)
    }
    pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::node(&self.syntax, 4usize) }
    pub fn implements_clause(&self) -> Option<TsImplementsClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn members(&self) -> JsClassMemberList { support::list(&self.syntax, 7usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 8usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportDefaultExpressionClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportDefaultExpressionClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn default_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportDefaultFunctionClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportDefaultFunctionClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn default_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 3usize) }
    pub fn id(&self) -> Option<JsAnyBinding> { support::node(&self.syntax, 4usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 5usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 7usize)
    }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 8usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportFromClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportFromClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn export_as(&self) -> Option<JsExportAsClause> { support::node(&self.syntax, 1usize) }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax, 4usize) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportNamedClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportNamedClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn specifiers(&self) -> JsExportNamedSpecifierList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportNamedFromClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportNamedFromClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn specifiers(&self) -> JsExportNamedFromSpecifierList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax, 5usize) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 6usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportNamedFromSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportNamedFromSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn source_name(&self) -> SyntaxResult<JsLiteralExportName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn export_as(&self) -> Option<JsExportAsClause> { support::node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportNamedShorthandSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportNamedShorthandSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn name(&self) -> SyntaxResult<JsReferenceIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExportNamedSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl JsExportNamedSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn local_name(&self) -> SyntaxResult<JsReferenceIdentifier> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn exported_name(&self) -> SyntaxResult<JsLiteralExportName> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExpressionSnipped {
    pub(crate) syntax: SyntaxNode,
}
impl JsExpressionSnipped {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExpressionStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsExpressionStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExtendsClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsExtendsClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn super_class(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn type_arguments(&self) -> Option<TsTypeArguments> { support::node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFinallyClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsFinallyClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn finally_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<JsBlockStatement> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsForInStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsForInStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn initializer(&self) -> SyntaxResult<JsAnyForInOrOfInitializer> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn body(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 6usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsForOfStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsForOfStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn await_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn initializer(&self) -> SyntaxResult<JsAnyForInOrOfInitializer> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn of_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn body(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 7usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsForStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsForStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn initializer(&self) -> Option<JsAnyForInitializer> { support::node(&self.syntax, 2usize) }
    pub fn first_semi_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn test(&self) -> Option<JsAnyExpression> { support::node(&self.syntax, 4usize) }
    pub fn second_semi_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn update(&self) -> Option<JsAnyExpression> { support::node(&self.syntax, 6usize) }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
    pub fn body(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 8usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsForVariableDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl JsForVariableDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn kind_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declarator(&self) -> SyntaxResult<JsVariableDeclarator> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFormalParameter {
    pub(crate) syntax: SyntaxNode,
}
impl JsFormalParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn binding(&self) -> SyntaxResult<JsAnyBindingPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn question_mark_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        support::node(&self.syntax, 2usize)
    }
    pub fn initializer(&self) -> Option<JsInitializerClause> { support::node(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionBody {
    pub(crate) syntax: SyntaxNode,
}
impl JsFunctionBody {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn directives(&self) -> JsDirectiveList { support::list(&self.syntax, 1usize) }
    pub fn statements(&self) -> JsStatementList { support::list(&self.syntax, 2usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl JsFunctionDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
    pub fn id(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax, 3usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 4usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 6usize)
    }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 7usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsFunctionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
    pub fn id(&self) -> Option<JsAnyBinding> { support::node(&self.syntax, 3usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 4usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 6usize)
    }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 7usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsGetterClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsGetterClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn access_modifier(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn abstract_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
    pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax, 7usize) }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 8usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsGetterObjectMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsGetterObjectMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax, 4usize) }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 5usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierBinding {
    pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<JsReferenceIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIfStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsIfStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn test(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn consequent(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn else_clause(&self) -> Option<JsElseClause> { support::node(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImport {
    pub(crate) syntax: SyntaxNode,
}
impl JsImport {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn import_clause(&self) -> SyntaxResult<JsAnyImportClause> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportAssertion {
    pub(crate) syntax: SyntaxNode,
}
impl JsImportAssertion {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn assert_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn assertions(&self) -> JsImportAssertionEntryList { support::list(&self.syntax, 2usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportAssertionEntry {
    pub(crate) syntax: SyntaxNode,
}
impl JsImportAssertionEntry {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn key(&self) -> SyntaxResult<SyntaxToken> { support::required_token(&self.syntax, 0usize) }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportBareClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsImportBareClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportCallExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsImportCallExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn arguments(&self) -> SyntaxResult<JsCallArguments> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportDefaultClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsImportDefaultClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportNamedClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsImportNamedClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn default_specifier(&self) -> Option<JsDefaultImportSpecifier> {
        support::node(&self.syntax, 0usize)
    }
    pub fn named_import(&self) -> SyntaxResult<JsAnyNamedImport> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax, 4usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportNamespaceClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsImportNamespaceClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsInExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsInExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn property(&self) -> SyntaxResult<JsAnyInProperty> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn object(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsInitializerClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsInitializerClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsInstanceofExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsInstanceofExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn left(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn instanceof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLabeledStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsLabeledStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn label_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn body(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLiteralExportName {
    pub(crate) syntax: SyntaxNode,
}
impl JsLiteralExportName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLiteralMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl JsLiteralMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLogicalExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsLogicalExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn left(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsMethodClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsMethodClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn access_modifier(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn abstract_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 3usize) }
    pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 4usize) }
    pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 7usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 8usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 9usize)
    }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 10usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsMethodObjectMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsMethodObjectMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 3usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 5usize)
    }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 6usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsModule {
    pub(crate) syntax: SyntaxNode,
}
impl JsModule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn interpreter_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn directives(&self) -> JsDirectiveList { support::list(&self.syntax, 1usize) }
    pub fn items(&self) -> JsModuleItemList { support::list(&self.syntax, 2usize) }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsModuleSource {
    pub(crate) syntax: SyntaxNode,
}
impl JsModuleSource {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsName {
    pub(crate) syntax: SyntaxNode,
}
impl JsName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamedImportSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl JsNamedImportSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<JsLiteralExportName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamedImportSpecifiers {
    pub(crate) syntax: SyntaxNode,
}
impl JsNamedImportSpecifiers {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn specifiers(&self) -> JsNamedImportSpecifierList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamespaceImportSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl JsNamespaceImportSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNewExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsNewExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn callee(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn type_arguments(&self) -> Option<TsTypeArguments> { support::node(&self.syntax, 2usize) }
    pub fn arguments(&self) -> Option<JsCallArguments> { support::node(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNullLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsNullLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNumberLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsNumberLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPattern {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn properties(&self) -> JsObjectAssignmentPatternPropertyList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternProperty {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn member(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn init(&self) -> Option<JsInitializerClause> { support::node(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternRest {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternRest {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn target(&self) -> SyntaxResult<JsAnyAssignment> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternShorthandProperty {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternShorthandProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn identifier(&self) -> SyntaxResult<JsAnyAssignment> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn init(&self) -> Option<JsInitializerClause> { support::node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPattern {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPattern {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn properties(&self) -> JsObjectBindingPatternPropertyList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternProperty {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn member(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn init(&self) -> Option<JsInitializerClause> { support::node(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternRest {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternRest {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn binding(&self) -> SyntaxResult<JsAnyBinding> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternShorthandProperty {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternShorthandProperty {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn identifier(&self) -> SyntaxResult<JsAnyBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn init(&self) -> Option<JsInitializerClause> { support::node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsObjectExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn members(&self) -> JsObjectMemberList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParameters {
    pub(crate) syntax: SyntaxNode,
}
impl JsParameters {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> JsParameterList { support::list(&self.syntax, 1usize) }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParenthesizedAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl JsParenthesizedAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn assignment(&self) -> SyntaxResult<JsAnyAssignment> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParenthesizedExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsParenthesizedExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPostUpdateExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsPostUpdateExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn operand(&self) -> SyntaxResult<JsAnyAssignment> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPreUpdateExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsPreUpdateExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn operand(&self) -> SyntaxResult<JsAnyAssignment> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPrivateClassMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl JsPrivateClassMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn id_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPrivateName {
    pub(crate) syntax: SyntaxNode,
}
impl JsPrivateName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsPropertyClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn declare_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn access_modifier(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
    pub fn readonly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 3usize) }
    pub fn abstract_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 4usize) }
    pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn property_annotation(&self) -> Option<TsAnyPropertyAnnotation> {
        support::node(&self.syntax, 6usize)
    }
    pub fn value(&self) -> Option<JsInitializerClause> { support::node(&self.syntax, 7usize) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 8usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyObjectMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsPropertyObjectMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsReferenceIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl JsReferenceIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsRegexLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsRegexLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsRestParameter {
    pub(crate) syntax: SyntaxNode,
}
impl JsRestParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn binding(&self) -> SyntaxResult<JsAnyBindingPattern> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        support::node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsReturnStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsReturnStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn return_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> Option<JsAnyExpression> { support::node(&self.syntax, 1usize) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsScript {
    pub(crate) syntax: SyntaxNode,
}
impl JsScript {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn interpreter_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn directives(&self) -> JsDirectiveList { support::list(&self.syntax, 1usize) }
    pub fn statements(&self) -> JsStatementList { support::list(&self.syntax, 2usize) }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSequenceExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsSequenceExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn left(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSetterClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsSetterClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn access_modifier(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn abstract_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
    pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn parameter(&self) -> SyntaxResult<JsAnyFormalParameter> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 8usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSetterObjectMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsSetterObjectMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn parameter(&self) -> SyntaxResult<JsAnyFormalParameter> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn body(&self) -> SyntaxResult<JsFunctionBody> {
        support::required_node(&self.syntax, 5usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsShorthandNamedImportSpecifier {
    pub(crate) syntax: SyntaxNode,
}
impl JsShorthandNamedImportSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsShorthandPropertyObjectMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsShorthandPropertyObjectMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<JsReferenceIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSpread {
    pub(crate) syntax: SyntaxNode,
}
impl JsSpread {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStaticInitializationBlockClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl JsStaticInitializationBlockClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn static_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn statements(&self) -> JsStatementList { support::list(&self.syntax, 2usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStaticMemberAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl JsStaticMemberAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn object(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<JsAnyName> { support::required_node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStaticMemberExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsStaticMemberExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn object(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<JsAnyName> { support::required_node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStringLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsStringLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSuperExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsSuperExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn super_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSwitchStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsSwitchStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn switch_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn discriminant(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn cases(&self) -> JsSwitchCaseList { support::list(&self.syntax, 5usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTemplate {
    pub(crate) syntax: SyntaxNode,
}
impl JsTemplate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn tag(&self) -> Option<JsAnyExpression> { support::node(&self.syntax, 0usize) }
    pub fn type_arguments(&self) -> Option<TsTypeArguments> { support::node(&self.syntax, 1usize) }
    pub fn l_tick_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn elements(&self) -> JsTemplateElementList { support::list(&self.syntax, 3usize) }
    pub fn r_tick_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTemplateChunkElement {
    pub(crate) syntax: SyntaxNode,
}
impl JsTemplateChunkElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn template_chunk_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTemplateElement {
    pub(crate) syntax: SyntaxNode,
}
impl JsTemplateElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dollar_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsThisExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsThisExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsThrowStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsThrowStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn throw_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTryFinallyStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsTryFinallyStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<JsBlockStatement> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn catch_clause(&self) -> Option<JsCatchClause> { support::node(&self.syntax, 2usize) }
    pub fn finally_clause(&self) -> SyntaxResult<JsFinallyClause> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTryStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsTryStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<JsBlockStatement> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn catch_clause(&self) -> SyntaxResult<JsCatchClause> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsUnaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declarators(&self) -> JsVariableDeclaratorList { support::list(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclarationClause {
    pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclarationClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn declaration(&self) -> SyntaxResult<JsVariableDeclaration> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclarator {
    pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclarator {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn id(&self) -> SyntaxResult<JsAnyBindingPattern> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn variable_annotation(&self) -> Option<TsAnyVariableAnnotation> {
        support::node(&self.syntax, 1usize)
    }
    pub fn initializer(&self) -> Option<JsInitializerClause> { support::node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsVariableStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn declaration(&self) -> SyntaxResult<JsVariableDeclaration> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsWhileStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsWhileStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn test(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsWithStatement {
    pub(crate) syntax: SyntaxNode,
}
impl JsWithStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn with_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn object(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<JsAnyStatement> {
        support::required_node(&self.syntax, 4usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsYieldArgument {
    pub(crate) syntax: SyntaxNode,
}
impl JsYieldArgument {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsYieldExpression {
    pub(crate) syntax: SyntaxNode,
}
impl JsYieldExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn yield_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> Option<JsYieldArgument> { support::node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NewTarget {
    pub(crate) syntax: SyntaxNode,
}
impl NewTarget {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn target_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsAnyType {
    pub(crate) syntax: SyntaxNode,
}
impl TsAnyType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn any_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsArrayType {
    pub(crate) syntax: SyntaxNode,
}
impl TsArrayType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn element_type(&self) -> SyntaxResult<TsType> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsAsExpression {
    pub(crate) syntax: SyntaxNode,
}
impl TsAsExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsBigIntLiteralType {
    pub(crate) syntax: SyntaxNode,
}
impl TsBigIntLiteralType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn minus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsBigintType {
    pub(crate) syntax: SyntaxNode,
}
impl TsBigintType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn bigint_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsBooleanLiteralType {
    pub(crate) syntax: SyntaxNode,
}
impl TsBooleanLiteralType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn literal(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsBooleanType {
    pub(crate) syntax: SyntaxNode,
}
impl TsBooleanType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn boolean_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsCallSignatureTypeMember {
    pub(crate) syntax: SyntaxNode,
}
impl TsCallSignatureTypeMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 0usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 2usize)
    }
    pub fn separator_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConditionalType {
    pub(crate) syntax: SyntaxNode,
}
impl TsConditionalType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn check_type(&self) -> SyntaxResult<TsType> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn extends_type(&self) -> SyntaxResult<TsType> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn true_type(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 4usize) }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn false_type(&self) -> SyntaxResult<TsType> {
        support::required_node(&self.syntax, 6usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstructSignatureTypeMember {
    pub(crate) syntax: SyntaxNode,
}
impl TsConstructSignatureTypeMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 1usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        support::node(&self.syntax, 3usize)
    }
    pub fn separator_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 4usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstructorType {
    pub(crate) syntax: SyntaxNode,
}
impl TsConstructorType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn abstract_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 2usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn return_type(&self) -> SyntaxResult<TsType> {
        support::required_node(&self.syntax, 5usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsDeclareFunctionDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl TsDeclareFunctionDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn id(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax, 2usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 3usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 5usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 6usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsDeclareStatement {
    pub(crate) syntax: SyntaxNode,
}
impl TsDeclareStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn declare_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declaration(&self) -> SyntaxResult<JsAnyDeclarationClause> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsDefaultTypeClause {
    pub(crate) syntax: SyntaxNode,
}
impl TsDefaultTypeClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsDefinitePropertyAnnotation {
    pub(crate) syntax: SyntaxNode,
}
impl TsDefinitePropertyAnnotation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_annotation(&self) -> SyntaxResult<TsTypeAnnotation> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsDefiniteVariableAnnotation {
    pub(crate) syntax: SyntaxNode,
}
impl TsDefiniteVariableAnnotation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_annotation(&self) -> SyntaxResult<TsTypeAnnotation> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsEmptyExternalModuleDeclarationBody {
    pub(crate) syntax: SyntaxNode,
}
impl TsEmptyExternalModuleDeclarationBody {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsEnumDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl TsEnumDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn enum_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn id(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax, 2usize) }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn members(&self) -> TsEnumMemberList { support::list(&self.syntax, 4usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsEnumMember {
    pub(crate) syntax: SyntaxNode,
}
impl TsEnumMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn initializer(&self) -> Option<JsInitializerClause> { support::node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExtendsClause {
    pub(crate) syntax: SyntaxNode,
}
impl TsExtendsClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn types(&self) -> TsTypeList { support::list(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExternalModuleDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl TsExternalModuleDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn module_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn source(&self) -> SyntaxResult<JsModuleSource> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn body(&self) -> Option<TsAnyExternalModuleDeclarationBody> {
        support::node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExternalModuleRef {
    pub(crate) syntax: SyntaxNode,
}
impl TsExternalModuleRef {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn require_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn module_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsFunctionType {
    pub(crate) syntax: SyntaxNode,
}
impl TsFunctionType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 0usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn return_type(&self) -> SyntaxResult<TsAnyReturnType> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsGetterSignatureTypeMember {
    pub(crate) syntax: SyntaxNode,
}
impl TsGetterSignatureTypeMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        support::node(&self.syntax, 4usize)
    }
    pub fn separator_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsGlobalDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl TsGlobalDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn global_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<TsModuleBlock> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIdentifierBinding {
    pub(crate) syntax: SyntaxNode,
}
impl TsIdentifierBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImplementsClause {
    pub(crate) syntax: SyntaxNode,
}
impl TsImplementsClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn implements_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn types(&self) -> TsTypeList { support::list(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImportEqualsDecl {
    pub(crate) syntax: SyntaxNode,
}
impl TsImportEqualsDecl {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn module(&self) -> SyntaxResult<TsModuleRef> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImportType {
    pub(crate) syntax: SyntaxNode,
}
impl TsImportType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn typeof_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn argument_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn qualifier_clause(&self) -> Option<TsImportTypeQualifier> {
        support::node(&self.syntax, 5usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImportTypeQualifier {
    pub(crate) syntax: SyntaxNode,
}
impl TsImportTypeQualifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn right(&self) -> SyntaxResult<TsAnyName> { support::required_node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIndexSignatureParameter {
    pub(crate) syntax: SyntaxNode,
}
impl TsIndexSignatureParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn binding(&self) -> SyntaxResult<JsIdentifierBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn type_annotation(&self) -> SyntaxResult<TsTypeAnnotation> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIndexSignatureTypeMember {
    pub(crate) syntax: SyntaxNode,
}
impl TsIndexSignatureTypeMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn readonly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn parameter(&self) -> SyntaxResult<TsIndexSignatureParameter> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn type_annotation(&self) -> SyntaxResult<TsTypeAnnotation> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn separator_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIndexedAccessType {
    pub(crate) syntax: SyntaxNode,
}
impl TsIndexedAccessType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn object_type(&self) -> SyntaxResult<TsType> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn index_type(&self) -> SyntaxResult<TsType> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsInferType {
    pub(crate) syntax: SyntaxNode,
}
impl TsInferType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn infer_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_parameter(&self) -> SyntaxResult<TsTypeParameterName> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsInterfaceDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl TsInterfaceDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn interface_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn id(&self) -> SyntaxResult<TsIdentifierBinding> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 2usize)
    }
    pub fn extends_clause(&self) -> Option<TsExtendsClause> { support::node(&self.syntax, 3usize) }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn members(&self) -> TsTypeMemberList { support::list(&self.syntax, 5usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIntersectionType {
    pub(crate) syntax: SyntaxNode,
}
impl TsIntersectionType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn leading_separator_token_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn types(&self) -> TsIntersectionTypeElementList { support::list(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedType {
    pub(crate) syntax: SyntaxNode,
}
impl TsMappedType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn readonly_modifier(&self) -> Option<TsMappedTypeReadonlyModifierClause> {
        support::node(&self.syntax, 1usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn property_name(&self) -> SyntaxResult<TsTypeParameterName> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn keys_type(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 5usize) }
    pub fn as_clause(&self) -> Option<TsMappedTypeAsClause> { support::node(&self.syntax, 6usize) }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
    pub fn optional_modifier(&self) -> Option<TsMappedTypeOptionalModifierClause> {
        support::node(&self.syntax, 8usize)
    }
    pub fn mapped_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax, 9usize) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 10usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 11usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeAsClause {
    pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeAsClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeOptionalModifierClause {
    pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeOptionalModifierClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeReadonlyModifierClause {
    pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeReadonlyModifierClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn readonly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMethodSignatureTypeMember {
    pub(crate) syntax: SyntaxNode,
}
impl TsMethodSignatureTypeMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn optional_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 1usize) }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 2usize)
    }
    pub fn parameters(&self) -> SyntaxResult<JsParameters> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn return_type_annotation(&self) -> Option<TsReturnTypeAnnotation> {
        support::node(&self.syntax, 4usize)
    }
    pub fn separator_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsModuleBlock {
    pub(crate) syntax: SyntaxNode,
}
impl TsModuleBlock {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> JsModuleItemList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsModuleDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl TsModuleDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn module_or_namespace(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<TsAnyModuleName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn body(&self) -> SyntaxResult<TsModuleBlock> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNameWithTypeArguments {
    pub(crate) syntax: SyntaxNode,
}
impl TsNameWithTypeArguments {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<TsAnyName> { support::required_node(&self.syntax, 0usize) }
    pub fn type_arguments(&self) -> Option<TsTypeArguments> { support::node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNamedTupleTypeElement {
    pub(crate) syntax: SyntaxNode,
}
impl TsNamedTupleTypeElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn name(&self) -> SyntaxResult<JsName> { support::required_node(&self.syntax, 1usize) }
    pub fn question_mark_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 4usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNeverType {
    pub(crate) syntax: SyntaxNode,
}
impl TsNeverType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn never_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNonNullAssertionAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl TsNonNullAssertionAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn assignment(&self) -> SyntaxResult<JsAnyAssignment> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNonNullAssertionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl TsNonNullAssertionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNonPrimitiveType {
    pub(crate) syntax: SyntaxNode,
}
impl TsNonPrimitiveType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn object_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNullLiteralType {
    pub(crate) syntax: SyntaxNode,
}
impl TsNullLiteralType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNumberLiteralType {
    pub(crate) syntax: SyntaxNode,
}
impl TsNumberLiteralType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn minus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNumberType {
    pub(crate) syntax: SyntaxNode,
}
impl TsNumberType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn number_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsObjectType {
    pub(crate) syntax: SyntaxNode,
}
impl TsObjectType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn members(&self) -> TsTypeMemberList { support::list(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsOptionalPropertyAnnotation {
    pub(crate) syntax: SyntaxNode,
}
impl TsOptionalPropertyAnnotation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        support::node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsOptionalTupleTypeElement {
    pub(crate) syntax: SyntaxNode,
}
impl TsOptionalTupleTypeElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 0usize) }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsParenthesizedType {
    pub(crate) syntax: SyntaxNode,
}
impl TsParenthesizedType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsPropertyParameter {
    pub(crate) syntax: SyntaxNode,
}
impl TsPropertyParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn accessibility(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn formal_parameter(&self) -> SyntaxResult<JsAnyFormalParameter> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsPropertySignatureTypeMember {
    pub(crate) syntax: SyntaxNode,
}
impl TsPropertySignatureTypeMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn readonly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn optional_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 2usize) }
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        support::node(&self.syntax, 3usize)
    }
    pub fn separator_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 4usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsQualifiedModuleName {
    pub(crate) syntax: SyntaxNode,
}
impl TsQualifiedModuleName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn left(&self) -> SyntaxResult<TsAnyModuleName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<JsName> { support::required_node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsQualifiedName {
    pub(crate) syntax: SyntaxNode,
}
impl TsQualifiedName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn left(&self) -> SyntaxResult<TsAnyName> { support::required_node(&self.syntax, 0usize) }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<JsName> { support::required_node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsReadonlyPropertyParameter {
    pub(crate) syntax: SyntaxNode,
}
impl TsReadonlyPropertyParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn accessibility(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn readonly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn formal_parameter(&self) -> SyntaxResult<JsAnyFormalParameter> {
        support::required_node(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsReferenceType {
    pub(crate) syntax: SyntaxNode,
}
impl TsReferenceType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<TsAnyName> { support::required_node(&self.syntax, 0usize) }
    pub fn type_arguments(&self) -> Option<TsTypeArguments> { support::node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsRestTupleTypeElement {
    pub(crate) syntax: SyntaxNode,
}
impl TsRestTupleTypeElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsReturnTypeAnnotation {
    pub(crate) syntax: SyntaxNode,
}
impl TsReturnTypeAnnotation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsAnyReturnType> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsSetterSignatureTypeMember {
    pub(crate) syntax: SyntaxNode,
}
impl TsSetterSignatureTypeMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn parameter(&self) -> SyntaxResult<JsAnyFormalParameter> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn separator_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsStringLiteralType {
    pub(crate) syntax: SyntaxNode,
}
impl TsStringLiteralType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn literal_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsStringType {
    pub(crate) syntax: SyntaxNode,
}
impl TsStringType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn string_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsSymbolType {
    pub(crate) syntax: SyntaxNode,
}
impl TsSymbolType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn symbol_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTemplateChunkElement {
    pub(crate) syntax: SyntaxNode,
}
impl TsTemplateChunkElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn template_chunk_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTemplateElement {
    pub(crate) syntax: SyntaxNode,
}
impl TsTemplateElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn dollar_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTemplateLiteralType {
    pub(crate) syntax: SyntaxNode,
}
impl TsTemplateLiteralType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_tick_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> TsTemplateElementList { support::list(&self.syntax, 1usize) }
    pub fn r_tick_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsThisParameter {
    pub(crate) syntax: SyntaxNode,
}
impl TsThisParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn type_annotation(&self) -> Option<TsTypeAnnotation> {
        support::node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsThisType {
    pub(crate) syntax: SyntaxNode,
}
impl TsThisType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTupleType {
    pub(crate) syntax: SyntaxNode,
}
impl TsTupleType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn elements(&self) -> TsTupleTypeElementList { support::list(&self.syntax, 1usize) }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeAliasDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeAliasDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn type_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn binding_identifier(&self) -> SyntaxResult<TsIdentifierBinding> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn type_parameters(&self) -> Option<TsTypeParameters> {
        support::node(&self.syntax, 2usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 4usize) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 5usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeAnnotation {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeAnnotation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeArguments {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeArguments {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ts_type_argument_list(&self) -> TsTypeArgumentList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeAssertionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeAssertionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
        support::required_node(&self.syntax, 3usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeConstraintClause {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeConstraintClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeOperatorType {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeOperatorType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParameter {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn name(&self) -> SyntaxResult<TsTypeParameterName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn constraint(&self) -> Option<TsTypeConstraintClause> {
        support::node(&self.syntax, 1usize)
    }
    pub fn default(&self) -> Option<TsDefaultTypeClause> { support::node(&self.syntax, 2usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParameterName {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeParameterName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParameters {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeParameters {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> TsTypeParameterList { support::list(&self.syntax, 1usize) }
    pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypePredicate {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypePredicate {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn asserts_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, 0usize) }
    pub fn parameter_name(&self) -> SyntaxResult<TsAnyTypePredicateParameterName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn is_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax, 3usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeofType {
    pub(crate) syntax: SyntaxNode,
}
impl TsTypeofType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn typeof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression_name(&self) -> SyntaxResult<TsAnyName> {
        support::required_node(&self.syntax, 1usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUndefinedType {
    pub(crate) syntax: SyntaxNode,
}
impl TsUndefinedType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn undefined_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUnionType {
    pub(crate) syntax: SyntaxNode,
}
impl TsUnionType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn leading_separator_token_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn types(&self) -> TsUnionTypeVariantList { support::list(&self.syntax, 1usize) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUnknownType {
    pub(crate) syntax: SyntaxNode,
}
impl TsUnknownType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn unknown_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsVoidType {
    pub(crate) syntax: SyntaxNode,
}
impl TsVoidType {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn void_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrayAssignmentPatternElement {
    JsAnyAssignmentPattern(JsAnyAssignmentPattern),
    JsArrayAssignmentPatternRestElement(JsArrayAssignmentPatternRestElement),
    JsArrayHole(JsArrayHole),
    JsAssignmentWithDefault(JsAssignmentWithDefault),
    JsUnknownAssignment(JsUnknownAssignment),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrayBindingPatternElement {
    JsAnyBindingPattern(JsAnyBindingPattern),
    JsArrayBindingPatternRestElement(JsArrayBindingPatternRestElement),
    JsArrayHole(JsArrayHole),
    JsBindingPatternWithDefault(JsBindingPatternWithDefault),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrayElement {
    JsAnyExpression(JsAnyExpression),
    JsArrayHole(JsArrayHole),
    JsSpread(JsSpread),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyArrowFunctionParameters {
    JsAnyBinding(JsAnyBinding),
    JsParameters(JsParameters),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyAssignment {
    JsComputedMemberAssignment(JsComputedMemberAssignment),
    JsIdentifierAssignment(JsIdentifierAssignment),
    JsParenthesizedAssignment(JsParenthesizedAssignment),
    JsStaticMemberAssignment(JsStaticMemberAssignment),
    JsUnknownAssignment(JsUnknownAssignment),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyAssignmentPattern {
    JsAnyAssignment(JsAnyAssignment),
    JsArrayAssignmentPattern(JsArrayAssignmentPattern),
    JsObjectAssignmentPattern(JsObjectAssignmentPattern),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyBinding {
    JsIdentifierBinding(JsIdentifierBinding),
    JsUnknownBinding(JsUnknownBinding),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyBindingPattern {
    JsAnyBinding(JsAnyBinding),
    JsArrayBindingPattern(JsArrayBindingPattern),
    JsObjectBindingPattern(JsObjectBindingPattern),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyClass {
    JsClassDeclaration(JsClassDeclaration),
    JsClassExpression(JsClassExpression),
    JsExportDefaultClassClause(JsExportDefaultClassClause),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyClassMember {
    JsConstructorClassMember(JsConstructorClassMember),
    JsEmptyClassMember(JsEmptyClassMember),
    JsGetterClassMember(JsGetterClassMember),
    JsMethodClassMember(JsMethodClassMember),
    JsPropertyClassMember(JsPropertyClassMember),
    JsSetterClassMember(JsSetterClassMember),
    JsStaticInitializationBlockClassMember(JsStaticInitializationBlockClassMember),
    JsUnknownMember(JsUnknownMember),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyClassMemberName {
    JsComputedMemberName(JsComputedMemberName),
    JsLiteralMemberName(JsLiteralMemberName),
    JsPrivateClassMemberName(JsPrivateClassMemberName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyConstructorParameter {
    JsAnyFormalParameter(JsAnyFormalParameter),
    JsRestParameter(JsRestParameter),
    TsPropertyParameter(TsPropertyParameter),
    TsReadonlyPropertyParameter(TsReadonlyPropertyParameter),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyDeclaration {
    JsClassDeclaration(JsClassDeclaration),
    JsFunctionDeclaration(JsFunctionDeclaration),
    JsVariableDeclaration(JsVariableDeclaration),
    TsDeclareFunctionDeclaration(TsDeclareFunctionDeclaration),
    TsEnumDeclaration(TsEnumDeclaration),
    TsExternalModuleDeclaration(TsExternalModuleDeclaration),
    TsGlobalDeclaration(TsGlobalDeclaration),
    TsInterfaceDeclaration(TsInterfaceDeclaration),
    TsModuleDeclaration(TsModuleDeclaration),
    TsTypeAliasDeclaration(TsTypeAliasDeclaration),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyDeclarationClause {
    JsClassDeclaration(JsClassDeclaration),
    JsFunctionDeclaration(JsFunctionDeclaration),
    JsVariableDeclarationClause(JsVariableDeclarationClause),
    TsDeclareFunctionDeclaration(TsDeclareFunctionDeclaration),
    TsEnumDeclaration(TsEnumDeclaration),
    TsExternalModuleDeclaration(TsExternalModuleDeclaration),
    TsGlobalDeclaration(TsGlobalDeclaration),
    TsInterfaceDeclaration(TsInterfaceDeclaration),
    TsModuleDeclaration(TsModuleDeclaration),
    TsTypeAliasDeclaration(TsTypeAliasDeclaration),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyExportClause {
    JsAnyDeclarationClause(JsAnyDeclarationClause),
    JsExportDefaultClassClause(JsExportDefaultClassClause),
    JsExportDefaultExpressionClause(JsExportDefaultExpressionClause),
    JsExportDefaultFunctionClause(JsExportDefaultFunctionClause),
    JsExportFromClause(JsExportFromClause),
    JsExportNamedClause(JsExportNamedClause),
    JsExportNamedFromClause(JsExportNamedFromClause),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyExportNamedSpecifier {
    JsExportNamedShorthandSpecifier(JsExportNamedShorthandSpecifier),
    JsExportNamedSpecifier(JsExportNamedSpecifier),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyExpression {
    ImportMeta(ImportMeta),
    JsAnyLiteralExpression(JsAnyLiteralExpression),
    JsArrayExpression(JsArrayExpression),
    JsArrowFunctionExpression(JsArrowFunctionExpression),
    JsAssignmentExpression(JsAssignmentExpression),
    JsAwaitExpression(JsAwaitExpression),
    JsBinaryExpression(JsBinaryExpression),
    JsCallExpression(JsCallExpression),
    JsClassExpression(JsClassExpression),
    JsComputedMemberExpression(JsComputedMemberExpression),
    JsConditionalExpression(JsConditionalExpression),
    JsFunctionExpression(JsFunctionExpression),
    JsIdentifierExpression(JsIdentifierExpression),
    JsImportCallExpression(JsImportCallExpression),
    JsInExpression(JsInExpression),
    JsInstanceofExpression(JsInstanceofExpression),
    JsLogicalExpression(JsLogicalExpression),
    JsNewExpression(JsNewExpression),
    JsObjectExpression(JsObjectExpression),
    JsParenthesizedExpression(JsParenthesizedExpression),
    JsPostUpdateExpression(JsPostUpdateExpression),
    JsPreUpdateExpression(JsPreUpdateExpression),
    JsSequenceExpression(JsSequenceExpression),
    JsStaticMemberExpression(JsStaticMemberExpression),
    JsSuperExpression(JsSuperExpression),
    JsTemplate(JsTemplate),
    JsThisExpression(JsThisExpression),
    JsUnaryExpression(JsUnaryExpression),
    JsUnknownExpression(JsUnknownExpression),
    JsYieldExpression(JsYieldExpression),
    NewTarget(NewTarget),
    TsAsExpression(TsAsExpression),
    TsNonNullAssertionExpression(TsNonNullAssertionExpression),
    TsTypeAssertionExpression(TsTypeAssertionExpression),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyForInOrOfInitializer {
    JsAnyAssignmentPattern(JsAnyAssignmentPattern),
    JsForVariableDeclaration(JsForVariableDeclaration),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyForInitializer {
    JsAnyExpression(JsAnyExpression),
    JsVariableDeclaration(JsVariableDeclaration),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyFormalParameter {
    JsFormalParameter(JsFormalParameter),
    JsUnknownParameter(JsUnknownParameter),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyFunction {
    JsArrowFunctionExpression(JsArrowFunctionExpression),
    JsExportDefaultFunctionClause(JsExportDefaultFunctionClause),
    JsFunctionDeclaration(JsFunctionDeclaration),
    JsFunctionExpression(JsFunctionExpression),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyFunctionBody {
    JsAnyExpression(JsAnyExpression),
    JsFunctionBody(JsFunctionBody),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyImportAssertionEntry {
    JsImportAssertionEntry(JsImportAssertionEntry),
    JsUnknownImportAssertionEntry(JsUnknownImportAssertionEntry),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyImportClause {
    JsImportBareClause(JsImportBareClause),
    JsImportDefaultClause(JsImportDefaultClause),
    JsImportNamedClause(JsImportNamedClause),
    JsImportNamespaceClause(JsImportNamespaceClause),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyInProperty {
    JsAnyExpression(JsAnyExpression),
    JsPrivateName(JsPrivateName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyLiteralExpression {
    JsBigIntLiteralExpression(JsBigIntLiteralExpression),
    JsBooleanLiteralExpression(JsBooleanLiteralExpression),
    JsNullLiteralExpression(JsNullLiteralExpression),
    JsNumberLiteralExpression(JsNumberLiteralExpression),
    JsRegexLiteralExpression(JsRegexLiteralExpression),
    JsStringLiteralExpression(JsStringLiteralExpression),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyModuleItem {
    JsAnyStatement(JsAnyStatement),
    JsExport(JsExport),
    JsImport(JsImport),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyName {
    JsName(JsName),
    JsPrivateName(JsPrivateName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyNamedImport {
    JsNamedImportSpecifiers(JsNamedImportSpecifiers),
    JsNamespaceImportSpecifier(JsNamespaceImportSpecifier),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyNamedImportSpecifier {
    JsNamedImportSpecifier(JsNamedImportSpecifier),
    JsShorthandNamedImportSpecifier(JsShorthandNamedImportSpecifier),
    JsUnknownNamedImportSpecifier(JsUnknownNamedImportSpecifier),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyObjectAssignmentPatternMember {
    JsObjectAssignmentPatternProperty(JsObjectAssignmentPatternProperty),
    JsObjectAssignmentPatternRest(JsObjectAssignmentPatternRest),
    JsObjectAssignmentPatternShorthandProperty(JsObjectAssignmentPatternShorthandProperty),
    JsUnknownAssignment(JsUnknownAssignment),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyObjectBindingPatternMember {
    JsIdentifierBinding(JsIdentifierBinding),
    JsObjectBindingPatternProperty(JsObjectBindingPatternProperty),
    JsObjectBindingPatternRest(JsObjectBindingPatternRest),
    JsObjectBindingPatternShorthandProperty(JsObjectBindingPatternShorthandProperty),
    JsUnknownBinding(JsUnknownBinding),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyObjectMember {
    JsGetterObjectMember(JsGetterObjectMember),
    JsMethodObjectMember(JsMethodObjectMember),
    JsPropertyObjectMember(JsPropertyObjectMember),
    JsSetterObjectMember(JsSetterObjectMember),
    JsShorthandPropertyObjectMember(JsShorthandPropertyObjectMember),
    JsSpread(JsSpread),
    JsUnknownMember(JsUnknownMember),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyObjectMemberName {
    JsComputedMemberName(JsComputedMemberName),
    JsLiteralMemberName(JsLiteralMemberName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyParameter {
    JsAnyFormalParameter(JsAnyFormalParameter),
    JsRestParameter(JsRestParameter),
    TsThisParameter(TsThisParameter),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyRoot {
    JsExpressionSnipped(JsExpressionSnipped),
    JsModule(JsModule),
    JsScript(JsScript),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyStatement {
    JsBlockStatement(JsBlockStatement),
    JsBreakStatement(JsBreakStatement),
    JsClassDeclaration(JsClassDeclaration),
    JsContinueStatement(JsContinueStatement),
    JsDebuggerStatement(JsDebuggerStatement),
    JsDoWhileStatement(JsDoWhileStatement),
    JsEmptyStatement(JsEmptyStatement),
    JsExpressionStatement(JsExpressionStatement),
    JsForInStatement(JsForInStatement),
    JsForOfStatement(JsForOfStatement),
    JsForStatement(JsForStatement),
    JsFunctionDeclaration(JsFunctionDeclaration),
    JsIfStatement(JsIfStatement),
    JsLabeledStatement(JsLabeledStatement),
    JsReturnStatement(JsReturnStatement),
    JsSwitchStatement(JsSwitchStatement),
    JsThrowStatement(JsThrowStatement),
    JsTryFinallyStatement(JsTryFinallyStatement),
    JsTryStatement(JsTryStatement),
    JsUnknownStatement(JsUnknownStatement),
    JsVariableStatement(JsVariableStatement),
    JsWhileStatement(JsWhileStatement),
    JsWithStatement(JsWithStatement),
    TsDeclareFunctionDeclaration(TsDeclareFunctionDeclaration),
    TsDeclareStatement(TsDeclareStatement),
    TsEnumDeclaration(TsEnumDeclaration),
    TsExternalModuleDeclaration(TsExternalModuleDeclaration),
    TsGlobalDeclaration(TsGlobalDeclaration),
    TsInterfaceDeclaration(TsInterfaceDeclaration),
    TsModuleDeclaration(TsModuleDeclaration),
    TsTypeAliasDeclaration(TsTypeAliasDeclaration),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnySwitchClause {
    JsCaseClause(JsCaseClause),
    JsDefaultClause(JsDefaultClause),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyTemplateElement {
    JsTemplateChunkElement(JsTemplateChunkElement),
    JsTemplateElement(JsTemplateElement),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyExternalModuleDeclarationBody {
    TsEmptyExternalModuleDeclarationBody(TsEmptyExternalModuleDeclarationBody),
    TsModuleBlock(TsModuleBlock),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyModuleName {
    TsIdentifierBinding(TsIdentifierBinding),
    TsQualifiedModuleName(TsQualifiedModuleName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyName {
    JsReferenceIdentifier(JsReferenceIdentifier),
    TsQualifiedName(TsQualifiedName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyPropertyAnnotation {
    TsDefinitePropertyAnnotation(TsDefinitePropertyAnnotation),
    TsOptionalPropertyAnnotation(TsOptionalPropertyAnnotation),
    TsTypeAnnotation(TsTypeAnnotation),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyPropertyParameter {
    TsPropertyParameter(TsPropertyParameter),
    TsReadonlyPropertyParameter(TsReadonlyPropertyParameter),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyReturnType {
    TsType(TsType),
    TsTypePredicate(TsTypePredicate),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyTemplateElement {
    TsTemplateChunkElement(TsTemplateChunkElement),
    TsTemplateElement(TsTemplateElement),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyTupleTypeElement {
    TsNamedTupleTypeElement(TsNamedTupleTypeElement),
    TsOptionalTupleTypeElement(TsOptionalTupleTypeElement),
    TsRestTupleTypeElement(TsRestTupleTypeElement),
    TsType(TsType),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyTypeMember {
    TsCallSignatureTypeMember(TsCallSignatureTypeMember),
    TsConstructSignatureTypeMember(TsConstructSignatureTypeMember),
    TsGetterSignatureTypeMember(TsGetterSignatureTypeMember),
    TsIndexSignatureTypeMember(TsIndexSignatureTypeMember),
    TsMethodSignatureTypeMember(TsMethodSignatureTypeMember),
    TsPropertySignatureTypeMember(TsPropertySignatureTypeMember),
    TsSetterSignatureTypeMember(TsSetterSignatureTypeMember),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyTypePredicateParameterName {
    JsReferenceIdentifier(JsReferenceIdentifier),
    TsThisType(TsThisType),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsAnyVariableAnnotation {
    TsDefiniteVariableAnnotation(TsDefiniteVariableAnnotation),
    TsTypeAnnotation(TsTypeAnnotation),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsModuleRef {
    TsAnyName(TsAnyName),
    TsExternalModuleRef(TsExternalModuleRef),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsType {
    TsAnyType(TsAnyType),
    TsArrayType(TsArrayType),
    TsBigIntLiteralType(TsBigIntLiteralType),
    TsBigintType(TsBigintType),
    TsBooleanLiteralType(TsBooleanLiteralType),
    TsBooleanType(TsBooleanType),
    TsConditionalType(TsConditionalType),
    TsConstructorType(TsConstructorType),
    TsFunctionType(TsFunctionType),
    TsImportType(TsImportType),
    TsIndexedAccessType(TsIndexedAccessType),
    TsInferType(TsInferType),
    TsIntersectionType(TsIntersectionType),
    TsMappedType(TsMappedType),
    TsNeverType(TsNeverType),
    TsNonPrimitiveType(TsNonPrimitiveType),
    TsNullLiteralType(TsNullLiteralType),
    TsNumberLiteralType(TsNumberLiteralType),
    TsNumberType(TsNumberType),
    TsObjectType(TsObjectType),
    TsParenthesizedType(TsParenthesizedType),
    TsReferenceType(TsReferenceType),
    TsStringLiteralType(TsStringLiteralType),
    TsStringType(TsStringType),
    TsSymbolType(TsSymbolType),
    TsTemplateLiteralType(TsTemplateLiteralType),
    TsThisType(TsThisType),
    TsTupleType(TsTupleType),
    TsTypeOperatorType(TsTypeOperatorType),
    TsTypeofType(TsTypeofType),
    TsUndefinedType(TsUndefinedType),
    TsUnionType(TsUnionType),
    TsUnknownType(TsUnknownType),
    TsVoidType(TsVoidType),
}
impl AstNode for ImportMeta {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == IMPORT_META }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ImportMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImportMeta")
            .field(
                "import_token",
                &support::DebugSyntaxResult(self.import_token()),
            )
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("meta_token", &support::DebugSyntaxResult(self.meta_token()))
            .finish()
    }
}
impl From<ImportMeta> for SyntaxNode {
    fn from(n: ImportMeta) -> SyntaxNode { n.syntax }
}
impl From<ImportMeta> for SyntaxElement {
    fn from(n: ImportMeta) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsArrayAssignmentPattern {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_PATTERN }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayAssignmentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsArrayAssignmentPattern")
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
impl From<JsArrayAssignmentPattern> for SyntaxNode {
    fn from(n: JsArrayAssignmentPattern) -> SyntaxNode { n.syntax }
}
impl From<JsArrayAssignmentPattern> for SyntaxElement {
    fn from(n: JsArrayAssignmentPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsArrayAssignmentPatternRestElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayAssignmentPatternRestElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsArrayAssignmentPatternRestElement")
            .field(
                "dotdotdot_token",
                &support::DebugSyntaxResult(self.dotdotdot_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<JsArrayAssignmentPatternRestElement> for SyntaxNode {
    fn from(n: JsArrayAssignmentPatternRestElement) -> SyntaxNode { n.syntax }
}
impl From<JsArrayAssignmentPatternRestElement> for SyntaxElement {
    fn from(n: JsArrayAssignmentPatternRestElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsArrayBindingPattern {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_BINDING_PATTERN }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayBindingPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsArrayBindingPattern")
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
impl From<JsArrayBindingPattern> for SyntaxNode {
    fn from(n: JsArrayBindingPattern) -> SyntaxNode { n.syntax }
}
impl From<JsArrayBindingPattern> for SyntaxElement {
    fn from(n: JsArrayBindingPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsArrayBindingPatternRestElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_BINDING_PATTERN_REST_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayBindingPatternRestElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsArrayBindingPatternRestElement")
            .field(
                "dotdotdot_token",
                &support::DebugSyntaxResult(self.dotdotdot_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .finish()
    }
}
impl From<JsArrayBindingPatternRestElement> for SyntaxNode {
    fn from(n: JsArrayBindingPatternRestElement) -> SyntaxNode { n.syntax }
}
impl From<JsArrayBindingPatternRestElement> for SyntaxElement {
    fn from(n: JsArrayBindingPatternRestElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsArrayExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsArrayExpression")
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
impl From<JsArrayExpression> for SyntaxNode {
    fn from(n: JsArrayExpression) -> SyntaxNode { n.syntax }
}
impl From<JsArrayExpression> for SyntaxElement {
    fn from(n: JsArrayExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsArrayHole {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_HOLE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrayHole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsArrayHole").finish()
    }
}
impl From<JsArrayHole> for SyntaxNode {
    fn from(n: JsArrayHole) -> SyntaxNode { n.syntax }
}
impl From<JsArrayHole> for SyntaxElement {
    fn from(n: JsArrayHole) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsArrowFunctionExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARROW_FUNCTION_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsArrowFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsArrowFunctionExpression")
            .field(
                "async_token",
                &support::DebugOptionalElement(self.async_token()),
            )
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field(
                "fat_arrow_token",
                &support::DebugSyntaxResult(self.fat_arrow_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsArrowFunctionExpression> for SyntaxNode {
    fn from(n: JsArrowFunctionExpression) -> SyntaxNode { n.syntax }
}
impl From<JsArrowFunctionExpression> for SyntaxElement {
    fn from(n: JsArrowFunctionExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsAssignmentExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ASSIGNMENT_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsAssignmentExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsAssignmentExpression")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "operator_token",
                &support::DebugSyntaxResult(self.operator_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<JsAssignmentExpression> for SyntaxNode {
    fn from(n: JsAssignmentExpression) -> SyntaxNode { n.syntax }
}
impl From<JsAssignmentExpression> for SyntaxElement {
    fn from(n: JsAssignmentExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsAssignmentWithDefault {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ASSIGNMENT_WITH_DEFAULT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsAssignmentWithDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsAssignmentWithDefault")
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("default", &support::DebugSyntaxResult(self.default()))
            .finish()
    }
}
impl From<JsAssignmentWithDefault> for SyntaxNode {
    fn from(n: JsAssignmentWithDefault) -> SyntaxNode { n.syntax }
}
impl From<JsAssignmentWithDefault> for SyntaxElement {
    fn from(n: JsAssignmentWithDefault) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsAwaitExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_AWAIT_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsAwaitExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsAwaitExpression")
            .field(
                "await_token",
                &support::DebugSyntaxResult(self.await_token()),
            )
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .finish()
    }
}
impl From<JsAwaitExpression> for SyntaxNode {
    fn from(n: JsAwaitExpression) -> SyntaxNode { n.syntax }
}
impl From<JsAwaitExpression> for SyntaxElement {
    fn from(n: JsAwaitExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsBigIntLiteralExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_BIG_INT_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBigIntLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsBigIntLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsBigIntLiteralExpression> for SyntaxNode {
    fn from(n: JsBigIntLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsBigIntLiteralExpression> for SyntaxElement {
    fn from(n: JsBigIntLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsBinaryExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_BINARY_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsBinaryExpression")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<JsBinaryExpression> for SyntaxNode {
    fn from(n: JsBinaryExpression) -> SyntaxNode { n.syntax }
}
impl From<JsBinaryExpression> for SyntaxElement {
    fn from(n: JsBinaryExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsBindingPatternWithDefault {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_BINDING_PATTERN_WITH_DEFAULT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBindingPatternWithDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsBindingPatternWithDefault")
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("default", &support::DebugSyntaxResult(self.default()))
            .finish()
    }
}
impl From<JsBindingPatternWithDefault> for SyntaxNode {
    fn from(n: JsBindingPatternWithDefault) -> SyntaxNode { n.syntax }
}
impl From<JsBindingPatternWithDefault> for SyntaxElement {
    fn from(n: JsBindingPatternWithDefault) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsBlockStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_BLOCK_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsBlockStatement")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("statements", &self.statements())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsBlockStatement> for SyntaxNode {
    fn from(n: JsBlockStatement) -> SyntaxNode { n.syntax }
}
impl From<JsBlockStatement> for SyntaxElement {
    fn from(n: JsBlockStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsBooleanLiteralExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_BOOLEAN_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsBooleanLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsBooleanLiteralExpression> for SyntaxNode {
    fn from(n: JsBooleanLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsBooleanLiteralExpression> for SyntaxElement {
    fn from(n: JsBooleanLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsBreakStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_BREAK_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsBreakStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsBreakStatement")
            .field(
                "break_token",
                &support::DebugSyntaxResult(self.break_token()),
            )
            .field(
                "label_token",
                &support::DebugOptionalElement(self.label_token()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsBreakStatement> for SyntaxNode {
    fn from(n: JsBreakStatement) -> SyntaxNode { n.syntax }
}
impl From<JsBreakStatement> for SyntaxElement {
    fn from(n: JsBreakStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsCallArguments {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CALL_ARGUMENTS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsCallArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsCallArguments")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("args", &self.args())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<JsCallArguments> for SyntaxNode {
    fn from(n: JsCallArguments) -> SyntaxNode { n.syntax }
}
impl From<JsCallArguments> for SyntaxElement {
    fn from(n: JsCallArguments) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsCallExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CALL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsCallExpression")
            .field("callee", &support::DebugSyntaxResult(self.callee()))
            .field(
                "optional_chain_token_token",
                &support::DebugOptionalElement(self.optional_chain_token_token()),
            )
            .field(
                "type_arguments",
                &support::DebugOptionalElement(self.type_arguments()),
            )
            .field("arguments", &support::DebugSyntaxResult(self.arguments()))
            .finish()
    }
}
impl From<JsCallExpression> for SyntaxNode {
    fn from(n: JsCallExpression) -> SyntaxNode { n.syntax }
}
impl From<JsCallExpression> for SyntaxElement {
    fn from(n: JsCallExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsCaseClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CASE_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsCaseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsCaseClause")
            .field("case_token", &support::DebugSyntaxResult(self.case_token()))
            .field("test", &support::DebugSyntaxResult(self.test()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("consequent", &self.consequent())
            .finish()
    }
}
impl From<JsCaseClause> for SyntaxNode {
    fn from(n: JsCaseClause) -> SyntaxNode { n.syntax }
}
impl From<JsCaseClause> for SyntaxElement {
    fn from(n: JsCaseClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsCatchClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CATCH_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsCatchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsCatchClause")
            .field(
                "catch_token",
                &support::DebugSyntaxResult(self.catch_token()),
            )
            .field(
                "declaration",
                &support::DebugOptionalElement(self.declaration()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsCatchClause> for SyntaxNode {
    fn from(n: JsCatchClause) -> SyntaxNode { n.syntax }
}
impl From<JsCatchClause> for SyntaxElement {
    fn from(n: JsCatchClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsCatchDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CATCH_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsCatchDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsCatchDeclaration")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("binding", &support::DebugSyntaxResult(self.binding()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<JsCatchDeclaration> for SyntaxNode {
    fn from(n: JsCatchDeclaration) -> SyntaxNode { n.syntax }
}
impl From<JsCatchDeclaration> for SyntaxElement {
    fn from(n: JsCatchDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsClassDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CLASS_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsClassDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsClassDeclaration")
            .field(
                "class_token",
                &support::DebugSyntaxResult(self.class_token()),
            )
            .field("id", &support::DebugSyntaxResult(self.id()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field(
                "extends_clause",
                &support::DebugOptionalElement(self.extends_clause()),
            )
            .field(
                "implements_clause",
                &support::DebugOptionalElement(self.implements_clause()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("members", &self.members())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsClassDeclaration> for SyntaxNode {
    fn from(n: JsClassDeclaration) -> SyntaxNode { n.syntax }
}
impl From<JsClassDeclaration> for SyntaxElement {
    fn from(n: JsClassDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsClassExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CLASS_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsClassExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsClassExpression")
            .field(
                "class_token",
                &support::DebugSyntaxResult(self.class_token()),
            )
            .field("id", &support::DebugOptionalElement(self.id()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field(
                "extends_clause",
                &support::DebugOptionalElement(self.extends_clause()),
            )
            .field(
                "implements_clause",
                &support::DebugOptionalElement(self.implements_clause()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("members", &self.members())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsClassExpression> for SyntaxNode {
    fn from(n: JsClassExpression) -> SyntaxNode { n.syntax }
}
impl From<JsClassExpression> for SyntaxElement {
    fn from(n: JsClassExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsComputedMemberAssignment {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_ASSIGNMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsComputedMemberAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsComputedMemberAssignment")
            .field("object", &support::DebugSyntaxResult(self.object()))
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("member", &support::DebugSyntaxResult(self.member()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<JsComputedMemberAssignment> for SyntaxNode {
    fn from(n: JsComputedMemberAssignment) -> SyntaxNode { n.syntax }
}
impl From<JsComputedMemberAssignment> for SyntaxElement {
    fn from(n: JsComputedMemberAssignment) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsComputedMemberExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsComputedMemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsComputedMemberExpression")
            .field("object", &support::DebugSyntaxResult(self.object()))
            .field(
                "optional_chain_token",
                &support::DebugOptionalElement(self.optional_chain_token()),
            )
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("member", &support::DebugSyntaxResult(self.member()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<JsComputedMemberExpression> for SyntaxNode {
    fn from(n: JsComputedMemberExpression) -> SyntaxNode { n.syntax }
}
impl From<JsComputedMemberExpression> for SyntaxElement {
    fn from(n: JsComputedMemberExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsComputedMemberName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsComputedMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsComputedMemberName")
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<JsComputedMemberName> for SyntaxNode {
    fn from(n: JsComputedMemberName) -> SyntaxNode { n.syntax }
}
impl From<JsComputedMemberName> for SyntaxElement {
    fn from(n: JsComputedMemberName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsConditionalExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CONDITIONAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsConditionalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsConditionalExpression")
            .field("test", &support::DebugSyntaxResult(self.test()))
            .field(
                "question_mark_token",
                &support::DebugSyntaxResult(self.question_mark_token()),
            )
            .field("consequent", &support::DebugSyntaxResult(self.consequent()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("alternate", &support::DebugSyntaxResult(self.alternate()))
            .finish()
    }
}
impl From<JsConditionalExpression> for SyntaxNode {
    fn from(n: JsConditionalExpression) -> SyntaxNode { n.syntax }
}
impl From<JsConditionalExpression> for SyntaxElement {
    fn from(n: JsConditionalExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsConstructorClassMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CONSTRUCTOR_CLASS_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsConstructorClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsConstructorClassMember")
            .field(
                "access_modifier",
                &support::DebugOptionalElement(self.access_modifier()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsConstructorClassMember> for SyntaxNode {
    fn from(n: JsConstructorClassMember) -> SyntaxNode { n.syntax }
}
impl From<JsConstructorClassMember> for SyntaxElement {
    fn from(n: JsConstructorClassMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsConstructorParameters {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CONSTRUCTOR_PARAMETERS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsConstructorParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsConstructorParameters")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("parameters", &self.parameters())
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<JsConstructorParameters> for SyntaxNode {
    fn from(n: JsConstructorParameters) -> SyntaxNode { n.syntax }
}
impl From<JsConstructorParameters> for SyntaxElement {
    fn from(n: JsConstructorParameters) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsContinueStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CONTINUE_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsContinueStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsContinueStatement")
            .field(
                "continue_token",
                &support::DebugSyntaxResult(self.continue_token()),
            )
            .field(
                "label_token",
                &support::DebugOptionalElement(self.label_token()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsContinueStatement> for SyntaxNode {
    fn from(n: JsContinueStatement) -> SyntaxNode { n.syntax }
}
impl From<JsContinueStatement> for SyntaxElement {
    fn from(n: JsContinueStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsDebuggerStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_DEBUGGER_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDebuggerStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsDebuggerStatement")
            .field(
                "debugger_token",
                &support::DebugSyntaxResult(self.debugger_token()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsDebuggerStatement> for SyntaxNode {
    fn from(n: JsDebuggerStatement) -> SyntaxNode { n.syntax }
}
impl From<JsDebuggerStatement> for SyntaxElement {
    fn from(n: JsDebuggerStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsDefaultClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_DEFAULT_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDefaultClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsDefaultClause")
            .field(
                "default_token",
                &support::DebugSyntaxResult(self.default_token()),
            )
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("consequent", &self.consequent())
            .finish()
    }
}
impl From<JsDefaultClause> for SyntaxNode {
    fn from(n: JsDefaultClause) -> SyntaxNode { n.syntax }
}
impl From<JsDefaultClause> for SyntaxElement {
    fn from(n: JsDefaultClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsDefaultImportSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_DEFAULT_IMPORT_SPECIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDefaultImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsDefaultImportSpecifier")
            .field("local_name", &support::DebugSyntaxResult(self.local_name()))
            .field(
                "trailing_comma_token",
                &support::DebugSyntaxResult(self.trailing_comma_token()),
            )
            .finish()
    }
}
impl From<JsDefaultImportSpecifier> for SyntaxNode {
    fn from(n: JsDefaultImportSpecifier) -> SyntaxNode { n.syntax }
}
impl From<JsDefaultImportSpecifier> for SyntaxElement {
    fn from(n: JsDefaultImportSpecifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsDirective {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_DIRECTIVE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsDirective")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsDirective> for SyntaxNode {
    fn from(n: JsDirective) -> SyntaxNode { n.syntax }
}
impl From<JsDirective> for SyntaxElement {
    fn from(n: JsDirective) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsDoWhileStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_DO_WHILE_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsDoWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsDoWhileStatement")
            .field("do_token", &support::DebugSyntaxResult(self.do_token()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .field(
                "while_token",
                &support::DebugSyntaxResult(self.while_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("test", &support::DebugSyntaxResult(self.test()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsDoWhileStatement> for SyntaxNode {
    fn from(n: JsDoWhileStatement) -> SyntaxNode { n.syntax }
}
impl From<JsDoWhileStatement> for SyntaxElement {
    fn from(n: JsDoWhileStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsElseClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ELSE_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsElseClause")
            .field("else_token", &support::DebugSyntaxResult(self.else_token()))
            .field("alternate", &support::DebugSyntaxResult(self.alternate()))
            .finish()
    }
}
impl From<JsElseClause> for SyntaxNode {
    fn from(n: JsElseClause) -> SyntaxNode { n.syntax }
}
impl From<JsElseClause> for SyntaxElement {
    fn from(n: JsElseClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsEmptyClassMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EMPTY_CLASS_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsEmptyClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsEmptyClassMember")
            .field(
                "semicolon_token",
                &support::DebugSyntaxResult(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsEmptyClassMember> for SyntaxNode {
    fn from(n: JsEmptyClassMember) -> SyntaxNode { n.syntax }
}
impl From<JsEmptyClassMember> for SyntaxElement {
    fn from(n: JsEmptyClassMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsEmptyStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EMPTY_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsEmptyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsEmptyStatement")
            .field(
                "semicolon_token",
                &support::DebugSyntaxResult(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsEmptyStatement> for SyntaxNode {
    fn from(n: JsEmptyStatement) -> SyntaxNode { n.syntax }
}
impl From<JsEmptyStatement> for SyntaxElement {
    fn from(n: JsEmptyStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExport {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExport")
            .field(
                "export_token",
                &support::DebugSyntaxResult(self.export_token()),
            )
            .field(
                "export_clause",
                &support::DebugSyntaxResult(self.export_clause()),
            )
            .finish()
    }
}
impl From<JsExport> for SyntaxNode {
    fn from(n: JsExport) -> SyntaxNode { n.syntax }
}
impl From<JsExport> for SyntaxElement {
    fn from(n: JsExport) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportAsClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_AS_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportAsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportAsClause")
            .field("as_token", &support::DebugSyntaxResult(self.as_token()))
            .field(
                "exported_name",
                &support::DebugSyntaxResult(self.exported_name()),
            )
            .finish()
    }
}
impl From<JsExportAsClause> for SyntaxNode {
    fn from(n: JsExportAsClause) -> SyntaxNode { n.syntax }
}
impl From<JsExportAsClause> for SyntaxElement {
    fn from(n: JsExportAsClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportDefaultClassClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_DEFAULT_CLASS_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportDefaultClassClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportDefaultClassClause")
            .field(
                "default_token",
                &support::DebugSyntaxResult(self.default_token()),
            )
            .field(
                "class_token",
                &support::DebugSyntaxResult(self.class_token()),
            )
            .field("id", &support::DebugOptionalElement(self.id()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field(
                "extends_clause",
                &support::DebugOptionalElement(self.extends_clause()),
            )
            .field(
                "implements_clause",
                &support::DebugOptionalElement(self.implements_clause()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("members", &self.members())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsExportDefaultClassClause> for SyntaxNode {
    fn from(n: JsExportDefaultClassClause) -> SyntaxNode { n.syntax }
}
impl From<JsExportDefaultClassClause> for SyntaxElement {
    fn from(n: JsExportDefaultClassClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportDefaultExpressionClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportDefaultExpressionClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportDefaultExpressionClause")
            .field(
                "default_token",
                &support::DebugSyntaxResult(self.default_token()),
            )
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsExportDefaultExpressionClause> for SyntaxNode {
    fn from(n: JsExportDefaultExpressionClause) -> SyntaxNode { n.syntax }
}
impl From<JsExportDefaultExpressionClause> for SyntaxElement {
    fn from(n: JsExportDefaultExpressionClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportDefaultFunctionClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_DEFAULT_FUNCTION_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportDefaultFunctionClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportDefaultFunctionClause")
            .field(
                "default_token",
                &support::DebugSyntaxResult(self.default_token()),
            )
            .field(
                "async_token",
                &support::DebugOptionalElement(self.async_token()),
            )
            .field(
                "function_token",
                &support::DebugSyntaxResult(self.function_token()),
            )
            .field(
                "star_token",
                &support::DebugOptionalElement(self.star_token()),
            )
            .field("id", &support::DebugOptionalElement(self.id()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsExportDefaultFunctionClause> for SyntaxNode {
    fn from(n: JsExportDefaultFunctionClause) -> SyntaxNode { n.syntax }
}
impl From<JsExportDefaultFunctionClause> for SyntaxElement {
    fn from(n: JsExportDefaultFunctionClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportFromClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_FROM_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportFromClause")
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .field(
                "export_as",
                &support::DebugOptionalElement(self.export_as()),
            )
            .field("from_token", &support::DebugSyntaxResult(self.from_token()))
            .field("source", &support::DebugSyntaxResult(self.source()))
            .field(
                "assertion",
                &support::DebugOptionalElement(self.assertion()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsExportFromClause> for SyntaxNode {
    fn from(n: JsExportFromClause) -> SyntaxNode { n.syntax }
}
impl From<JsExportFromClause> for SyntaxElement {
    fn from(n: JsExportFromClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportNamedClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_NAMED_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportNamedClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportNamedClause")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("specifiers", &self.specifiers())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsExportNamedClause> for SyntaxNode {
    fn from(n: JsExportNamedClause) -> SyntaxNode { n.syntax }
}
impl From<JsExportNamedClause> for SyntaxElement {
    fn from(n: JsExportNamedClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportNamedFromClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_NAMED_FROM_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportNamedFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportNamedFromClause")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("specifiers", &self.specifiers())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .field("from_token", &support::DebugSyntaxResult(self.from_token()))
            .field("source", &support::DebugSyntaxResult(self.source()))
            .field(
                "assertion",
                &support::DebugOptionalElement(self.assertion()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsExportNamedFromClause> for SyntaxNode {
    fn from(n: JsExportNamedFromClause) -> SyntaxNode { n.syntax }
}
impl From<JsExportNamedFromClause> for SyntaxElement {
    fn from(n: JsExportNamedFromClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportNamedFromSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_NAMED_FROM_SPECIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportNamedFromSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportNamedFromSpecifier")
            .field(
                "type_token",
                &support::DebugOptionalElement(self.type_token()),
            )
            .field(
                "source_name",
                &support::DebugSyntaxResult(self.source_name()),
            )
            .field(
                "export_as",
                &support::DebugOptionalElement(self.export_as()),
            )
            .finish()
    }
}
impl From<JsExportNamedFromSpecifier> for SyntaxNode {
    fn from(n: JsExportNamedFromSpecifier) -> SyntaxNode { n.syntax }
}
impl From<JsExportNamedFromSpecifier> for SyntaxElement {
    fn from(n: JsExportNamedFromSpecifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportNamedShorthandSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_NAMED_SHORTHAND_SPECIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportNamedShorthandSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportNamedShorthandSpecifier")
            .field(
                "type_token",
                &support::DebugOptionalElement(self.type_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<JsExportNamedShorthandSpecifier> for SyntaxNode {
    fn from(n: JsExportNamedShorthandSpecifier) -> SyntaxNode { n.syntax }
}
impl From<JsExportNamedShorthandSpecifier> for SyntaxElement {
    fn from(n: JsExportNamedShorthandSpecifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExportNamedSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_NAMED_SPECIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExportNamedSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExportNamedSpecifier")
            .field(
                "type_token",
                &support::DebugOptionalElement(self.type_token()),
            )
            .field("local_name", &support::DebugSyntaxResult(self.local_name()))
            .field("as_token", &support::DebugSyntaxResult(self.as_token()))
            .field(
                "exported_name",
                &support::DebugSyntaxResult(self.exported_name()),
            )
            .finish()
    }
}
impl From<JsExportNamedSpecifier> for SyntaxNode {
    fn from(n: JsExportNamedSpecifier) -> SyntaxNode { n.syntax }
}
impl From<JsExportNamedSpecifier> for SyntaxElement {
    fn from(n: JsExportNamedSpecifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExpressionSnipped {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPRESSION_SNIPPED }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExpressionSnipped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExpressionSnipped")
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<JsExpressionSnipped> for SyntaxNode {
    fn from(n: JsExpressionSnipped) -> SyntaxNode { n.syntax }
}
impl From<JsExpressionSnipped> for SyntaxElement {
    fn from(n: JsExpressionSnipped) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExpressionStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPRESSION_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExpressionStatement")
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsExpressionStatement> for SyntaxNode {
    fn from(n: JsExpressionStatement) -> SyntaxNode { n.syntax }
}
impl From<JsExpressionStatement> for SyntaxElement {
    fn from(n: JsExpressionStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsExtendsClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXTENDS_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsExtendsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsExtendsClause")
            .field(
                "extends_token",
                &support::DebugSyntaxResult(self.extends_token()),
            )
            .field(
                "super_class",
                &support::DebugSyntaxResult(self.super_class()),
            )
            .field(
                "type_arguments",
                &support::DebugOptionalElement(self.type_arguments()),
            )
            .finish()
    }
}
impl From<JsExtendsClause> for SyntaxNode {
    fn from(n: JsExtendsClause) -> SyntaxNode { n.syntax }
}
impl From<JsExtendsClause> for SyntaxElement {
    fn from(n: JsExtendsClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsFinallyClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FINALLY_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFinallyClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsFinallyClause")
            .field(
                "finally_token",
                &support::DebugSyntaxResult(self.finally_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsFinallyClause> for SyntaxNode {
    fn from(n: JsFinallyClause) -> SyntaxNode { n.syntax }
}
impl From<JsFinallyClause> for SyntaxElement {
    fn from(n: JsFinallyClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsForInStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FOR_IN_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsForInStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsForInStatement")
            .field("for_token", &support::DebugSyntaxResult(self.for_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "initializer",
                &support::DebugSyntaxResult(self.initializer()),
            )
            .field("in_token", &support::DebugSyntaxResult(self.in_token()))
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsForInStatement> for SyntaxNode {
    fn from(n: JsForInStatement) -> SyntaxNode { n.syntax }
}
impl From<JsForInStatement> for SyntaxElement {
    fn from(n: JsForInStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsForOfStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FOR_OF_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsForOfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsForOfStatement")
            .field("for_token", &support::DebugSyntaxResult(self.for_token()))
            .field(
                "await_token",
                &support::DebugOptionalElement(self.await_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "initializer",
                &support::DebugSyntaxResult(self.initializer()),
            )
            .field("of_token", &support::DebugSyntaxResult(self.of_token()))
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsForOfStatement> for SyntaxNode {
    fn from(n: JsForOfStatement) -> SyntaxNode { n.syntax }
}
impl From<JsForOfStatement> for SyntaxElement {
    fn from(n: JsForOfStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsForStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FOR_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsForStatement")
            .field("for_token", &support::DebugSyntaxResult(self.for_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "initializer",
                &support::DebugOptionalElement(self.initializer()),
            )
            .field(
                "first_semi_token",
                &support::DebugSyntaxResult(self.first_semi_token()),
            )
            .field("test", &support::DebugOptionalElement(self.test()))
            .field(
                "second_semi_token",
                &support::DebugSyntaxResult(self.second_semi_token()),
            )
            .field("update", &support::DebugOptionalElement(self.update()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsForStatement> for SyntaxNode {
    fn from(n: JsForStatement) -> SyntaxNode { n.syntax }
}
impl From<JsForStatement> for SyntaxElement {
    fn from(n: JsForStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsForVariableDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FOR_VARIABLE_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsForVariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsForVariableDeclaration")
            .field("kind_token", &support::DebugSyntaxResult(self.kind_token()))
            .field("declarator", &support::DebugSyntaxResult(self.declarator()))
            .finish()
    }
}
impl From<JsForVariableDeclaration> for SyntaxNode {
    fn from(n: JsForVariableDeclaration) -> SyntaxNode { n.syntax }
}
impl From<JsForVariableDeclaration> for SyntaxElement {
    fn from(n: JsForVariableDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsFormalParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FORMAL_PARAMETER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFormalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsFormalParameter")
            .field("binding", &support::DebugSyntaxResult(self.binding()))
            .field(
                "question_mark_token",
                &support::DebugOptionalElement(self.question_mark_token()),
            )
            .field(
                "type_annotation",
                &support::DebugOptionalElement(self.type_annotation()),
            )
            .field(
                "initializer",
                &support::DebugOptionalElement(self.initializer()),
            )
            .finish()
    }
}
impl From<JsFormalParameter> for SyntaxNode {
    fn from(n: JsFormalParameter) -> SyntaxNode { n.syntax }
}
impl From<JsFormalParameter> for SyntaxElement {
    fn from(n: JsFormalParameter) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsFunctionBody {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FUNCTION_BODY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsFunctionBody")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("directives", &self.directives())
            .field("statements", &self.statements())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsFunctionBody> for SyntaxNode {
    fn from(n: JsFunctionBody) -> SyntaxNode { n.syntax }
}
impl From<JsFunctionBody> for SyntaxElement {
    fn from(n: JsFunctionBody) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsFunctionDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FUNCTION_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsFunctionDeclaration")
            .field(
                "async_token",
                &support::DebugOptionalElement(self.async_token()),
            )
            .field(
                "function_token",
                &support::DebugSyntaxResult(self.function_token()),
            )
            .field(
                "star_token",
                &support::DebugOptionalElement(self.star_token()),
            )
            .field("id", &support::DebugSyntaxResult(self.id()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsFunctionDeclaration> for SyntaxNode {
    fn from(n: JsFunctionDeclaration) -> SyntaxNode { n.syntax }
}
impl From<JsFunctionDeclaration> for SyntaxElement {
    fn from(n: JsFunctionDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsFunctionExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_FUNCTION_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsFunctionExpression")
            .field(
                "async_token",
                &support::DebugOptionalElement(self.async_token()),
            )
            .field(
                "function_token",
                &support::DebugSyntaxResult(self.function_token()),
            )
            .field(
                "star_token",
                &support::DebugOptionalElement(self.star_token()),
            )
            .field("id", &support::DebugOptionalElement(self.id()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsFunctionExpression> for SyntaxNode {
    fn from(n: JsFunctionExpression) -> SyntaxNode { n.syntax }
}
impl From<JsFunctionExpression> for SyntaxElement {
    fn from(n: JsFunctionExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsGetterClassMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_GETTER_CLASS_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsGetterClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsGetterClassMember")
            .field(
                "access_modifier",
                &support::DebugOptionalElement(self.access_modifier()),
            )
            .field(
                "static_token",
                &support::DebugOptionalElement(self.static_token()),
            )
            .field(
                "abstract_token",
                &support::DebugOptionalElement(self.abstract_token()),
            )
            .field("get_token", &support::DebugSyntaxResult(self.get_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "return_type",
                &support::DebugOptionalElement(self.return_type()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsGetterClassMember> for SyntaxNode {
    fn from(n: JsGetterClassMember) -> SyntaxNode { n.syntax }
}
impl From<JsGetterClassMember> for SyntaxElement {
    fn from(n: JsGetterClassMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsGetterObjectMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_GETTER_OBJECT_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsGetterObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsGetterObjectMember")
            .field("get_token", &support::DebugSyntaxResult(self.get_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "return_type",
                &support::DebugOptionalElement(self.return_type()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsGetterObjectMember> for SyntaxNode {
    fn from(n: JsGetterObjectMember) -> SyntaxNode { n.syntax }
}
impl From<JsGetterObjectMember> for SyntaxElement {
    fn from(n: JsGetterObjectMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsIdentifierAssignment {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IDENTIFIER_ASSIGNMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsIdentifierAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsIdentifierAssignment")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .finish()
    }
}
impl From<JsIdentifierAssignment> for SyntaxNode {
    fn from(n: JsIdentifierAssignment) -> SyntaxNode { n.syntax }
}
impl From<JsIdentifierAssignment> for SyntaxElement {
    fn from(n: JsIdentifierAssignment) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsIdentifierBinding {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IDENTIFIER_BINDING }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsIdentifierBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsIdentifierBinding")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .finish()
    }
}
impl From<JsIdentifierBinding> for SyntaxNode {
    fn from(n: JsIdentifierBinding) -> SyntaxNode { n.syntax }
}
impl From<JsIdentifierBinding> for SyntaxElement {
    fn from(n: JsIdentifierBinding) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsIdentifierExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IDENTIFIER_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsIdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsIdentifierExpression")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<JsIdentifierExpression> for SyntaxNode {
    fn from(n: JsIdentifierExpression) -> SyntaxNode { n.syntax }
}
impl From<JsIdentifierExpression> for SyntaxElement {
    fn from(n: JsIdentifierExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsIfStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IF_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsIfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsIfStatement")
            .field("if_token", &support::DebugSyntaxResult(self.if_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("test", &support::DebugSyntaxResult(self.test()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("consequent", &support::DebugSyntaxResult(self.consequent()))
            .field(
                "else_clause",
                &support::DebugOptionalElement(self.else_clause()),
            )
            .finish()
    }
}
impl From<JsIfStatement> for SyntaxNode {
    fn from(n: JsIfStatement) -> SyntaxNode { n.syntax }
}
impl From<JsIfStatement> for SyntaxElement {
    fn from(n: JsIfStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsImport {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsImport")
            .field(
                "import_token",
                &support::DebugSyntaxResult(self.import_token()),
            )
            .field(
                "import_clause",
                &support::DebugSyntaxResult(self.import_clause()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsImport> for SyntaxNode {
    fn from(n: JsImport) -> SyntaxNode { n.syntax }
}
impl From<JsImport> for SyntaxElement {
    fn from(n: JsImport) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsImportAssertion {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT_ASSERTION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportAssertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsImportAssertion")
            .field(
                "assert_token",
                &support::DebugSyntaxResult(self.assert_token()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("assertions", &self.assertions())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsImportAssertion> for SyntaxNode {
    fn from(n: JsImportAssertion) -> SyntaxNode { n.syntax }
}
impl From<JsImportAssertion> for SyntaxElement {
    fn from(n: JsImportAssertion) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsImportAssertionEntry {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT_ASSERTION_ENTRY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportAssertionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsImportAssertionEntry")
            .field("key", &support::DebugSyntaxResult(self.key()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsImportAssertionEntry> for SyntaxNode {
    fn from(n: JsImportAssertionEntry) -> SyntaxNode { n.syntax }
}
impl From<JsImportAssertionEntry> for SyntaxElement {
    fn from(n: JsImportAssertionEntry) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsImportBareClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT_BARE_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportBareClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsImportBareClause")
            .field("source", &support::DebugSyntaxResult(self.source()))
            .field(
                "assertion",
                &support::DebugOptionalElement(self.assertion()),
            )
            .finish()
    }
}
impl From<JsImportBareClause> for SyntaxNode {
    fn from(n: JsImportBareClause) -> SyntaxNode { n.syntax }
}
impl From<JsImportBareClause> for SyntaxElement {
    fn from(n: JsImportBareClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsImportCallExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT_CALL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsImportCallExpression")
            .field(
                "import_token",
                &support::DebugSyntaxResult(self.import_token()),
            )
            .field("arguments", &support::DebugSyntaxResult(self.arguments()))
            .finish()
    }
}
impl From<JsImportCallExpression> for SyntaxNode {
    fn from(n: JsImportCallExpression) -> SyntaxNode { n.syntax }
}
impl From<JsImportCallExpression> for SyntaxElement {
    fn from(n: JsImportCallExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsImportDefaultClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT_DEFAULT_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportDefaultClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsImportDefaultClause")
            .field("local_name", &support::DebugSyntaxResult(self.local_name()))
            .field("from_token", &support::DebugSyntaxResult(self.from_token()))
            .field("source", &support::DebugSyntaxResult(self.source()))
            .field(
                "assertion",
                &support::DebugOptionalElement(self.assertion()),
            )
            .finish()
    }
}
impl From<JsImportDefaultClause> for SyntaxNode {
    fn from(n: JsImportDefaultClause) -> SyntaxNode { n.syntax }
}
impl From<JsImportDefaultClause> for SyntaxElement {
    fn from(n: JsImportDefaultClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsImportNamedClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT_NAMED_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportNamedClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsImportNamedClause")
            .field(
                "default_specifier",
                &support::DebugOptionalElement(self.default_specifier()),
            )
            .field(
                "named_import",
                &support::DebugSyntaxResult(self.named_import()),
            )
            .field("from_token", &support::DebugSyntaxResult(self.from_token()))
            .field("source", &support::DebugSyntaxResult(self.source()))
            .field(
                "assertion",
                &support::DebugOptionalElement(self.assertion()),
            )
            .finish()
    }
}
impl From<JsImportNamedClause> for SyntaxNode {
    fn from(n: JsImportNamedClause) -> SyntaxNode { n.syntax }
}
impl From<JsImportNamedClause> for SyntaxElement {
    fn from(n: JsImportNamedClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsImportNamespaceClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT_NAMESPACE_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsImportNamespaceClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsImportNamespaceClause")
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .field("as_token", &support::DebugSyntaxResult(self.as_token()))
            .field("local_name", &support::DebugSyntaxResult(self.local_name()))
            .field("from_token", &support::DebugSyntaxResult(self.from_token()))
            .field("source", &support::DebugSyntaxResult(self.source()))
            .field(
                "assertion",
                &support::DebugOptionalElement(self.assertion()),
            )
            .finish()
    }
}
impl From<JsImportNamespaceClause> for SyntaxNode {
    fn from(n: JsImportNamespaceClause) -> SyntaxNode { n.syntax }
}
impl From<JsImportNamespaceClause> for SyntaxElement {
    fn from(n: JsImportNamespaceClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsInExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IN_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsInExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsInExpression")
            .field("property", &support::DebugSyntaxResult(self.property()))
            .field("in_token", &support::DebugSyntaxResult(self.in_token()))
            .field("object", &support::DebugSyntaxResult(self.object()))
            .finish()
    }
}
impl From<JsInExpression> for SyntaxNode {
    fn from(n: JsInExpression) -> SyntaxNode { n.syntax }
}
impl From<JsInExpression> for SyntaxElement {
    fn from(n: JsInExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsInitializerClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_INITIALIZER_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsInitializerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsInitializerClause")
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .finish()
    }
}
impl From<JsInitializerClause> for SyntaxNode {
    fn from(n: JsInitializerClause) -> SyntaxNode { n.syntax }
}
impl From<JsInitializerClause> for SyntaxElement {
    fn from(n: JsInitializerClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsInstanceofExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_INSTANCEOF_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsInstanceofExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsInstanceofExpression")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "instanceof_token",
                &support::DebugSyntaxResult(self.instanceof_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<JsInstanceofExpression> for SyntaxNode {
    fn from(n: JsInstanceofExpression) -> SyntaxNode { n.syntax }
}
impl From<JsInstanceofExpression> for SyntaxElement {
    fn from(n: JsInstanceofExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsLabeledStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_LABELED_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsLabeledStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsLabeledStatement")
            .field(
                "label_token",
                &support::DebugSyntaxResult(self.label_token()),
            )
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsLabeledStatement> for SyntaxNode {
    fn from(n: JsLabeledStatement) -> SyntaxNode { n.syntax }
}
impl From<JsLabeledStatement> for SyntaxElement {
    fn from(n: JsLabeledStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsLiteralExportName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_LITERAL_EXPORT_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsLiteralExportName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsLiteralExportName")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<JsLiteralExportName> for SyntaxNode {
    fn from(n: JsLiteralExportName) -> SyntaxNode { n.syntax }
}
impl From<JsLiteralExportName> for SyntaxElement {
    fn from(n: JsLiteralExportName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsLiteralMemberName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_LITERAL_MEMBER_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsLiteralMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsLiteralMemberName")
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<JsLiteralMemberName> for SyntaxNode {
    fn from(n: JsLiteralMemberName) -> SyntaxNode { n.syntax }
}
impl From<JsLiteralMemberName> for SyntaxElement {
    fn from(n: JsLiteralMemberName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsLogicalExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_LOGICAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsLogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsLogicalExpression")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<JsLogicalExpression> for SyntaxNode {
    fn from(n: JsLogicalExpression) -> SyntaxNode { n.syntax }
}
impl From<JsLogicalExpression> for SyntaxElement {
    fn from(n: JsLogicalExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsMethodClassMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_METHOD_CLASS_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsMethodClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsMethodClassMember")
            .field(
                "access_modifier",
                &support::DebugOptionalElement(self.access_modifier()),
            )
            .field(
                "static_token",
                &support::DebugOptionalElement(self.static_token()),
            )
            .field(
                "abstract_token",
                &support::DebugOptionalElement(self.abstract_token()),
            )
            .field(
                "async_token",
                &support::DebugOptionalElement(self.async_token()),
            )
            .field(
                "star_token",
                &support::DebugOptionalElement(self.star_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "question_mark_token",
                &support::DebugSyntaxResult(self.question_mark_token()),
            )
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsMethodClassMember> for SyntaxNode {
    fn from(n: JsMethodClassMember) -> SyntaxNode { n.syntax }
}
impl From<JsMethodClassMember> for SyntaxElement {
    fn from(n: JsMethodClassMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsMethodObjectMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_METHOD_OBJECT_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsMethodObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsMethodObjectMember")
            .field(
                "async_token",
                &support::DebugOptionalElement(self.async_token()),
            )
            .field(
                "star_token",
                &support::DebugOptionalElement(self.star_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsMethodObjectMember> for SyntaxNode {
    fn from(n: JsMethodObjectMember) -> SyntaxNode { n.syntax }
}
impl From<JsMethodObjectMember> for SyntaxElement {
    fn from(n: JsMethodObjectMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsModule {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_MODULE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsModule")
            .field(
                "interpreter_token",
                &support::DebugOptionalElement(self.interpreter_token()),
            )
            .field("directives", &self.directives())
            .field("items", &self.items())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<JsModule> for SyntaxNode {
    fn from(n: JsModule) -> SyntaxNode { n.syntax }
}
impl From<JsModule> for SyntaxElement {
    fn from(n: JsModule) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsModuleSource {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_MODULE_SOURCE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsModuleSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsModuleSource")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsModuleSource> for SyntaxNode {
    fn from(n: JsModuleSource) -> SyntaxNode { n.syntax }
}
impl From<JsModuleSource> for SyntaxElement {
    fn from(n: JsModuleSource) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsName")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsName> for SyntaxNode {
    fn from(n: JsName) -> SyntaxNode { n.syntax }
}
impl From<JsName> for SyntaxElement {
    fn from(n: JsName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsNamedImportSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_NAMED_IMPORT_SPECIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsNamedImportSpecifier")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("as_token", &support::DebugSyntaxResult(self.as_token()))
            .field("local_name", &support::DebugSyntaxResult(self.local_name()))
            .finish()
    }
}
impl From<JsNamedImportSpecifier> for SyntaxNode {
    fn from(n: JsNamedImportSpecifier) -> SyntaxNode { n.syntax }
}
impl From<JsNamedImportSpecifier> for SyntaxElement {
    fn from(n: JsNamedImportSpecifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsNamedImportSpecifiers {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_NAMED_IMPORT_SPECIFIERS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNamedImportSpecifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsNamedImportSpecifiers")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("specifiers", &self.specifiers())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsNamedImportSpecifiers> for SyntaxNode {
    fn from(n: JsNamedImportSpecifiers) -> SyntaxNode { n.syntax }
}
impl From<JsNamedImportSpecifiers> for SyntaxElement {
    fn from(n: JsNamedImportSpecifiers) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsNamespaceImportSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_NAMESPACE_IMPORT_SPECIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNamespaceImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsNamespaceImportSpecifier")
            .field("star_token", &support::DebugSyntaxResult(self.star_token()))
            .field("as_token", &support::DebugSyntaxResult(self.as_token()))
            .field("local_name", &support::DebugSyntaxResult(self.local_name()))
            .finish()
    }
}
impl From<JsNamespaceImportSpecifier> for SyntaxNode {
    fn from(n: JsNamespaceImportSpecifier) -> SyntaxNode { n.syntax }
}
impl From<JsNamespaceImportSpecifier> for SyntaxElement {
    fn from(n: JsNamespaceImportSpecifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsNewExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_NEW_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNewExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsNewExpression")
            .field("new_token", &support::DebugSyntaxResult(self.new_token()))
            .field("callee", &support::DebugSyntaxResult(self.callee()))
            .field(
                "type_arguments",
                &support::DebugOptionalElement(self.type_arguments()),
            )
            .field(
                "arguments",
                &support::DebugOptionalElement(self.arguments()),
            )
            .finish()
    }
}
impl From<JsNewExpression> for SyntaxNode {
    fn from(n: JsNewExpression) -> SyntaxNode { n.syntax }
}
impl From<JsNewExpression> for SyntaxElement {
    fn from(n: JsNewExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsNullLiteralExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_NULL_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsNullLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsNullLiteralExpression> for SyntaxNode {
    fn from(n: JsNullLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsNullLiteralExpression> for SyntaxElement {
    fn from(n: JsNullLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsNumberLiteralExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_NUMBER_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsNumberLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsNumberLiteralExpression> for SyntaxNode {
    fn from(n: JsNumberLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsNumberLiteralExpression> for SyntaxElement {
    fn from(n: JsNumberLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectAssignmentPattern {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectAssignmentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectAssignmentPattern")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("properties", &self.properties())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsObjectAssignmentPattern> for SyntaxNode {
    fn from(n: JsObjectAssignmentPattern) -> SyntaxNode { n.syntax }
}
impl From<JsObjectAssignmentPattern> for SyntaxElement {
    fn from(n: JsObjectAssignmentPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectAssignmentPatternProperty {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectAssignmentPatternProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectAssignmentPatternProperty")
            .field("member", &support::DebugSyntaxResult(self.member()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .field("init", &support::DebugOptionalElement(self.init()))
            .finish()
    }
}
impl From<JsObjectAssignmentPatternProperty> for SyntaxNode {
    fn from(n: JsObjectAssignmentPatternProperty) -> SyntaxNode { n.syntax }
}
impl From<JsObjectAssignmentPatternProperty> for SyntaxElement {
    fn from(n: JsObjectAssignmentPatternProperty) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectAssignmentPatternRest {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN_REST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectAssignmentPatternRest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectAssignmentPatternRest")
            .field(
                "dotdotdot_token",
                &support::DebugSyntaxResult(self.dotdotdot_token()),
            )
            .field("target", &support::DebugSyntaxResult(self.target()))
            .finish()
    }
}
impl From<JsObjectAssignmentPatternRest> for SyntaxNode {
    fn from(n: JsObjectAssignmentPatternRest) -> SyntaxNode { n.syntax }
}
impl From<JsObjectAssignmentPatternRest> for SyntaxElement {
    fn from(n: JsObjectAssignmentPatternRest) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectAssignmentPatternShorthandProperty {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        kind == JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectAssignmentPatternShorthandProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectAssignmentPatternShorthandProperty")
            .field("identifier", &support::DebugSyntaxResult(self.identifier()))
            .field("init", &support::DebugOptionalElement(self.init()))
            .finish()
    }
}
impl From<JsObjectAssignmentPatternShorthandProperty> for SyntaxNode {
    fn from(n: JsObjectAssignmentPatternShorthandProperty) -> SyntaxNode { n.syntax }
}
impl From<JsObjectAssignmentPatternShorthandProperty> for SyntaxElement {
    fn from(n: JsObjectAssignmentPatternShorthandProperty) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectBindingPattern {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectBindingPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectBindingPattern")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("properties", &self.properties())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsObjectBindingPattern> for SyntaxNode {
    fn from(n: JsObjectBindingPattern) -> SyntaxNode { n.syntax }
}
impl From<JsObjectBindingPattern> for SyntaxElement {
    fn from(n: JsObjectBindingPattern) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectBindingPatternProperty {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_PROPERTY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectBindingPatternProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectBindingPatternProperty")
            .field("member", &support::DebugSyntaxResult(self.member()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("pattern", &support::DebugSyntaxResult(self.pattern()))
            .field("init", &support::DebugOptionalElement(self.init()))
            .finish()
    }
}
impl From<JsObjectBindingPatternProperty> for SyntaxNode {
    fn from(n: JsObjectBindingPatternProperty) -> SyntaxNode { n.syntax }
}
impl From<JsObjectBindingPatternProperty> for SyntaxElement {
    fn from(n: JsObjectBindingPatternProperty) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectBindingPatternRest {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_REST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectBindingPatternRest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectBindingPatternRest")
            .field(
                "dotdotdot_token",
                &support::DebugSyntaxResult(self.dotdotdot_token()),
            )
            .field("binding", &support::DebugSyntaxResult(self.binding()))
            .finish()
    }
}
impl From<JsObjectBindingPatternRest> for SyntaxNode {
    fn from(n: JsObjectBindingPatternRest) -> SyntaxNode { n.syntax }
}
impl From<JsObjectBindingPatternRest> for SyntaxElement {
    fn from(n: JsObjectBindingPatternRest) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectBindingPatternShorthandProperty {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectBindingPatternShorthandProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectBindingPatternShorthandProperty")
            .field("identifier", &support::DebugSyntaxResult(self.identifier()))
            .field("init", &support::DebugOptionalElement(self.init()))
            .finish()
    }
}
impl From<JsObjectBindingPatternShorthandProperty> for SyntaxNode {
    fn from(n: JsObjectBindingPatternShorthandProperty) -> SyntaxNode { n.syntax }
}
impl From<JsObjectBindingPatternShorthandProperty> for SyntaxElement {
    fn from(n: JsObjectBindingPatternShorthandProperty) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsObjectExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsObjectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsObjectExpression")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("members", &self.members())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsObjectExpression> for SyntaxNode {
    fn from(n: JsObjectExpression) -> SyntaxNode { n.syntax }
}
impl From<JsObjectExpression> for SyntaxElement {
    fn from(n: JsObjectExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsParameters {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PARAMETERS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsParameters")
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
impl From<JsParameters> for SyntaxNode {
    fn from(n: JsParameters) -> SyntaxNode { n.syntax }
}
impl From<JsParameters> for SyntaxElement {
    fn from(n: JsParameters) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsParenthesizedAssignment {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PARENTHESIZED_ASSIGNMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsParenthesizedAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsParenthesizedAssignment")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("assignment", &support::DebugSyntaxResult(self.assignment()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<JsParenthesizedAssignment> for SyntaxNode {
    fn from(n: JsParenthesizedAssignment) -> SyntaxNode { n.syntax }
}
impl From<JsParenthesizedAssignment> for SyntaxElement {
    fn from(n: JsParenthesizedAssignment) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsParenthesizedExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PARENTHESIZED_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsParenthesizedExpression")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<JsParenthesizedExpression> for SyntaxNode {
    fn from(n: JsParenthesizedExpression) -> SyntaxNode { n.syntax }
}
impl From<JsParenthesizedExpression> for SyntaxElement {
    fn from(n: JsParenthesizedExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsPostUpdateExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_POST_UPDATE_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPostUpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsPostUpdateExpression")
            .field("operand", &support::DebugSyntaxResult(self.operand()))
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .finish()
    }
}
impl From<JsPostUpdateExpression> for SyntaxNode {
    fn from(n: JsPostUpdateExpression) -> SyntaxNode { n.syntax }
}
impl From<JsPostUpdateExpression> for SyntaxElement {
    fn from(n: JsPostUpdateExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsPreUpdateExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PRE_UPDATE_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPreUpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsPreUpdateExpression")
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .field("operand", &support::DebugSyntaxResult(self.operand()))
            .finish()
    }
}
impl From<JsPreUpdateExpression> for SyntaxNode {
    fn from(n: JsPreUpdateExpression) -> SyntaxNode { n.syntax }
}
impl From<JsPreUpdateExpression> for SyntaxElement {
    fn from(n: JsPreUpdateExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsPrivateClassMemberName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PRIVATE_CLASS_MEMBER_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPrivateClassMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsPrivateClassMemberName")
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field("id_token", &support::DebugSyntaxResult(self.id_token()))
            .finish()
    }
}
impl From<JsPrivateClassMemberName> for SyntaxNode {
    fn from(n: JsPrivateClassMemberName) -> SyntaxNode { n.syntax }
}
impl From<JsPrivateClassMemberName> for SyntaxElement {
    fn from(n: JsPrivateClassMemberName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsPrivateName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PRIVATE_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPrivateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsPrivateName")
            .field("hash_token", &support::DebugSyntaxResult(self.hash_token()))
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsPrivateName> for SyntaxNode {
    fn from(n: JsPrivateName) -> SyntaxNode { n.syntax }
}
impl From<JsPrivateName> for SyntaxElement {
    fn from(n: JsPrivateName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsPropertyClassMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PROPERTY_CLASS_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPropertyClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsPropertyClassMember")
            .field(
                "declare_token",
                &support::DebugOptionalElement(self.declare_token()),
            )
            .field(
                "access_modifier",
                &support::DebugOptionalElement(self.access_modifier()),
            )
            .field(
                "static_token",
                &support::DebugOptionalElement(self.static_token()),
            )
            .field(
                "readonly_token",
                &support::DebugOptionalElement(self.readonly_token()),
            )
            .field(
                "abstract_token",
                &support::DebugOptionalElement(self.abstract_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "property_annotation",
                &support::DebugOptionalElement(self.property_annotation()),
            )
            .field("value", &support::DebugOptionalElement(self.value()))
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsPropertyClassMember> for SyntaxNode {
    fn from(n: JsPropertyClassMember) -> SyntaxNode { n.syntax }
}
impl From<JsPropertyClassMember> for SyntaxElement {
    fn from(n: JsPropertyClassMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsPropertyObjectMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PROPERTY_OBJECT_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsPropertyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsPropertyObjectMember")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("value", &support::DebugSyntaxResult(self.value()))
            .finish()
    }
}
impl From<JsPropertyObjectMember> for SyntaxNode {
    fn from(n: JsPropertyObjectMember) -> SyntaxNode { n.syntax }
}
impl From<JsPropertyObjectMember> for SyntaxElement {
    fn from(n: JsPropertyObjectMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsReferenceIdentifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_REFERENCE_IDENTIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsReferenceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsReferenceIdentifier")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsReferenceIdentifier> for SyntaxNode {
    fn from(n: JsReferenceIdentifier) -> SyntaxNode { n.syntax }
}
impl From<JsReferenceIdentifier> for SyntaxElement {
    fn from(n: JsReferenceIdentifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsRegexLiteralExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_REGEX_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsRegexLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsRegexLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsRegexLiteralExpression> for SyntaxNode {
    fn from(n: JsRegexLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsRegexLiteralExpression> for SyntaxElement {
    fn from(n: JsRegexLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsRestParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_REST_PARAMETER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsRestParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsRestParameter")
            .field(
                "dotdotdot_token",
                &support::DebugSyntaxResult(self.dotdotdot_token()),
            )
            .field("binding", &support::DebugSyntaxResult(self.binding()))
            .field(
                "type_annotation",
                &support::DebugOptionalElement(self.type_annotation()),
            )
            .finish()
    }
}
impl From<JsRestParameter> for SyntaxNode {
    fn from(n: JsRestParameter) -> SyntaxNode { n.syntax }
}
impl From<JsRestParameter> for SyntaxElement {
    fn from(n: JsRestParameter) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsReturnStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_RETURN_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsReturnStatement")
            .field(
                "return_token",
                &support::DebugSyntaxResult(self.return_token()),
            )
            .field("argument", &support::DebugOptionalElement(self.argument()))
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsReturnStatement> for SyntaxNode {
    fn from(n: JsReturnStatement) -> SyntaxNode { n.syntax }
}
impl From<JsReturnStatement> for SyntaxElement {
    fn from(n: JsReturnStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsScript {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SCRIPT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsScript")
            .field(
                "interpreter_token",
                &support::DebugOptionalElement(self.interpreter_token()),
            )
            .field("directives", &self.directives())
            .field("statements", &self.statements())
            .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
            .finish()
    }
}
impl From<JsScript> for SyntaxNode {
    fn from(n: JsScript) -> SyntaxNode { n.syntax }
}
impl From<JsScript> for SyntaxElement {
    fn from(n: JsScript) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsSequenceExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SEQUENCE_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSequenceExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsSequenceExpression")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field(
                "comma_token",
                &support::DebugSyntaxResult(self.comma_token()),
            )
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<JsSequenceExpression> for SyntaxNode {
    fn from(n: JsSequenceExpression) -> SyntaxNode { n.syntax }
}
impl From<JsSequenceExpression> for SyntaxElement {
    fn from(n: JsSequenceExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsSetterClassMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SETTER_CLASS_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSetterClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsSetterClassMember")
            .field(
                "access_modifier",
                &support::DebugOptionalElement(self.access_modifier()),
            )
            .field(
                "static_token",
                &support::DebugOptionalElement(self.static_token()),
            )
            .field(
                "abstract_token",
                &support::DebugOptionalElement(self.abstract_token()),
            )
            .field("set_token", &support::DebugSyntaxResult(self.set_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("parameter", &support::DebugSyntaxResult(self.parameter()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsSetterClassMember> for SyntaxNode {
    fn from(n: JsSetterClassMember) -> SyntaxNode { n.syntax }
}
impl From<JsSetterClassMember> for SyntaxElement {
    fn from(n: JsSetterClassMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsSetterObjectMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SETTER_OBJECT_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSetterObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsSetterObjectMember")
            .field("set_token", &support::DebugSyntaxResult(self.set_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("parameter", &support::DebugSyntaxResult(self.parameter()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsSetterObjectMember> for SyntaxNode {
    fn from(n: JsSetterObjectMember) -> SyntaxNode { n.syntax }
}
impl From<JsSetterObjectMember> for SyntaxElement {
    fn from(n: JsSetterObjectMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsShorthandNamedImportSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SHORTHAND_NAMED_IMPORT_SPECIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsShorthandNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsShorthandNamedImportSpecifier")
            .field("local_name", &support::DebugSyntaxResult(self.local_name()))
            .finish()
    }
}
impl From<JsShorthandNamedImportSpecifier> for SyntaxNode {
    fn from(n: JsShorthandNamedImportSpecifier) -> SyntaxNode { n.syntax }
}
impl From<JsShorthandNamedImportSpecifier> for SyntaxElement {
    fn from(n: JsShorthandNamedImportSpecifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsShorthandPropertyObjectMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SHORTHAND_PROPERTY_OBJECT_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsShorthandPropertyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsShorthandPropertyObjectMember")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .finish()
    }
}
impl From<JsShorthandPropertyObjectMember> for SyntaxNode {
    fn from(n: JsShorthandPropertyObjectMember) -> SyntaxNode { n.syntax }
}
impl From<JsShorthandPropertyObjectMember> for SyntaxElement {
    fn from(n: JsShorthandPropertyObjectMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsSpread {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SPREAD }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSpread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsSpread")
            .field(
                "dotdotdot_token",
                &support::DebugSyntaxResult(self.dotdotdot_token()),
            )
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .finish()
    }
}
impl From<JsSpread> for SyntaxNode {
    fn from(n: JsSpread) -> SyntaxNode { n.syntax }
}
impl From<JsSpread> for SyntaxElement {
    fn from(n: JsSpread) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsStaticInitializationBlockClassMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsStaticInitializationBlockClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsStaticInitializationBlockClassMember")
            .field(
                "static_token",
                &support::DebugSyntaxResult(self.static_token()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("statements", &self.statements())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsStaticInitializationBlockClassMember> for SyntaxNode {
    fn from(n: JsStaticInitializationBlockClassMember) -> SyntaxNode { n.syntax }
}
impl From<JsStaticInitializationBlockClassMember> for SyntaxElement {
    fn from(n: JsStaticInitializationBlockClassMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsStaticMemberAssignment {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_STATIC_MEMBER_ASSIGNMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsStaticMemberAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsStaticMemberAssignment")
            .field("object", &support::DebugSyntaxResult(self.object()))
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("member", &support::DebugSyntaxResult(self.member()))
            .finish()
    }
}
impl From<JsStaticMemberAssignment> for SyntaxNode {
    fn from(n: JsStaticMemberAssignment) -> SyntaxNode { n.syntax }
}
impl From<JsStaticMemberAssignment> for SyntaxElement {
    fn from(n: JsStaticMemberAssignment) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsStaticMemberExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_STATIC_MEMBER_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsStaticMemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsStaticMemberExpression")
            .field("object", &support::DebugSyntaxResult(self.object()))
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .field("member", &support::DebugSyntaxResult(self.member()))
            .finish()
    }
}
impl From<JsStaticMemberExpression> for SyntaxNode {
    fn from(n: JsStaticMemberExpression) -> SyntaxNode { n.syntax }
}
impl From<JsStaticMemberExpression> for SyntaxElement {
    fn from(n: JsStaticMemberExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsStringLiteralExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_STRING_LITERAL_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsStringLiteralExpression")
            .field(
                "value_token",
                &support::DebugSyntaxResult(self.value_token()),
            )
            .finish()
    }
}
impl From<JsStringLiteralExpression> for SyntaxNode {
    fn from(n: JsStringLiteralExpression) -> SyntaxNode { n.syntax }
}
impl From<JsStringLiteralExpression> for SyntaxElement {
    fn from(n: JsStringLiteralExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsSuperExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SUPER_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSuperExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsSuperExpression")
            .field(
                "super_token",
                &support::DebugSyntaxResult(self.super_token()),
            )
            .finish()
    }
}
impl From<JsSuperExpression> for SyntaxNode {
    fn from(n: JsSuperExpression) -> SyntaxNode { n.syntax }
}
impl From<JsSuperExpression> for SyntaxElement {
    fn from(n: JsSuperExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsSwitchStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SWITCH_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsSwitchStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsSwitchStatement")
            .field(
                "switch_token",
                &support::DebugSyntaxResult(self.switch_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "discriminant",
                &support::DebugSyntaxResult(self.discriminant()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("cases", &self.cases())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsSwitchStatement> for SyntaxNode {
    fn from(n: JsSwitchStatement) -> SyntaxNode { n.syntax }
}
impl From<JsSwitchStatement> for SyntaxElement {
    fn from(n: JsSwitchStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsTemplate {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_TEMPLATE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsTemplate")
            .field("tag", &support::DebugOptionalElement(self.tag()))
            .field(
                "type_arguments",
                &support::DebugOptionalElement(self.type_arguments()),
            )
            .field(
                "l_tick_token",
                &support::DebugSyntaxResult(self.l_tick_token()),
            )
            .field("elements", &self.elements())
            .field(
                "r_tick_token",
                &support::DebugSyntaxResult(self.r_tick_token()),
            )
            .finish()
    }
}
impl From<JsTemplate> for SyntaxNode {
    fn from(n: JsTemplate) -> SyntaxNode { n.syntax }
}
impl From<JsTemplate> for SyntaxElement {
    fn from(n: JsTemplate) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsTemplateChunkElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_TEMPLATE_CHUNK_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsTemplateChunkElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsTemplateChunkElement")
            .field(
                "template_chunk_token",
                &support::DebugSyntaxResult(self.template_chunk_token()),
            )
            .finish()
    }
}
impl From<JsTemplateChunkElement> for SyntaxNode {
    fn from(n: JsTemplateChunkElement) -> SyntaxNode { n.syntax }
}
impl From<JsTemplateChunkElement> for SyntaxElement {
    fn from(n: JsTemplateChunkElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsTemplateElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_TEMPLATE_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsTemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsTemplateElement")
            .field(
                "dollar_curly_token",
                &support::DebugSyntaxResult(self.dollar_curly_token()),
            )
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<JsTemplateElement> for SyntaxNode {
    fn from(n: JsTemplateElement) -> SyntaxNode { n.syntax }
}
impl From<JsTemplateElement> for SyntaxElement {
    fn from(n: JsTemplateElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsThisExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_THIS_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsThisExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsThisExpression")
            .field("this_token", &support::DebugSyntaxResult(self.this_token()))
            .finish()
    }
}
impl From<JsThisExpression> for SyntaxNode {
    fn from(n: JsThisExpression) -> SyntaxNode { n.syntax }
}
impl From<JsThisExpression> for SyntaxElement {
    fn from(n: JsThisExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsThrowStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_THROW_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsThrowStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsThrowStatement")
            .field(
                "throw_token",
                &support::DebugSyntaxResult(self.throw_token()),
            )
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsThrowStatement> for SyntaxNode {
    fn from(n: JsThrowStatement) -> SyntaxNode { n.syntax }
}
impl From<JsThrowStatement> for SyntaxElement {
    fn from(n: JsThrowStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsTryFinallyStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_TRY_FINALLY_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsTryFinallyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsTryFinallyStatement")
            .field("try_token", &support::DebugSyntaxResult(self.try_token()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .field(
                "catch_clause",
                &support::DebugOptionalElement(self.catch_clause()),
            )
            .field(
                "finally_clause",
                &support::DebugSyntaxResult(self.finally_clause()),
            )
            .finish()
    }
}
impl From<JsTryFinallyStatement> for SyntaxNode {
    fn from(n: JsTryFinallyStatement) -> SyntaxNode { n.syntax }
}
impl From<JsTryFinallyStatement> for SyntaxElement {
    fn from(n: JsTryFinallyStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsTryStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_TRY_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsTryStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsTryStatement")
            .field("try_token", &support::DebugSyntaxResult(self.try_token()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .field(
                "catch_clause",
                &support::DebugSyntaxResult(self.catch_clause()),
            )
            .finish()
    }
}
impl From<JsTryStatement> for SyntaxNode {
    fn from(n: JsTryStatement) -> SyntaxNode { n.syntax }
}
impl From<JsTryStatement> for SyntaxElement {
    fn from(n: JsTryStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsUnaryExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNARY_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnaryExpression")
            .field("operator", &support::DebugSyntaxResult(self.operator()))
            .field("argument", &support::DebugSyntaxResult(self.argument()))
            .finish()
    }
}
impl From<JsUnaryExpression> for SyntaxNode {
    fn from(n: JsUnaryExpression) -> SyntaxNode { n.syntax }
}
impl From<JsUnaryExpression> for SyntaxElement {
    fn from(n: JsUnaryExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsVariableDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsVariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsVariableDeclaration")
            .field("kind", &support::DebugSyntaxResult(self.kind()))
            .field("declarators", &self.declarators())
            .finish()
    }
}
impl From<JsVariableDeclaration> for SyntaxNode {
    fn from(n: JsVariableDeclaration) -> SyntaxNode { n.syntax }
}
impl From<JsVariableDeclaration> for SyntaxElement {
    fn from(n: JsVariableDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsVariableDeclarationClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATION_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsVariableDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsVariableDeclarationClause")
            .field(
                "declaration",
                &support::DebugSyntaxResult(self.declaration()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsVariableDeclarationClause> for SyntaxNode {
    fn from(n: JsVariableDeclarationClause) -> SyntaxNode { n.syntax }
}
impl From<JsVariableDeclarationClause> for SyntaxElement {
    fn from(n: JsVariableDeclarationClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsVariableDeclarator {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATOR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsVariableDeclarator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsVariableDeclarator")
            .field("id", &support::DebugSyntaxResult(self.id()))
            .field(
                "variable_annotation",
                &support::DebugOptionalElement(self.variable_annotation()),
            )
            .field(
                "initializer",
                &support::DebugOptionalElement(self.initializer()),
            )
            .finish()
    }
}
impl From<JsVariableDeclarator> for SyntaxNode {
    fn from(n: JsVariableDeclarator) -> SyntaxNode { n.syntax }
}
impl From<JsVariableDeclarator> for SyntaxElement {
    fn from(n: JsVariableDeclarator) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsVariableStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_VARIABLE_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsVariableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsVariableStatement")
            .field(
                "declaration",
                &support::DebugSyntaxResult(self.declaration()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<JsVariableStatement> for SyntaxNode {
    fn from(n: JsVariableStatement) -> SyntaxNode { n.syntax }
}
impl From<JsVariableStatement> for SyntaxElement {
    fn from(n: JsVariableStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsWhileStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_WHILE_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsWhileStatement")
            .field(
                "while_token",
                &support::DebugSyntaxResult(self.while_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("test", &support::DebugSyntaxResult(self.test()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsWhileStatement> for SyntaxNode {
    fn from(n: JsWhileStatement) -> SyntaxNode { n.syntax }
}
impl From<JsWhileStatement> for SyntaxElement {
    fn from(n: JsWhileStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsWithStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_WITH_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsWithStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsWithStatement")
            .field("with_token", &support::DebugSyntaxResult(self.with_token()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("object", &support::DebugSyntaxResult(self.object()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<JsWithStatement> for SyntaxNode {
    fn from(n: JsWithStatement) -> SyntaxNode { n.syntax }
}
impl From<JsWithStatement> for SyntaxElement {
    fn from(n: JsWithStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsYieldArgument {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_YIELD_ARGUMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsYieldArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsYieldArgument")
            .field(
                "star_token",
                &support::DebugOptionalElement(self.star_token()),
            )
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .finish()
    }
}
impl From<JsYieldArgument> for SyntaxNode {
    fn from(n: JsYieldArgument) -> SyntaxNode { n.syntax }
}
impl From<JsYieldArgument> for SyntaxElement {
    fn from(n: JsYieldArgument) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for JsYieldExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_YIELD_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsYieldExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsYieldExpression")
            .field(
                "yield_token",
                &support::DebugSyntaxResult(self.yield_token()),
            )
            .field("argument", &support::DebugOptionalElement(self.argument()))
            .finish()
    }
}
impl From<JsYieldExpression> for SyntaxNode {
    fn from(n: JsYieldExpression) -> SyntaxNode { n.syntax }
}
impl From<JsYieldExpression> for SyntaxElement {
    fn from(n: JsYieldExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for NewTarget {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == NEW_TARGET }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for NewTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewTarget")
            .field("new_token", &support::DebugSyntaxResult(self.new_token()))
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field(
                "target_token",
                &support::DebugSyntaxResult(self.target_token()),
            )
            .finish()
    }
}
impl From<NewTarget> for SyntaxNode {
    fn from(n: NewTarget) -> SyntaxNode { n.syntax }
}
impl From<NewTarget> for SyntaxElement {
    fn from(n: NewTarget) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsAnyType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_ANY_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsAnyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsAnyType")
            .field("any_token", &support::DebugSyntaxResult(self.any_token()))
            .finish()
    }
}
impl From<TsAnyType> for SyntaxNode {
    fn from(n: TsAnyType) -> SyntaxNode { n.syntax }
}
impl From<TsAnyType> for SyntaxElement {
    fn from(n: TsAnyType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsArrayType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_ARRAY_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsArrayType")
            .field(
                "element_type",
                &support::DebugSyntaxResult(self.element_type()),
            )
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<TsArrayType> for SyntaxNode {
    fn from(n: TsArrayType) -> SyntaxNode { n.syntax }
}
impl From<TsArrayType> for SyntaxElement {
    fn from(n: TsArrayType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsAsExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_AS_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsAsExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsAsExpression")
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field("as_token", &support::DebugSyntaxResult(self.as_token()))
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsAsExpression> for SyntaxNode {
    fn from(n: TsAsExpression) -> SyntaxNode { n.syntax }
}
impl From<TsAsExpression> for SyntaxElement {
    fn from(n: TsAsExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsBigIntLiteralType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_BIG_INT_LITERAL_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsBigIntLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsBigIntLiteralType")
            .field(
                "minus_token",
                &support::DebugOptionalElement(self.minus_token()),
            )
            .field(
                "literal_token",
                &support::DebugSyntaxResult(self.literal_token()),
            )
            .finish()
    }
}
impl From<TsBigIntLiteralType> for SyntaxNode {
    fn from(n: TsBigIntLiteralType) -> SyntaxNode { n.syntax }
}
impl From<TsBigIntLiteralType> for SyntaxElement {
    fn from(n: TsBigIntLiteralType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsBigintType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_BIGINT_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsBigintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsBigintType")
            .field(
                "bigint_token",
                &support::DebugSyntaxResult(self.bigint_token()),
            )
            .finish()
    }
}
impl From<TsBigintType> for SyntaxNode {
    fn from(n: TsBigintType) -> SyntaxNode { n.syntax }
}
impl From<TsBigintType> for SyntaxElement {
    fn from(n: TsBigintType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsBooleanLiteralType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_BOOLEAN_LITERAL_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsBooleanLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsBooleanLiteralType")
            .field("literal", &support::DebugSyntaxResult(self.literal()))
            .finish()
    }
}
impl From<TsBooleanLiteralType> for SyntaxNode {
    fn from(n: TsBooleanLiteralType) -> SyntaxNode { n.syntax }
}
impl From<TsBooleanLiteralType> for SyntaxElement {
    fn from(n: TsBooleanLiteralType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsBooleanType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_BOOLEAN_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsBooleanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsBooleanType")
            .field(
                "boolean_token",
                &support::DebugSyntaxResult(self.boolean_token()),
            )
            .finish()
    }
}
impl From<TsBooleanType> for SyntaxNode {
    fn from(n: TsBooleanType) -> SyntaxNode { n.syntax }
}
impl From<TsBooleanType> for SyntaxElement {
    fn from(n: TsBooleanType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsCallSignatureTypeMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_CALL_SIGNATURE_TYPE_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsCallSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsCallSignatureTypeMember")
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field(
                "separator_token",
                &support::DebugOptionalElement(self.separator_token()),
            )
            .finish()
    }
}
impl From<TsCallSignatureTypeMember> for SyntaxNode {
    fn from(n: TsCallSignatureTypeMember) -> SyntaxNode { n.syntax }
}
impl From<TsCallSignatureTypeMember> for SyntaxElement {
    fn from(n: TsCallSignatureTypeMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsConditionalType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_CONDITIONAL_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConditionalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsConditionalType")
            .field("check_type", &support::DebugSyntaxResult(self.check_type()))
            .field(
                "extends_token",
                &support::DebugSyntaxResult(self.extends_token()),
            )
            .field(
                "extends_type",
                &support::DebugSyntaxResult(self.extends_type()),
            )
            .field(
                "question_mark_token",
                &support::DebugSyntaxResult(self.question_mark_token()),
            )
            .field("true_type", &support::DebugSyntaxResult(self.true_type()))
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("false_type", &support::DebugSyntaxResult(self.false_type()))
            .finish()
    }
}
impl From<TsConditionalType> for SyntaxNode {
    fn from(n: TsConditionalType) -> SyntaxNode { n.syntax }
}
impl From<TsConditionalType> for SyntaxElement {
    fn from(n: TsConditionalType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsConstructSignatureTypeMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstructSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsConstructSignatureTypeMember")
            .field("new_token", &support::DebugSyntaxResult(self.new_token()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "type_annotation",
                &support::DebugOptionalElement(self.type_annotation()),
            )
            .field(
                "separator_token",
                &support::DebugOptionalElement(self.separator_token()),
            )
            .finish()
    }
}
impl From<TsConstructSignatureTypeMember> for SyntaxNode {
    fn from(n: TsConstructSignatureTypeMember) -> SyntaxNode { n.syntax }
}
impl From<TsConstructSignatureTypeMember> for SyntaxElement {
    fn from(n: TsConstructSignatureTypeMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsConstructorType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_CONSTRUCTOR_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstructorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsConstructorType")
            .field(
                "abstract_token",
                &support::DebugOptionalElement(self.abstract_token()),
            )
            .field("new_token", &support::DebugSyntaxResult(self.new_token()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "fat_arrow_token",
                &support::DebugSyntaxResult(self.fat_arrow_token()),
            )
            .field(
                "return_type",
                &support::DebugSyntaxResult(self.return_type()),
            )
            .finish()
    }
}
impl From<TsConstructorType> for SyntaxNode {
    fn from(n: TsConstructorType) -> SyntaxNode { n.syntax }
}
impl From<TsConstructorType> for SyntaxElement {
    fn from(n: TsConstructorType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsDeclareFunctionDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_DECLARE_FUNCTION_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsDeclareFunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsDeclareFunctionDeclaration")
            .field(
                "async_token",
                &support::DebugOptionalElement(self.async_token()),
            )
            .field(
                "function_token",
                &support::DebugSyntaxResult(self.function_token()),
            )
            .field("id", &support::DebugSyntaxResult(self.id()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<TsDeclareFunctionDeclaration> for SyntaxNode {
    fn from(n: TsDeclareFunctionDeclaration) -> SyntaxNode { n.syntax }
}
impl From<TsDeclareFunctionDeclaration> for SyntaxElement {
    fn from(n: TsDeclareFunctionDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsDeclareStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_DECLARE_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsDeclareStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsDeclareStatement")
            .field(
                "declare_token",
                &support::DebugSyntaxResult(self.declare_token()),
            )
            .field(
                "declaration",
                &support::DebugSyntaxResult(self.declaration()),
            )
            .finish()
    }
}
impl From<TsDeclareStatement> for SyntaxNode {
    fn from(n: TsDeclareStatement) -> SyntaxNode { n.syntax }
}
impl From<TsDeclareStatement> for SyntaxElement {
    fn from(n: TsDeclareStatement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsDefaultTypeClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_DEFAULT_TYPE_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsDefaultTypeClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsDefaultTypeClause")
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsDefaultTypeClause> for SyntaxNode {
    fn from(n: TsDefaultTypeClause) -> SyntaxNode { n.syntax }
}
impl From<TsDefaultTypeClause> for SyntaxElement {
    fn from(n: TsDefaultTypeClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsDefinitePropertyAnnotation {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_DEFINITE_PROPERTY_ANNOTATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsDefinitePropertyAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsDefinitePropertyAnnotation")
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .field(
                "type_annotation",
                &support::DebugSyntaxResult(self.type_annotation()),
            )
            .finish()
    }
}
impl From<TsDefinitePropertyAnnotation> for SyntaxNode {
    fn from(n: TsDefinitePropertyAnnotation) -> SyntaxNode { n.syntax }
}
impl From<TsDefinitePropertyAnnotation> for SyntaxElement {
    fn from(n: TsDefinitePropertyAnnotation) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsDefiniteVariableAnnotation {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_DEFINITE_VARIABLE_ANNOTATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsDefiniteVariableAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsDefiniteVariableAnnotation")
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .field(
                "type_annotation",
                &support::DebugSyntaxResult(self.type_annotation()),
            )
            .finish()
    }
}
impl From<TsDefiniteVariableAnnotation> for SyntaxNode {
    fn from(n: TsDefiniteVariableAnnotation) -> SyntaxNode { n.syntax }
}
impl From<TsDefiniteVariableAnnotation> for SyntaxElement {
    fn from(n: TsDefiniteVariableAnnotation) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsEmptyExternalModuleDeclarationBody {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsEmptyExternalModuleDeclarationBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsEmptyExternalModuleDeclarationBody")
            .field(
                "semicolon_token",
                &support::DebugSyntaxResult(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<TsEmptyExternalModuleDeclarationBody> for SyntaxNode {
    fn from(n: TsEmptyExternalModuleDeclarationBody) -> SyntaxNode { n.syntax }
}
impl From<TsEmptyExternalModuleDeclarationBody> for SyntaxElement {
    fn from(n: TsEmptyExternalModuleDeclarationBody) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsEnumDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_ENUM_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsEnumDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsEnumDeclaration")
            .field(
                "const_token",
                &support::DebugOptionalElement(self.const_token()),
            )
            .field("enum_token", &support::DebugSyntaxResult(self.enum_token()))
            .field("id", &support::DebugSyntaxResult(self.id()))
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("members", &self.members())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<TsEnumDeclaration> for SyntaxNode {
    fn from(n: TsEnumDeclaration) -> SyntaxNode { n.syntax }
}
impl From<TsEnumDeclaration> for SyntaxElement {
    fn from(n: TsEnumDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsEnumMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_ENUM_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsEnumMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsEnumMember")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "initializer",
                &support::DebugOptionalElement(self.initializer()),
            )
            .finish()
    }
}
impl From<TsEnumMember> for SyntaxNode {
    fn from(n: TsEnumMember) -> SyntaxNode { n.syntax }
}
impl From<TsEnumMember> for SyntaxElement {
    fn from(n: TsEnumMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsExtendsClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_EXTENDS_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExtendsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsExtendsClause")
            .field(
                "extends_token",
                &support::DebugSyntaxResult(self.extends_token()),
            )
            .field("types", &self.types())
            .finish()
    }
}
impl From<TsExtendsClause> for SyntaxNode {
    fn from(n: TsExtendsClause) -> SyntaxNode { n.syntax }
}
impl From<TsExtendsClause> for SyntaxElement {
    fn from(n: TsExtendsClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsExternalModuleDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_EXTERNAL_MODULE_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExternalModuleDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsExternalModuleDeclaration")
            .field(
                "module_token",
                &support::DebugSyntaxResult(self.module_token()),
            )
            .field("source", &support::DebugSyntaxResult(self.source()))
            .field("body", &support::DebugOptionalElement(self.body()))
            .finish()
    }
}
impl From<TsExternalModuleDeclaration> for SyntaxNode {
    fn from(n: TsExternalModuleDeclaration) -> SyntaxNode { n.syntax }
}
impl From<TsExternalModuleDeclaration> for SyntaxElement {
    fn from(n: TsExternalModuleDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsExternalModuleRef {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_EXTERNAL_MODULE_REF }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExternalModuleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsExternalModuleRef")
            .field(
                "require_token",
                &support::DebugSyntaxResult(self.require_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "module_token",
                &support::DebugSyntaxResult(self.module_token()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<TsExternalModuleRef> for SyntaxNode {
    fn from(n: TsExternalModuleRef) -> SyntaxNode { n.syntax }
}
impl From<TsExternalModuleRef> for SyntaxElement {
    fn from(n: TsExternalModuleRef) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsFunctionType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_FUNCTION_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsFunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsFunctionType")
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "fat_arrow_token",
                &support::DebugSyntaxResult(self.fat_arrow_token()),
            )
            .field(
                "return_type",
                &support::DebugSyntaxResult(self.return_type()),
            )
            .finish()
    }
}
impl From<TsFunctionType> for SyntaxNode {
    fn from(n: TsFunctionType) -> SyntaxNode { n.syntax }
}
impl From<TsFunctionType> for SyntaxElement {
    fn from(n: TsFunctionType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsGetterSignatureTypeMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_GETTER_SIGNATURE_TYPE_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsGetterSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsGetterSignatureTypeMember")
            .field("get_token", &support::DebugSyntaxResult(self.get_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "type_annotation",
                &support::DebugOptionalElement(self.type_annotation()),
            )
            .field(
                "separator_token",
                &support::DebugOptionalElement(self.separator_token()),
            )
            .finish()
    }
}
impl From<TsGetterSignatureTypeMember> for SyntaxNode {
    fn from(n: TsGetterSignatureTypeMember) -> SyntaxNode { n.syntax }
}
impl From<TsGetterSignatureTypeMember> for SyntaxElement {
    fn from(n: TsGetterSignatureTypeMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsGlobalDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_GLOBAL_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsGlobalDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsGlobalDeclaration")
            .field(
                "global_token",
                &support::DebugSyntaxResult(self.global_token()),
            )
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<TsGlobalDeclaration> for SyntaxNode {
    fn from(n: TsGlobalDeclaration) -> SyntaxNode { n.syntax }
}
impl From<TsGlobalDeclaration> for SyntaxElement {
    fn from(n: TsGlobalDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsIdentifierBinding {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_IDENTIFIER_BINDING }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIdentifierBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsIdentifierBinding")
            .field("name_token", &support::DebugSyntaxResult(self.name_token()))
            .finish()
    }
}
impl From<TsIdentifierBinding> for SyntaxNode {
    fn from(n: TsIdentifierBinding) -> SyntaxNode { n.syntax }
}
impl From<TsIdentifierBinding> for SyntaxElement {
    fn from(n: TsIdentifierBinding) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsImplementsClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_IMPLEMENTS_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsImplementsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsImplementsClause")
            .field(
                "implements_token",
                &support::DebugSyntaxResult(self.implements_token()),
            )
            .field("types", &self.types())
            .finish()
    }
}
impl From<TsImplementsClause> for SyntaxNode {
    fn from(n: TsImplementsClause) -> SyntaxNode { n.syntax }
}
impl From<TsImplementsClause> for SyntaxElement {
    fn from(n: TsImplementsClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsImportEqualsDecl {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_IMPORT_EQUALS_DECL }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsImportEqualsDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsImportEqualsDecl")
            .field(
                "import_token",
                &support::DebugSyntaxResult(self.import_token()),
            )
            .field(
                "export_token",
                &support::DebugSyntaxResult(self.export_token()),
            )
            .field(
                "ident_token",
                &support::DebugSyntaxResult(self.ident_token()),
            )
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("module", &support::DebugSyntaxResult(self.module()))
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<TsImportEqualsDecl> for SyntaxNode {
    fn from(n: TsImportEqualsDecl) -> SyntaxNode { n.syntax }
}
impl From<TsImportEqualsDecl> for SyntaxElement {
    fn from(n: TsImportEqualsDecl) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsImportType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_IMPORT_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsImportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsImportType")
            .field(
                "typeof_token",
                &support::DebugOptionalElement(self.typeof_token()),
            )
            .field(
                "import_token",
                &support::DebugSyntaxResult(self.import_token()),
            )
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field(
                "argument_token",
                &support::DebugSyntaxResult(self.argument_token()),
            )
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "qualifier_clause",
                &support::DebugOptionalElement(self.qualifier_clause()),
            )
            .finish()
    }
}
impl From<TsImportType> for SyntaxNode {
    fn from(n: TsImportType) -> SyntaxNode { n.syntax }
}
impl From<TsImportType> for SyntaxElement {
    fn from(n: TsImportType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsImportTypeQualifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_IMPORT_TYPE_QUALIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsImportTypeQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsImportTypeQualifier")
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<TsImportTypeQualifier> for SyntaxNode {
    fn from(n: TsImportTypeQualifier) -> SyntaxNode { n.syntax }
}
impl From<TsImportTypeQualifier> for SyntaxElement {
    fn from(n: TsImportTypeQualifier) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsIndexSignatureParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_INDEX_SIGNATURE_PARAMETER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIndexSignatureParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsIndexSignatureParameter")
            .field("binding", &support::DebugSyntaxResult(self.binding()))
            .field(
                "type_annotation",
                &support::DebugSyntaxResult(self.type_annotation()),
            )
            .finish()
    }
}
impl From<TsIndexSignatureParameter> for SyntaxNode {
    fn from(n: TsIndexSignatureParameter) -> SyntaxNode { n.syntax }
}
impl From<TsIndexSignatureParameter> for SyntaxElement {
    fn from(n: TsIndexSignatureParameter) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsIndexSignatureTypeMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_INDEX_SIGNATURE_TYPE_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIndexSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsIndexSignatureTypeMember")
            .field(
                "readonly_token",
                &support::DebugOptionalElement(self.readonly_token()),
            )
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("parameter", &support::DebugSyntaxResult(self.parameter()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .field(
                "type_annotation",
                &support::DebugSyntaxResult(self.type_annotation()),
            )
            .field(
                "separator_token",
                &support::DebugOptionalElement(self.separator_token()),
            )
            .finish()
    }
}
impl From<TsIndexSignatureTypeMember> for SyntaxNode {
    fn from(n: TsIndexSignatureTypeMember) -> SyntaxNode { n.syntax }
}
impl From<TsIndexSignatureTypeMember> for SyntaxElement {
    fn from(n: TsIndexSignatureTypeMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsIndexedAccessType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_INDEXED_ACCESS_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIndexedAccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsIndexedAccessType")
            .field(
                "object_type",
                &support::DebugSyntaxResult(self.object_type()),
            )
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field("index_type", &support::DebugSyntaxResult(self.index_type()))
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .finish()
    }
}
impl From<TsIndexedAccessType> for SyntaxNode {
    fn from(n: TsIndexedAccessType) -> SyntaxNode { n.syntax }
}
impl From<TsIndexedAccessType> for SyntaxElement {
    fn from(n: TsIndexedAccessType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsInferType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_INFER_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsInferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsInferType")
            .field(
                "infer_token",
                &support::DebugSyntaxResult(self.infer_token()),
            )
            .field(
                "type_parameter",
                &support::DebugSyntaxResult(self.type_parameter()),
            )
            .finish()
    }
}
impl From<TsInferType> for SyntaxNode {
    fn from(n: TsInferType) -> SyntaxNode { n.syntax }
}
impl From<TsInferType> for SyntaxElement {
    fn from(n: TsInferType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsInterfaceDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_INTERFACE_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsInterfaceDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsInterfaceDeclaration")
            .field(
                "interface_token",
                &support::DebugSyntaxResult(self.interface_token()),
            )
            .field("id", &support::DebugSyntaxResult(self.id()))
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field(
                "extends_clause",
                &support::DebugOptionalElement(self.extends_clause()),
            )
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("members", &self.members())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<TsInterfaceDeclaration> for SyntaxNode {
    fn from(n: TsInterfaceDeclaration) -> SyntaxNode { n.syntax }
}
impl From<TsInterfaceDeclaration> for SyntaxElement {
    fn from(n: TsInterfaceDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsIntersectionType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_INTERSECTION_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIntersectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsIntersectionType")
            .field(
                "leading_separator_token_token",
                &support::DebugOptionalElement(self.leading_separator_token_token()),
            )
            .field("types", &self.types())
            .finish()
    }
}
impl From<TsIntersectionType> for SyntaxNode {
    fn from(n: TsIntersectionType) -> SyntaxNode { n.syntax }
}
impl From<TsIntersectionType> for SyntaxElement {
    fn from(n: TsIntersectionType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsMappedType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_MAPPED_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsMappedType")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field(
                "readonly_modifier",
                &support::DebugOptionalElement(self.readonly_modifier()),
            )
            .field(
                "l_brack_token",
                &support::DebugSyntaxResult(self.l_brack_token()),
            )
            .field(
                "property_name",
                &support::DebugSyntaxResult(self.property_name()),
            )
            .field("in_token", &support::DebugSyntaxResult(self.in_token()))
            .field("keys_type", &support::DebugSyntaxResult(self.keys_type()))
            .field(
                "as_clause",
                &support::DebugOptionalElement(self.as_clause()),
            )
            .field(
                "r_brack_token",
                &support::DebugSyntaxResult(self.r_brack_token()),
            )
            .field(
                "optional_modifier",
                &support::DebugOptionalElement(self.optional_modifier()),
            )
            .field(
                "mapped_type",
                &support::DebugOptionalElement(self.mapped_type()),
            )
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<TsMappedType> for SyntaxNode {
    fn from(n: TsMappedType) -> SyntaxNode { n.syntax }
}
impl From<TsMappedType> for SyntaxElement {
    fn from(n: TsMappedType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsMappedTypeAsClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_MAPPED_TYPE_AS_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedTypeAsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsMappedTypeAsClause")
            .field("as_token", &support::DebugSyntaxResult(self.as_token()))
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsMappedTypeAsClause> for SyntaxNode {
    fn from(n: TsMappedTypeAsClause) -> SyntaxNode { n.syntax }
}
impl From<TsMappedTypeAsClause> for SyntaxElement {
    fn from(n: TsMappedTypeAsClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsMappedTypeOptionalModifierClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedTypeOptionalModifierClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsMappedTypeOptionalModifierClause")
            .field(
                "operator_token",
                &support::DebugSyntaxResult(self.operator_token()),
            )
            .field(
                "question_mark_token",
                &support::DebugSyntaxResult(self.question_mark_token()),
            )
            .finish()
    }
}
impl From<TsMappedTypeOptionalModifierClause> for SyntaxNode {
    fn from(n: TsMappedTypeOptionalModifierClause) -> SyntaxNode { n.syntax }
}
impl From<TsMappedTypeOptionalModifierClause> for SyntaxElement {
    fn from(n: TsMappedTypeOptionalModifierClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsMappedTypeReadonlyModifierClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedTypeReadonlyModifierClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsMappedTypeReadonlyModifierClause")
            .field(
                "operator_token",
                &support::DebugSyntaxResult(self.operator_token()),
            )
            .field(
                "readonly_token",
                &support::DebugSyntaxResult(self.readonly_token()),
            )
            .finish()
    }
}
impl From<TsMappedTypeReadonlyModifierClause> for SyntaxNode {
    fn from(n: TsMappedTypeReadonlyModifierClause) -> SyntaxNode { n.syntax }
}
impl From<TsMappedTypeReadonlyModifierClause> for SyntaxElement {
    fn from(n: TsMappedTypeReadonlyModifierClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsMethodSignatureTypeMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_METHOD_SIGNATURE_TYPE_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMethodSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsMethodSignatureTypeMember")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "optional_token",
                &support::DebugOptionalElement(self.optional_token()),
            )
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("parameters", &support::DebugSyntaxResult(self.parameters()))
            .field(
                "return_type_annotation",
                &support::DebugOptionalElement(self.return_type_annotation()),
            )
            .field(
                "separator_token",
                &support::DebugOptionalElement(self.separator_token()),
            )
            .finish()
    }
}
impl From<TsMethodSignatureTypeMember> for SyntaxNode {
    fn from(n: TsMethodSignatureTypeMember) -> SyntaxNode { n.syntax }
}
impl From<TsMethodSignatureTypeMember> for SyntaxElement {
    fn from(n: TsMethodSignatureTypeMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsModuleBlock {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_MODULE_BLOCK }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsModuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsModuleBlock")
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
impl From<TsModuleBlock> for SyntaxNode {
    fn from(n: TsModuleBlock) -> SyntaxNode { n.syntax }
}
impl From<TsModuleBlock> for SyntaxElement {
    fn from(n: TsModuleBlock) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsModuleDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_MODULE_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsModuleDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsModuleDeclaration")
            .field(
                "module_or_namespace",
                &support::DebugSyntaxResult(self.module_or_namespace()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field("body", &support::DebugSyntaxResult(self.body()))
            .finish()
    }
}
impl From<TsModuleDeclaration> for SyntaxNode {
    fn from(n: TsModuleDeclaration) -> SyntaxNode { n.syntax }
}
impl From<TsModuleDeclaration> for SyntaxElement {
    fn from(n: TsModuleDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNameWithTypeArguments {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NAME_WITH_TYPE_ARGUMENTS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNameWithTypeArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNameWithTypeArguments")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "type_arguments",
                &support::DebugOptionalElement(self.type_arguments()),
            )
            .finish()
    }
}
impl From<TsNameWithTypeArguments> for SyntaxNode {
    fn from(n: TsNameWithTypeArguments) -> SyntaxNode { n.syntax }
}
impl From<TsNameWithTypeArguments> for SyntaxElement {
    fn from(n: TsNameWithTypeArguments) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNamedTupleTypeElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NAMED_TUPLE_TYPE_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNamedTupleTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNamedTupleTypeElement")
            .field(
                "dotdotdot_token",
                &support::DebugOptionalElement(self.dotdotdot_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "question_mark_token",
                &support::DebugOptionalElement(self.question_mark_token()),
            )
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsNamedTupleTypeElement> for SyntaxNode {
    fn from(n: TsNamedTupleTypeElement) -> SyntaxNode { n.syntax }
}
impl From<TsNamedTupleTypeElement> for SyntaxElement {
    fn from(n: TsNamedTupleTypeElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNeverType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NEVER_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNeverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNeverType")
            .field(
                "never_token",
                &support::DebugSyntaxResult(self.never_token()),
            )
            .finish()
    }
}
impl From<TsNeverType> for SyntaxNode {
    fn from(n: TsNeverType) -> SyntaxNode { n.syntax }
}
impl From<TsNeverType> for SyntaxElement {
    fn from(n: TsNeverType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNonNullAssertionAssignment {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NON_NULL_ASSERTION_ASSIGNMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNonNullAssertionAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNonNullAssertionAssignment")
            .field("assignment", &support::DebugSyntaxResult(self.assignment()))
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .finish()
    }
}
impl From<TsNonNullAssertionAssignment> for SyntaxNode {
    fn from(n: TsNonNullAssertionAssignment) -> SyntaxNode { n.syntax }
}
impl From<TsNonNullAssertionAssignment> for SyntaxElement {
    fn from(n: TsNonNullAssertionAssignment) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNonNullAssertionExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NON_NULL_ASSERTION_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNonNullAssertionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNonNullAssertionExpression")
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
            .finish()
    }
}
impl From<TsNonNullAssertionExpression> for SyntaxNode {
    fn from(n: TsNonNullAssertionExpression) -> SyntaxNode { n.syntax }
}
impl From<TsNonNullAssertionExpression> for SyntaxElement {
    fn from(n: TsNonNullAssertionExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNonPrimitiveType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NON_PRIMITIVE_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNonPrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNonPrimitiveType")
            .field(
                "object_token",
                &support::DebugSyntaxResult(self.object_token()),
            )
            .finish()
    }
}
impl From<TsNonPrimitiveType> for SyntaxNode {
    fn from(n: TsNonPrimitiveType) -> SyntaxNode { n.syntax }
}
impl From<TsNonPrimitiveType> for SyntaxElement {
    fn from(n: TsNonPrimitiveType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNullLiteralType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NULL_LITERAL_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNullLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNullLiteralType")
            .field(
                "literal_token",
                &support::DebugSyntaxResult(self.literal_token()),
            )
            .finish()
    }
}
impl From<TsNullLiteralType> for SyntaxNode {
    fn from(n: TsNullLiteralType) -> SyntaxNode { n.syntax }
}
impl From<TsNullLiteralType> for SyntaxElement {
    fn from(n: TsNullLiteralType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNumberLiteralType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NUMBER_LITERAL_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNumberLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNumberLiteralType")
            .field(
                "minus_token",
                &support::DebugOptionalElement(self.minus_token()),
            )
            .field(
                "literal_token",
                &support::DebugSyntaxResult(self.literal_token()),
            )
            .finish()
    }
}
impl From<TsNumberLiteralType> for SyntaxNode {
    fn from(n: TsNumberLiteralType) -> SyntaxNode { n.syntax }
}
impl From<TsNumberLiteralType> for SyntaxElement {
    fn from(n: TsNumberLiteralType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsNumberType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_NUMBER_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsNumberType")
            .field(
                "number_token",
                &support::DebugSyntaxResult(self.number_token()),
            )
            .finish()
    }
}
impl From<TsNumberType> for SyntaxNode {
    fn from(n: TsNumberType) -> SyntaxNode { n.syntax }
}
impl From<TsNumberType> for SyntaxElement {
    fn from(n: TsNumberType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsObjectType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_OBJECT_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsObjectType")
            .field(
                "l_curly_token",
                &support::DebugSyntaxResult(self.l_curly_token()),
            )
            .field("members", &self.members())
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<TsObjectType> for SyntaxNode {
    fn from(n: TsObjectType) -> SyntaxNode { n.syntax }
}
impl From<TsObjectType> for SyntaxElement {
    fn from(n: TsObjectType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsOptionalPropertyAnnotation {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_OPTIONAL_PROPERTY_ANNOTATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsOptionalPropertyAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsOptionalPropertyAnnotation")
            .field(
                "question_mark_token",
                &support::DebugSyntaxResult(self.question_mark_token()),
            )
            .field(
                "type_annotation",
                &support::DebugOptionalElement(self.type_annotation()),
            )
            .finish()
    }
}
impl From<TsOptionalPropertyAnnotation> for SyntaxNode {
    fn from(n: TsOptionalPropertyAnnotation) -> SyntaxNode { n.syntax }
}
impl From<TsOptionalPropertyAnnotation> for SyntaxElement {
    fn from(n: TsOptionalPropertyAnnotation) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsOptionalTupleTypeElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_OPTIONAL_TUPLE_TYPE_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsOptionalTupleTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsOptionalTupleTypeElement")
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .field(
                "question_mark_token",
                &support::DebugSyntaxResult(self.question_mark_token()),
            )
            .finish()
    }
}
impl From<TsOptionalTupleTypeElement> for SyntaxNode {
    fn from(n: TsOptionalTupleTypeElement) -> SyntaxNode { n.syntax }
}
impl From<TsOptionalTupleTypeElement> for SyntaxElement {
    fn from(n: TsOptionalTupleTypeElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsParenthesizedType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_PARENTHESIZED_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsParenthesizedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsParenthesizedType")
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .finish()
    }
}
impl From<TsParenthesizedType> for SyntaxNode {
    fn from(n: TsParenthesizedType) -> SyntaxNode { n.syntax }
}
impl From<TsParenthesizedType> for SyntaxElement {
    fn from(n: TsParenthesizedType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsPropertyParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_PROPERTY_PARAMETER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsPropertyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsPropertyParameter")
            .field(
                "accessibility",
                &support::DebugSyntaxResult(self.accessibility()),
            )
            .field(
                "formal_parameter",
                &support::DebugSyntaxResult(self.formal_parameter()),
            )
            .finish()
    }
}
impl From<TsPropertyParameter> for SyntaxNode {
    fn from(n: TsPropertyParameter) -> SyntaxNode { n.syntax }
}
impl From<TsPropertyParameter> for SyntaxElement {
    fn from(n: TsPropertyParameter) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsPropertySignatureTypeMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_PROPERTY_SIGNATURE_TYPE_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsPropertySignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsPropertySignatureTypeMember")
            .field(
                "readonly_token",
                &support::DebugOptionalElement(self.readonly_token()),
            )
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "optional_token",
                &support::DebugOptionalElement(self.optional_token()),
            )
            .field(
                "type_annotation",
                &support::DebugOptionalElement(self.type_annotation()),
            )
            .field(
                "separator_token",
                &support::DebugOptionalElement(self.separator_token()),
            )
            .finish()
    }
}
impl From<TsPropertySignatureTypeMember> for SyntaxNode {
    fn from(n: TsPropertySignatureTypeMember) -> SyntaxNode { n.syntax }
}
impl From<TsPropertySignatureTypeMember> for SyntaxElement {
    fn from(n: TsPropertySignatureTypeMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsQualifiedModuleName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_QUALIFIED_MODULE_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsQualifiedModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsQualifiedModuleName")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<TsQualifiedModuleName> for SyntaxNode {
    fn from(n: TsQualifiedModuleName) -> SyntaxNode { n.syntax }
}
impl From<TsQualifiedModuleName> for SyntaxElement {
    fn from(n: TsQualifiedModuleName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsQualifiedName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_QUALIFIED_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsQualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsQualifiedName")
            .field("left", &support::DebugSyntaxResult(self.left()))
            .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
            .field("right", &support::DebugSyntaxResult(self.right()))
            .finish()
    }
}
impl From<TsQualifiedName> for SyntaxNode {
    fn from(n: TsQualifiedName) -> SyntaxNode { n.syntax }
}
impl From<TsQualifiedName> for SyntaxElement {
    fn from(n: TsQualifiedName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsReadonlyPropertyParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_READONLY_PROPERTY_PARAMETER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsReadonlyPropertyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsReadonlyPropertyParameter")
            .field(
                "accessibility",
                &support::DebugOptionalElement(self.accessibility()),
            )
            .field(
                "readonly_token",
                &support::DebugSyntaxResult(self.readonly_token()),
            )
            .field(
                "formal_parameter",
                &support::DebugSyntaxResult(self.formal_parameter()),
            )
            .finish()
    }
}
impl From<TsReadonlyPropertyParameter> for SyntaxNode {
    fn from(n: TsReadonlyPropertyParameter) -> SyntaxNode { n.syntax }
}
impl From<TsReadonlyPropertyParameter> for SyntaxElement {
    fn from(n: TsReadonlyPropertyParameter) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsReferenceType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_REFERENCE_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsReferenceType")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "type_arguments",
                &support::DebugOptionalElement(self.type_arguments()),
            )
            .finish()
    }
}
impl From<TsReferenceType> for SyntaxNode {
    fn from(n: TsReferenceType) -> SyntaxNode { n.syntax }
}
impl From<TsReferenceType> for SyntaxElement {
    fn from(n: TsReferenceType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsRestTupleTypeElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_REST_TUPLE_TYPE_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsRestTupleTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsRestTupleTypeElement")
            .field(
                "dotdotdot_token",
                &support::DebugSyntaxResult(self.dotdotdot_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsRestTupleTypeElement> for SyntaxNode {
    fn from(n: TsRestTupleTypeElement) -> SyntaxNode { n.syntax }
}
impl From<TsRestTupleTypeElement> for SyntaxElement {
    fn from(n: TsRestTupleTypeElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsReturnTypeAnnotation {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_RETURN_TYPE_ANNOTATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsReturnTypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsReturnTypeAnnotation")
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsReturnTypeAnnotation> for SyntaxNode {
    fn from(n: TsReturnTypeAnnotation) -> SyntaxNode { n.syntax }
}
impl From<TsReturnTypeAnnotation> for SyntaxElement {
    fn from(n: TsReturnTypeAnnotation) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsSetterSignatureTypeMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_SETTER_SIGNATURE_TYPE_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsSetterSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsSetterSignatureTypeMember")
            .field("set_token", &support::DebugSyntaxResult(self.set_token()))
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "l_paren_token",
                &support::DebugSyntaxResult(self.l_paren_token()),
            )
            .field("parameter", &support::DebugSyntaxResult(self.parameter()))
            .field(
                "r_paren_token",
                &support::DebugSyntaxResult(self.r_paren_token()),
            )
            .field(
                "separator_token",
                &support::DebugOptionalElement(self.separator_token()),
            )
            .finish()
    }
}
impl From<TsSetterSignatureTypeMember> for SyntaxNode {
    fn from(n: TsSetterSignatureTypeMember) -> SyntaxNode { n.syntax }
}
impl From<TsSetterSignatureTypeMember> for SyntaxElement {
    fn from(n: TsSetterSignatureTypeMember) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsStringLiteralType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_STRING_LITERAL_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsStringLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsStringLiteralType")
            .field(
                "literal_token",
                &support::DebugSyntaxResult(self.literal_token()),
            )
            .finish()
    }
}
impl From<TsStringLiteralType> for SyntaxNode {
    fn from(n: TsStringLiteralType) -> SyntaxNode { n.syntax }
}
impl From<TsStringLiteralType> for SyntaxElement {
    fn from(n: TsStringLiteralType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsStringType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_STRING_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsStringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsStringType")
            .field(
                "string_token",
                &support::DebugSyntaxResult(self.string_token()),
            )
            .finish()
    }
}
impl From<TsStringType> for SyntaxNode {
    fn from(n: TsStringType) -> SyntaxNode { n.syntax }
}
impl From<TsStringType> for SyntaxElement {
    fn from(n: TsStringType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsSymbolType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_SYMBOL_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsSymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsSymbolType")
            .field(
                "symbol_token",
                &support::DebugSyntaxResult(self.symbol_token()),
            )
            .finish()
    }
}
impl From<TsSymbolType> for SyntaxNode {
    fn from(n: TsSymbolType) -> SyntaxNode { n.syntax }
}
impl From<TsSymbolType> for SyntaxElement {
    fn from(n: TsSymbolType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTemplateChunkElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TEMPLATE_CHUNK_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTemplateChunkElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTemplateChunkElement")
            .field(
                "template_chunk_token",
                &support::DebugSyntaxResult(self.template_chunk_token()),
            )
            .finish()
    }
}
impl From<TsTemplateChunkElement> for SyntaxNode {
    fn from(n: TsTemplateChunkElement) -> SyntaxNode { n.syntax }
}
impl From<TsTemplateChunkElement> for SyntaxElement {
    fn from(n: TsTemplateChunkElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTemplateElement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TEMPLATE_ELEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTemplateElement")
            .field(
                "dollar_curly_token",
                &support::DebugSyntaxResult(self.dollar_curly_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .field(
                "r_curly_token",
                &support::DebugSyntaxResult(self.r_curly_token()),
            )
            .finish()
    }
}
impl From<TsTemplateElement> for SyntaxNode {
    fn from(n: TsTemplateElement) -> SyntaxNode { n.syntax }
}
impl From<TsTemplateElement> for SyntaxElement {
    fn from(n: TsTemplateElement) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTemplateLiteralType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TEMPLATE_LITERAL_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTemplateLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTemplateLiteralType")
            .field(
                "l_tick_token",
                &support::DebugSyntaxResult(self.l_tick_token()),
            )
            .field("elements", &self.elements())
            .field(
                "r_tick_token",
                &support::DebugSyntaxResult(self.r_tick_token()),
            )
            .finish()
    }
}
impl From<TsTemplateLiteralType> for SyntaxNode {
    fn from(n: TsTemplateLiteralType) -> SyntaxNode { n.syntax }
}
impl From<TsTemplateLiteralType> for SyntaxElement {
    fn from(n: TsTemplateLiteralType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsThisParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_THIS_PARAMETER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsThisParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsThisParameter")
            .field("this_token", &support::DebugSyntaxResult(self.this_token()))
            .field(
                "type_annotation",
                &support::DebugOptionalElement(self.type_annotation()),
            )
            .finish()
    }
}
impl From<TsThisParameter> for SyntaxNode {
    fn from(n: TsThisParameter) -> SyntaxNode { n.syntax }
}
impl From<TsThisParameter> for SyntaxElement {
    fn from(n: TsThisParameter) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsThisType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_THIS_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsThisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsThisType")
            .field("this_token", &support::DebugSyntaxResult(self.this_token()))
            .finish()
    }
}
impl From<TsThisType> for SyntaxNode {
    fn from(n: TsThisType) -> SyntaxNode { n.syntax }
}
impl From<TsThisType> for SyntaxElement {
    fn from(n: TsThisType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTupleType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TUPLE_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTupleType")
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
impl From<TsTupleType> for SyntaxNode {
    fn from(n: TsTupleType) -> SyntaxNode { n.syntax }
}
impl From<TsTupleType> for SyntaxElement {
    fn from(n: TsTupleType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeAliasDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_ALIAS_DECLARATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeAliasDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeAliasDeclaration")
            .field("type_token", &support::DebugSyntaxResult(self.type_token()))
            .field(
                "binding_identifier",
                &support::DebugSyntaxResult(self.binding_identifier()),
            )
            .field(
                "type_parameters",
                &support::DebugOptionalElement(self.type_parameters()),
            )
            .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .field(
                "semicolon_token",
                &support::DebugOptionalElement(self.semicolon_token()),
            )
            .finish()
    }
}
impl From<TsTypeAliasDeclaration> for SyntaxNode {
    fn from(n: TsTypeAliasDeclaration) -> SyntaxNode { n.syntax }
}
impl From<TsTypeAliasDeclaration> for SyntaxElement {
    fn from(n: TsTypeAliasDeclaration) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeAnnotation {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_ANNOTATION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeAnnotation")
            .field(
                "colon_token",
                &support::DebugSyntaxResult(self.colon_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsTypeAnnotation> for SyntaxNode {
    fn from(n: TsTypeAnnotation) -> SyntaxNode { n.syntax }
}
impl From<TsTypeAnnotation> for SyntaxElement {
    fn from(n: TsTypeAnnotation) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeArguments {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_ARGUMENTS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeArguments")
            .field(
                "l_angle_token",
                &support::DebugSyntaxResult(self.l_angle_token()),
            )
            .field("ts_type_argument_list", &self.ts_type_argument_list())
            .field(
                "r_angle_token",
                &support::DebugSyntaxResult(self.r_angle_token()),
            )
            .finish()
    }
}
impl From<TsTypeArguments> for SyntaxNode {
    fn from(n: TsTypeArguments) -> SyntaxNode { n.syntax }
}
impl From<TsTypeArguments> for SyntaxElement {
    fn from(n: TsTypeArguments) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeAssertionExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_ASSERTION_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeAssertionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeAssertionExpression")
            .field(
                "l_angle_token",
                &support::DebugSyntaxResult(self.l_angle_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .field(
                "r_angle_token",
                &support::DebugSyntaxResult(self.r_angle_token()),
            )
            .field("expression", &support::DebugSyntaxResult(self.expression()))
            .finish()
    }
}
impl From<TsTypeAssertionExpression> for SyntaxNode {
    fn from(n: TsTypeAssertionExpression) -> SyntaxNode { n.syntax }
}
impl From<TsTypeAssertionExpression> for SyntaxElement {
    fn from(n: TsTypeAssertionExpression) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeConstraintClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_CONSTRAINT_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeConstraintClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeConstraintClause")
            .field(
                "extends_token",
                &support::DebugSyntaxResult(self.extends_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsTypeConstraintClause> for SyntaxNode {
    fn from(n: TsTypeConstraintClause) -> SyntaxNode { n.syntax }
}
impl From<TsTypeConstraintClause> for SyntaxElement {
    fn from(n: TsTypeConstraintClause) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeOperatorType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_OPERATOR_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeOperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeOperatorType")
            .field(
                "operator_token",
                &support::DebugSyntaxResult(self.operator_token()),
            )
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsTypeOperatorType> for SyntaxNode {
    fn from(n: TsTypeOperatorType) -> SyntaxNode { n.syntax }
}
impl From<TsTypeOperatorType> for SyntaxElement {
    fn from(n: TsTypeOperatorType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_PARAMETER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeParameter")
            .field("name", &support::DebugSyntaxResult(self.name()))
            .field(
                "constraint",
                &support::DebugOptionalElement(self.constraint()),
            )
            .field("default", &support::DebugOptionalElement(self.default()))
            .finish()
    }
}
impl From<TsTypeParameter> for SyntaxNode {
    fn from(n: TsTypeParameter) -> SyntaxNode { n.syntax }
}
impl From<TsTypeParameter> for SyntaxElement {
    fn from(n: TsTypeParameter) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeParameterName {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_PARAMETER_NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeParameterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeParameterName")
            .field(
                "ident_token",
                &support::DebugSyntaxResult(self.ident_token()),
            )
            .finish()
    }
}
impl From<TsTypeParameterName> for SyntaxNode {
    fn from(n: TsTypeParameterName) -> SyntaxNode { n.syntax }
}
impl From<TsTypeParameterName> for SyntaxElement {
    fn from(n: TsTypeParameterName) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeParameters {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_PARAMETERS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeParameters")
            .field(
                "l_angle_token",
                &support::DebugSyntaxResult(self.l_angle_token()),
            )
            .field("items", &self.items())
            .field(
                "r_angle_token",
                &support::DebugSyntaxResult(self.r_angle_token()),
            )
            .finish()
    }
}
impl From<TsTypeParameters> for SyntaxNode {
    fn from(n: TsTypeParameters) -> SyntaxNode { n.syntax }
}
impl From<TsTypeParameters> for SyntaxElement {
    fn from(n: TsTypeParameters) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypePredicate {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_PREDICATE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypePredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypePredicate")
            .field(
                "asserts_token",
                &support::DebugOptionalElement(self.asserts_token()),
            )
            .field(
                "parameter_name",
                &support::DebugSyntaxResult(self.parameter_name()),
            )
            .field("is_token", &support::DebugSyntaxResult(self.is_token()))
            .field("ty", &support::DebugSyntaxResult(self.ty()))
            .finish()
    }
}
impl From<TsTypePredicate> for SyntaxNode {
    fn from(n: TsTypePredicate) -> SyntaxNode { n.syntax }
}
impl From<TsTypePredicate> for SyntaxElement {
    fn from(n: TsTypePredicate) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsTypeofType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPEOF_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeofType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsTypeofType")
            .field(
                "typeof_token",
                &support::DebugSyntaxResult(self.typeof_token()),
            )
            .field(
                "expression_name",
                &support::DebugSyntaxResult(self.expression_name()),
            )
            .finish()
    }
}
impl From<TsTypeofType> for SyntaxNode {
    fn from(n: TsTypeofType) -> SyntaxNode { n.syntax }
}
impl From<TsTypeofType> for SyntaxElement {
    fn from(n: TsTypeofType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsUndefinedType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_UNDEFINED_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUndefinedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsUndefinedType")
            .field(
                "undefined_token",
                &support::DebugSyntaxResult(self.undefined_token()),
            )
            .finish()
    }
}
impl From<TsUndefinedType> for SyntaxNode {
    fn from(n: TsUndefinedType) -> SyntaxNode { n.syntax }
}
impl From<TsUndefinedType> for SyntaxElement {
    fn from(n: TsUndefinedType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsUnionType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_UNION_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUnionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsUnionType")
            .field(
                "leading_separator_token_token",
                &support::DebugOptionalElement(self.leading_separator_token_token()),
            )
            .field("types", &self.types())
            .finish()
    }
}
impl From<TsUnionType> for SyntaxNode {
    fn from(n: TsUnionType) -> SyntaxNode { n.syntax }
}
impl From<TsUnionType> for SyntaxElement {
    fn from(n: TsUnionType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsUnknownType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_UNKNOWN_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUnknownType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsUnknownType")
            .field(
                "unknown_token",
                &support::DebugSyntaxResult(self.unknown_token()),
            )
            .finish()
    }
}
impl From<TsUnknownType> for SyntaxNode {
    fn from(n: TsUnknownType) -> SyntaxNode { n.syntax }
}
impl From<TsUnknownType> for SyntaxElement {
    fn from(n: TsUnknownType) -> SyntaxElement { n.syntax.into() }
}
impl AstNode for TsVoidType {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_VOID_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsVoidType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TsVoidType")
            .field("void_token", &support::DebugSyntaxResult(self.void_token()))
            .finish()
    }
}
impl From<TsVoidType> for SyntaxNode {
    fn from(n: TsVoidType) -> SyntaxNode { n.syntax }
}
impl From<TsVoidType> for SyntaxElement {
    fn from(n: TsVoidType) -> SyntaxElement { n.syntax.into() }
}
impl From<JsArrayAssignmentPatternRestElement> for JsAnyArrayAssignmentPatternElement {
    fn from(node: JsArrayAssignmentPatternRestElement) -> JsAnyArrayAssignmentPatternElement {
        JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(node)
    }
}
impl From<JsArrayHole> for JsAnyArrayAssignmentPatternElement {
    fn from(node: JsArrayHole) -> JsAnyArrayAssignmentPatternElement {
        JsAnyArrayAssignmentPatternElement::JsArrayHole(node)
    }
}
impl From<JsAssignmentWithDefault> for JsAnyArrayAssignmentPatternElement {
    fn from(node: JsAssignmentWithDefault) -> JsAnyArrayAssignmentPatternElement {
        JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(node)
    }
}
impl From<JsUnknownAssignment> for JsAnyArrayAssignmentPatternElement {
    fn from(node: JsUnknownAssignment) -> JsAnyArrayAssignmentPatternElement {
        JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(node)
    }
}
impl AstNode for JsAnyArrayAssignmentPatternElement {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT
            | JS_ARRAY_HOLE
            | JS_ASSIGNMENT_WITH_DEFAULT
            | JS_UNKNOWN_ASSIGNMENT => true,
            k if JsAnyAssignmentPattern::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
                JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(
                    JsArrayAssignmentPatternRestElement { syntax },
                )
            }
            JS_ARRAY_HOLE => {
                JsAnyArrayAssignmentPatternElement::JsArrayHole(JsArrayHole { syntax })
            }
            JS_ASSIGNMENT_WITH_DEFAULT => {
                JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(
                    JsAssignmentWithDefault { syntax },
                )
            }
            JS_UNKNOWN_ASSIGNMENT => {
                JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(JsUnknownAssignment {
                    syntax,
                })
            }
            _ => {
                if let Some(js_any_assignment_pattern) = JsAnyAssignmentPattern::cast(syntax) {
                    return Some(JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(
                        js_any_assignment_pattern,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(it) => {
                &it.syntax
            }
            JsAnyArrayAssignmentPatternElement::JsArrayHole(it) => &it.syntax,
            JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(it) => &it.syntax,
            JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(it) => &it.syntax,
            JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyArrayAssignmentPatternElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyArrayAssignmentPatternElement::JsArrayHole(it) => std::fmt::Debug::fmt(it, f),
            JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<JsAnyArrayAssignmentPatternElement> for SyntaxNode {
    fn from(n: JsAnyArrayAssignmentPatternElement) -> SyntaxNode {
        match n {
            JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(it) => it.into(),
            JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(it) => {
                it.into()
            }
            JsAnyArrayAssignmentPatternElement::JsArrayHole(it) => it.into(),
            JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(it) => it.into(),
            JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(it) => it.into(),
        }
    }
}
impl From<JsAnyArrayAssignmentPatternElement> for SyntaxElement {
    fn from(n: JsAnyArrayAssignmentPatternElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsArrayBindingPatternRestElement> for JsAnyArrayBindingPatternElement {
    fn from(node: JsArrayBindingPatternRestElement) -> JsAnyArrayBindingPatternElement {
        JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(node)
    }
}
impl From<JsArrayHole> for JsAnyArrayBindingPatternElement {
    fn from(node: JsArrayHole) -> JsAnyArrayBindingPatternElement {
        JsAnyArrayBindingPatternElement::JsArrayHole(node)
    }
}
impl From<JsBindingPatternWithDefault> for JsAnyArrayBindingPatternElement {
    fn from(node: JsBindingPatternWithDefault) -> JsAnyArrayBindingPatternElement {
        JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(node)
    }
}
impl AstNode for JsAnyArrayBindingPatternElement {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
            | JS_ARRAY_HOLE
            | JS_BINDING_PATTERN_WITH_DEFAULT => true,
            k if JsAnyBindingPattern::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
                JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                    JsArrayBindingPatternRestElement { syntax },
                )
            }
            JS_ARRAY_HOLE => JsAnyArrayBindingPatternElement::JsArrayHole(JsArrayHole { syntax }),
            JS_BINDING_PATTERN_WITH_DEFAULT => {
                JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(
                    JsBindingPatternWithDefault { syntax },
                )
            }
            _ => {
                if let Some(js_any_binding_pattern) = JsAnyBindingPattern::cast(syntax) {
                    return Some(JsAnyArrayBindingPatternElement::JsAnyBindingPattern(
                        js_any_binding_pattern,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(it) => &it.syntax,
            JsAnyArrayBindingPatternElement::JsArrayHole(it) => &it.syntax,
            JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(it) => &it.syntax,
            JsAnyArrayBindingPatternElement::JsAnyBindingPattern(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyArrayBindingPatternElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyArrayBindingPatternElement::JsAnyBindingPattern(it) => std::fmt::Debug::fmt(it, f),
            JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyArrayBindingPatternElement::JsArrayHole(it) => std::fmt::Debug::fmt(it, f),
            JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<JsAnyArrayBindingPatternElement> for SyntaxNode {
    fn from(n: JsAnyArrayBindingPatternElement) -> SyntaxNode {
        match n {
            JsAnyArrayBindingPatternElement::JsAnyBindingPattern(it) => it.into(),
            JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(it) => it.into(),
            JsAnyArrayBindingPatternElement::JsArrayHole(it) => it.into(),
            JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(it) => it.into(),
        }
    }
}
impl From<JsAnyArrayBindingPatternElement> for SyntaxElement {
    fn from(n: JsAnyArrayBindingPatternElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsArrayHole> for JsAnyArrayElement {
    fn from(node: JsArrayHole) -> JsAnyArrayElement { JsAnyArrayElement::JsArrayHole(node) }
}
impl From<JsSpread> for JsAnyArrayElement {
    fn from(node: JsSpread) -> JsAnyArrayElement { JsAnyArrayElement::JsSpread(node) }
}
impl AstNode for JsAnyArrayElement {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_ARRAY_HOLE | JS_SPREAD => true,
            k if JsAnyExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_ARRAY_HOLE => JsAnyArrayElement::JsArrayHole(JsArrayHole { syntax }),
            JS_SPREAD => JsAnyArrayElement::JsSpread(JsSpread { syntax }),
            _ => {
                if let Some(js_any_expression) = JsAnyExpression::cast(syntax) {
                    return Some(JsAnyArrayElement::JsAnyExpression(js_any_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyArrayElement::JsArrayHole(it) => &it.syntax,
            JsAnyArrayElement::JsSpread(it) => &it.syntax,
            JsAnyArrayElement::JsAnyExpression(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyArrayElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyArrayElement::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyArrayElement::JsArrayHole(it) => std::fmt::Debug::fmt(it, f),
            JsAnyArrayElement::JsSpread(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyArrayElement> for SyntaxNode {
    fn from(n: JsAnyArrayElement) -> SyntaxNode {
        match n {
            JsAnyArrayElement::JsAnyExpression(it) => it.into(),
            JsAnyArrayElement::JsArrayHole(it) => it.into(),
            JsAnyArrayElement::JsSpread(it) => it.into(),
        }
    }
}
impl From<JsAnyArrayElement> for SyntaxElement {
    fn from(n: JsAnyArrayElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsParameters> for JsAnyArrowFunctionParameters {
    fn from(node: JsParameters) -> JsAnyArrowFunctionParameters {
        JsAnyArrowFunctionParameters::JsParameters(node)
    }
}
impl AstNode for JsAnyArrowFunctionParameters {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_PARAMETERS => true,
            k if JsAnyBinding::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_PARAMETERS => JsAnyArrowFunctionParameters::JsParameters(JsParameters { syntax }),
            _ => {
                if let Some(js_any_binding) = JsAnyBinding::cast(syntax) {
                    return Some(JsAnyArrowFunctionParameters::JsAnyBinding(js_any_binding));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyArrowFunctionParameters::JsParameters(it) => &it.syntax,
            JsAnyArrowFunctionParameters::JsAnyBinding(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyArrowFunctionParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyArrowFunctionParameters::JsAnyBinding(it) => std::fmt::Debug::fmt(it, f),
            JsAnyArrowFunctionParameters::JsParameters(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyArrowFunctionParameters> for SyntaxNode {
    fn from(n: JsAnyArrowFunctionParameters) -> SyntaxNode {
        match n {
            JsAnyArrowFunctionParameters::JsAnyBinding(it) => it.into(),
            JsAnyArrowFunctionParameters::JsParameters(it) => it.into(),
        }
    }
}
impl From<JsAnyArrowFunctionParameters> for SyntaxElement {
    fn from(n: JsAnyArrowFunctionParameters) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsComputedMemberAssignment> for JsAnyAssignment {
    fn from(node: JsComputedMemberAssignment) -> JsAnyAssignment {
        JsAnyAssignment::JsComputedMemberAssignment(node)
    }
}
impl From<JsIdentifierAssignment> for JsAnyAssignment {
    fn from(node: JsIdentifierAssignment) -> JsAnyAssignment {
        JsAnyAssignment::JsIdentifierAssignment(node)
    }
}
impl From<JsParenthesizedAssignment> for JsAnyAssignment {
    fn from(node: JsParenthesizedAssignment) -> JsAnyAssignment {
        JsAnyAssignment::JsParenthesizedAssignment(node)
    }
}
impl From<JsStaticMemberAssignment> for JsAnyAssignment {
    fn from(node: JsStaticMemberAssignment) -> JsAnyAssignment {
        JsAnyAssignment::JsStaticMemberAssignment(node)
    }
}
impl From<JsUnknownAssignment> for JsAnyAssignment {
    fn from(node: JsUnknownAssignment) -> JsAnyAssignment {
        JsAnyAssignment::JsUnknownAssignment(node)
    }
}
impl AstNode for JsAnyAssignment {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_COMPUTED_MEMBER_ASSIGNMENT
                | JS_IDENTIFIER_ASSIGNMENT
                | JS_PARENTHESIZED_ASSIGNMENT
                | JS_STATIC_MEMBER_ASSIGNMENT
                | JS_UNKNOWN_ASSIGNMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_COMPUTED_MEMBER_ASSIGNMENT => {
                JsAnyAssignment::JsComputedMemberAssignment(JsComputedMemberAssignment { syntax })
            }
            JS_IDENTIFIER_ASSIGNMENT => {
                JsAnyAssignment::JsIdentifierAssignment(JsIdentifierAssignment { syntax })
            }
            JS_PARENTHESIZED_ASSIGNMENT => {
                JsAnyAssignment::JsParenthesizedAssignment(JsParenthesizedAssignment { syntax })
            }
            JS_STATIC_MEMBER_ASSIGNMENT => {
                JsAnyAssignment::JsStaticMemberAssignment(JsStaticMemberAssignment { syntax })
            }
            JS_UNKNOWN_ASSIGNMENT => {
                JsAnyAssignment::JsUnknownAssignment(JsUnknownAssignment { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyAssignment::JsComputedMemberAssignment(it) => &it.syntax,
            JsAnyAssignment::JsIdentifierAssignment(it) => &it.syntax,
            JsAnyAssignment::JsParenthesizedAssignment(it) => &it.syntax,
            JsAnyAssignment::JsStaticMemberAssignment(it) => &it.syntax,
            JsAnyAssignment::JsUnknownAssignment(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyAssignment::JsComputedMemberAssignment(it) => std::fmt::Debug::fmt(it, f),
            JsAnyAssignment::JsIdentifierAssignment(it) => std::fmt::Debug::fmt(it, f),
            JsAnyAssignment::JsParenthesizedAssignment(it) => std::fmt::Debug::fmt(it, f),
            JsAnyAssignment::JsStaticMemberAssignment(it) => std::fmt::Debug::fmt(it, f),
            JsAnyAssignment::JsUnknownAssignment(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyAssignment> for SyntaxNode {
    fn from(n: JsAnyAssignment) -> SyntaxNode {
        match n {
            JsAnyAssignment::JsComputedMemberAssignment(it) => it.into(),
            JsAnyAssignment::JsIdentifierAssignment(it) => it.into(),
            JsAnyAssignment::JsParenthesizedAssignment(it) => it.into(),
            JsAnyAssignment::JsStaticMemberAssignment(it) => it.into(),
            JsAnyAssignment::JsUnknownAssignment(it) => it.into(),
        }
    }
}
impl From<JsAnyAssignment> for SyntaxElement {
    fn from(n: JsAnyAssignment) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsArrayAssignmentPattern> for JsAnyAssignmentPattern {
    fn from(node: JsArrayAssignmentPattern) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsArrayAssignmentPattern(node)
    }
}
impl From<JsObjectAssignmentPattern> for JsAnyAssignmentPattern {
    fn from(node: JsObjectAssignmentPattern) -> JsAnyAssignmentPattern {
        JsAnyAssignmentPattern::JsObjectAssignmentPattern(node)
    }
}
impl AstNode for JsAnyAssignmentPattern {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_ARRAY_ASSIGNMENT_PATTERN | JS_OBJECT_ASSIGNMENT_PATTERN => true,
            k if JsAnyAssignment::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_ARRAY_ASSIGNMENT_PATTERN => {
                JsAnyAssignmentPattern::JsArrayAssignmentPattern(JsArrayAssignmentPattern {
                    syntax,
                })
            }
            JS_OBJECT_ASSIGNMENT_PATTERN => {
                JsAnyAssignmentPattern::JsObjectAssignmentPattern(JsObjectAssignmentPattern {
                    syntax,
                })
            }
            _ => {
                if let Some(js_any_assignment) = JsAnyAssignment::cast(syntax) {
                    return Some(JsAnyAssignmentPattern::JsAnyAssignment(js_any_assignment));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyAssignmentPattern::JsArrayAssignmentPattern(it) => &it.syntax,
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(it) => &it.syntax,
            JsAnyAssignmentPattern::JsAnyAssignment(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyAssignmentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyAssignmentPattern::JsAnyAssignment(it) => std::fmt::Debug::fmt(it, f),
            JsAnyAssignmentPattern::JsArrayAssignmentPattern(it) => std::fmt::Debug::fmt(it, f),
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyAssignmentPattern> for SyntaxNode {
    fn from(n: JsAnyAssignmentPattern) -> SyntaxNode {
        match n {
            JsAnyAssignmentPattern::JsAnyAssignment(it) => it.into(),
            JsAnyAssignmentPattern::JsArrayAssignmentPattern(it) => it.into(),
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(it) => it.into(),
        }
    }
}
impl From<JsAnyAssignmentPattern> for SyntaxElement {
    fn from(n: JsAnyAssignmentPattern) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsIdentifierBinding> for JsAnyBinding {
    fn from(node: JsIdentifierBinding) -> JsAnyBinding { JsAnyBinding::JsIdentifierBinding(node) }
}
impl From<JsUnknownBinding> for JsAnyBinding {
    fn from(node: JsUnknownBinding) -> JsAnyBinding { JsAnyBinding::JsUnknownBinding(node) }
}
impl AstNode for JsAnyBinding {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, JS_IDENTIFIER_BINDING | JS_UNKNOWN_BINDING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_IDENTIFIER_BINDING => {
                JsAnyBinding::JsIdentifierBinding(JsIdentifierBinding { syntax })
            }
            JS_UNKNOWN_BINDING => JsAnyBinding::JsUnknownBinding(JsUnknownBinding { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyBinding::JsIdentifierBinding(it) => &it.syntax,
            JsAnyBinding::JsUnknownBinding(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyBinding::JsIdentifierBinding(it) => std::fmt::Debug::fmt(it, f),
            JsAnyBinding::JsUnknownBinding(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyBinding> for SyntaxNode {
    fn from(n: JsAnyBinding) -> SyntaxNode {
        match n {
            JsAnyBinding::JsIdentifierBinding(it) => it.into(),
            JsAnyBinding::JsUnknownBinding(it) => it.into(),
        }
    }
}
impl From<JsAnyBinding> for SyntaxElement {
    fn from(n: JsAnyBinding) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsArrayBindingPattern> for JsAnyBindingPattern {
    fn from(node: JsArrayBindingPattern) -> JsAnyBindingPattern {
        JsAnyBindingPattern::JsArrayBindingPattern(node)
    }
}
impl From<JsObjectBindingPattern> for JsAnyBindingPattern {
    fn from(node: JsObjectBindingPattern) -> JsAnyBindingPattern {
        JsAnyBindingPattern::JsObjectBindingPattern(node)
    }
}
impl AstNode for JsAnyBindingPattern {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_ARRAY_BINDING_PATTERN | JS_OBJECT_BINDING_PATTERN => true,
            k if JsAnyBinding::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_ARRAY_BINDING_PATTERN => {
                JsAnyBindingPattern::JsArrayBindingPattern(JsArrayBindingPattern { syntax })
            }
            JS_OBJECT_BINDING_PATTERN => {
                JsAnyBindingPattern::JsObjectBindingPattern(JsObjectBindingPattern { syntax })
            }
            _ => {
                if let Some(js_any_binding) = JsAnyBinding::cast(syntax) {
                    return Some(JsAnyBindingPattern::JsAnyBinding(js_any_binding));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyBindingPattern::JsArrayBindingPattern(it) => &it.syntax,
            JsAnyBindingPattern::JsObjectBindingPattern(it) => &it.syntax,
            JsAnyBindingPattern::JsAnyBinding(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyBindingPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyBindingPattern::JsAnyBinding(it) => std::fmt::Debug::fmt(it, f),
            JsAnyBindingPattern::JsArrayBindingPattern(it) => std::fmt::Debug::fmt(it, f),
            JsAnyBindingPattern::JsObjectBindingPattern(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyBindingPattern> for SyntaxNode {
    fn from(n: JsAnyBindingPattern) -> SyntaxNode {
        match n {
            JsAnyBindingPattern::JsAnyBinding(it) => it.into(),
            JsAnyBindingPattern::JsArrayBindingPattern(it) => it.into(),
            JsAnyBindingPattern::JsObjectBindingPattern(it) => it.into(),
        }
    }
}
impl From<JsAnyBindingPattern> for SyntaxElement {
    fn from(n: JsAnyBindingPattern) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsClassDeclaration> for JsAnyClass {
    fn from(node: JsClassDeclaration) -> JsAnyClass { JsAnyClass::JsClassDeclaration(node) }
}
impl From<JsClassExpression> for JsAnyClass {
    fn from(node: JsClassExpression) -> JsAnyClass { JsAnyClass::JsClassExpression(node) }
}
impl From<JsExportDefaultClassClause> for JsAnyClass {
    fn from(node: JsExportDefaultClassClause) -> JsAnyClass {
        JsAnyClass::JsExportDefaultClassClause(node)
    }
}
impl AstNode for JsAnyClass {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_CLASS_DECLARATION | JS_CLASS_EXPRESSION | JS_EXPORT_DEFAULT_CLASS_CLAUSE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_CLASS_DECLARATION => JsAnyClass::JsClassDeclaration(JsClassDeclaration { syntax }),
            JS_CLASS_EXPRESSION => JsAnyClass::JsClassExpression(JsClassExpression { syntax }),
            JS_EXPORT_DEFAULT_CLASS_CLAUSE => {
                JsAnyClass::JsExportDefaultClassClause(JsExportDefaultClassClause { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyClass::JsClassDeclaration(it) => &it.syntax,
            JsAnyClass::JsClassExpression(it) => &it.syntax,
            JsAnyClass::JsExportDefaultClassClause(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyClass::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClass::JsClassExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClass::JsExportDefaultClassClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyClass> for SyntaxNode {
    fn from(n: JsAnyClass) -> SyntaxNode {
        match n {
            JsAnyClass::JsClassDeclaration(it) => it.into(),
            JsAnyClass::JsClassExpression(it) => it.into(),
            JsAnyClass::JsExportDefaultClassClause(it) => it.into(),
        }
    }
}
impl From<JsAnyClass> for SyntaxElement {
    fn from(n: JsAnyClass) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsConstructorClassMember> for JsAnyClassMember {
    fn from(node: JsConstructorClassMember) -> JsAnyClassMember {
        JsAnyClassMember::JsConstructorClassMember(node)
    }
}
impl From<JsEmptyClassMember> for JsAnyClassMember {
    fn from(node: JsEmptyClassMember) -> JsAnyClassMember {
        JsAnyClassMember::JsEmptyClassMember(node)
    }
}
impl From<JsGetterClassMember> for JsAnyClassMember {
    fn from(node: JsGetterClassMember) -> JsAnyClassMember {
        JsAnyClassMember::JsGetterClassMember(node)
    }
}
impl From<JsMethodClassMember> for JsAnyClassMember {
    fn from(node: JsMethodClassMember) -> JsAnyClassMember {
        JsAnyClassMember::JsMethodClassMember(node)
    }
}
impl From<JsPropertyClassMember> for JsAnyClassMember {
    fn from(node: JsPropertyClassMember) -> JsAnyClassMember {
        JsAnyClassMember::JsPropertyClassMember(node)
    }
}
impl From<JsSetterClassMember> for JsAnyClassMember {
    fn from(node: JsSetterClassMember) -> JsAnyClassMember {
        JsAnyClassMember::JsSetterClassMember(node)
    }
}
impl From<JsStaticInitializationBlockClassMember> for JsAnyClassMember {
    fn from(node: JsStaticInitializationBlockClassMember) -> JsAnyClassMember {
        JsAnyClassMember::JsStaticInitializationBlockClassMember(node)
    }
}
impl From<JsUnknownMember> for JsAnyClassMember {
    fn from(node: JsUnknownMember) -> JsAnyClassMember { JsAnyClassMember::JsUnknownMember(node) }
}
impl AstNode for JsAnyClassMember {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_CONSTRUCTOR_CLASS_MEMBER
                | JS_EMPTY_CLASS_MEMBER
                | JS_GETTER_CLASS_MEMBER
                | JS_METHOD_CLASS_MEMBER
                | JS_PROPERTY_CLASS_MEMBER
                | JS_SETTER_CLASS_MEMBER
                | JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
                | JS_UNKNOWN_MEMBER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_CONSTRUCTOR_CLASS_MEMBER => {
                JsAnyClassMember::JsConstructorClassMember(JsConstructorClassMember { syntax })
            }
            JS_EMPTY_CLASS_MEMBER => {
                JsAnyClassMember::JsEmptyClassMember(JsEmptyClassMember { syntax })
            }
            JS_GETTER_CLASS_MEMBER => {
                JsAnyClassMember::JsGetterClassMember(JsGetterClassMember { syntax })
            }
            JS_METHOD_CLASS_MEMBER => {
                JsAnyClassMember::JsMethodClassMember(JsMethodClassMember { syntax })
            }
            JS_PROPERTY_CLASS_MEMBER => {
                JsAnyClassMember::JsPropertyClassMember(JsPropertyClassMember { syntax })
            }
            JS_SETTER_CLASS_MEMBER => {
                JsAnyClassMember::JsSetterClassMember(JsSetterClassMember { syntax })
            }
            JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                JsAnyClassMember::JsStaticInitializationBlockClassMember(
                    JsStaticInitializationBlockClassMember { syntax },
                )
            }
            JS_UNKNOWN_MEMBER => JsAnyClassMember::JsUnknownMember(JsUnknownMember { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyClassMember::JsConstructorClassMember(it) => &it.syntax,
            JsAnyClassMember::JsEmptyClassMember(it) => &it.syntax,
            JsAnyClassMember::JsGetterClassMember(it) => &it.syntax,
            JsAnyClassMember::JsMethodClassMember(it) => &it.syntax,
            JsAnyClassMember::JsPropertyClassMember(it) => &it.syntax,
            JsAnyClassMember::JsSetterClassMember(it) => &it.syntax,
            JsAnyClassMember::JsStaticInitializationBlockClassMember(it) => &it.syntax,
            JsAnyClassMember::JsUnknownMember(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyClassMember::JsConstructorClassMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClassMember::JsEmptyClassMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClassMember::JsGetterClassMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClassMember::JsMethodClassMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClassMember::JsPropertyClassMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClassMember::JsSetterClassMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClassMember::JsStaticInitializationBlockClassMember(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyClassMember::JsUnknownMember(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyClassMember> for SyntaxNode {
    fn from(n: JsAnyClassMember) -> SyntaxNode {
        match n {
            JsAnyClassMember::JsConstructorClassMember(it) => it.into(),
            JsAnyClassMember::JsEmptyClassMember(it) => it.into(),
            JsAnyClassMember::JsGetterClassMember(it) => it.into(),
            JsAnyClassMember::JsMethodClassMember(it) => it.into(),
            JsAnyClassMember::JsPropertyClassMember(it) => it.into(),
            JsAnyClassMember::JsSetterClassMember(it) => it.into(),
            JsAnyClassMember::JsStaticInitializationBlockClassMember(it) => it.into(),
            JsAnyClassMember::JsUnknownMember(it) => it.into(),
        }
    }
}
impl From<JsAnyClassMember> for SyntaxElement {
    fn from(n: JsAnyClassMember) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsComputedMemberName> for JsAnyClassMemberName {
    fn from(node: JsComputedMemberName) -> JsAnyClassMemberName {
        JsAnyClassMemberName::JsComputedMemberName(node)
    }
}
impl From<JsLiteralMemberName> for JsAnyClassMemberName {
    fn from(node: JsLiteralMemberName) -> JsAnyClassMemberName {
        JsAnyClassMemberName::JsLiteralMemberName(node)
    }
}
impl From<JsPrivateClassMemberName> for JsAnyClassMemberName {
    fn from(node: JsPrivateClassMemberName) -> JsAnyClassMemberName {
        JsAnyClassMemberName::JsPrivateClassMemberName(node)
    }
}
impl AstNode for JsAnyClassMemberName {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_COMPUTED_MEMBER_NAME | JS_LITERAL_MEMBER_NAME | JS_PRIVATE_CLASS_MEMBER_NAME
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_COMPUTED_MEMBER_NAME => {
                JsAnyClassMemberName::JsComputedMemberName(JsComputedMemberName { syntax })
            }
            JS_LITERAL_MEMBER_NAME => {
                JsAnyClassMemberName::JsLiteralMemberName(JsLiteralMemberName { syntax })
            }
            JS_PRIVATE_CLASS_MEMBER_NAME => {
                JsAnyClassMemberName::JsPrivateClassMemberName(JsPrivateClassMemberName { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyClassMemberName::JsComputedMemberName(it) => &it.syntax,
            JsAnyClassMemberName::JsLiteralMemberName(it) => &it.syntax,
            JsAnyClassMemberName::JsPrivateClassMemberName(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyClassMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyClassMemberName::JsComputedMemberName(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClassMemberName::JsLiteralMemberName(it) => std::fmt::Debug::fmt(it, f),
            JsAnyClassMemberName::JsPrivateClassMemberName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyClassMemberName> for SyntaxNode {
    fn from(n: JsAnyClassMemberName) -> SyntaxNode {
        match n {
            JsAnyClassMemberName::JsComputedMemberName(it) => it.into(),
            JsAnyClassMemberName::JsLiteralMemberName(it) => it.into(),
            JsAnyClassMemberName::JsPrivateClassMemberName(it) => it.into(),
        }
    }
}
impl From<JsAnyClassMemberName> for SyntaxElement {
    fn from(n: JsAnyClassMemberName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsRestParameter> for JsAnyConstructorParameter {
    fn from(node: JsRestParameter) -> JsAnyConstructorParameter {
        JsAnyConstructorParameter::JsRestParameter(node)
    }
}
impl From<TsPropertyParameter> for JsAnyConstructorParameter {
    fn from(node: TsPropertyParameter) -> JsAnyConstructorParameter {
        JsAnyConstructorParameter::TsPropertyParameter(node)
    }
}
impl From<TsReadonlyPropertyParameter> for JsAnyConstructorParameter {
    fn from(node: TsReadonlyPropertyParameter) -> JsAnyConstructorParameter {
        JsAnyConstructorParameter::TsReadonlyPropertyParameter(node)
    }
}
impl AstNode for JsAnyConstructorParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_REST_PARAMETER | TS_PROPERTY_PARAMETER | TS_READONLY_PROPERTY_PARAMETER => true,
            k if JsAnyFormalParameter::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_REST_PARAMETER => {
                JsAnyConstructorParameter::JsRestParameter(JsRestParameter { syntax })
            }
            TS_PROPERTY_PARAMETER => {
                JsAnyConstructorParameter::TsPropertyParameter(TsPropertyParameter { syntax })
            }
            TS_READONLY_PROPERTY_PARAMETER => {
                JsAnyConstructorParameter::TsReadonlyPropertyParameter(
                    TsReadonlyPropertyParameter { syntax },
                )
            }
            _ => {
                if let Some(js_any_formal_parameter) = JsAnyFormalParameter::cast(syntax) {
                    return Some(JsAnyConstructorParameter::JsAnyFormalParameter(
                        js_any_formal_parameter,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyConstructorParameter::JsRestParameter(it) => &it.syntax,
            JsAnyConstructorParameter::TsPropertyParameter(it) => &it.syntax,
            JsAnyConstructorParameter::TsReadonlyPropertyParameter(it) => &it.syntax,
            JsAnyConstructorParameter::JsAnyFormalParameter(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyConstructorParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyConstructorParameter::JsAnyFormalParameter(it) => std::fmt::Debug::fmt(it, f),
            JsAnyConstructorParameter::JsRestParameter(it) => std::fmt::Debug::fmt(it, f),
            JsAnyConstructorParameter::TsPropertyParameter(it) => std::fmt::Debug::fmt(it, f),
            JsAnyConstructorParameter::TsReadonlyPropertyParameter(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<JsAnyConstructorParameter> for SyntaxNode {
    fn from(n: JsAnyConstructorParameter) -> SyntaxNode {
        match n {
            JsAnyConstructorParameter::JsAnyFormalParameter(it) => it.into(),
            JsAnyConstructorParameter::JsRestParameter(it) => it.into(),
            JsAnyConstructorParameter::TsPropertyParameter(it) => it.into(),
            JsAnyConstructorParameter::TsReadonlyPropertyParameter(it) => it.into(),
        }
    }
}
impl From<JsAnyConstructorParameter> for SyntaxElement {
    fn from(n: JsAnyConstructorParameter) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsClassDeclaration> for JsAnyDeclaration {
    fn from(node: JsClassDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::JsClassDeclaration(node)
    }
}
impl From<JsFunctionDeclaration> for JsAnyDeclaration {
    fn from(node: JsFunctionDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::JsFunctionDeclaration(node)
    }
}
impl From<JsVariableDeclaration> for JsAnyDeclaration {
    fn from(node: JsVariableDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::JsVariableDeclaration(node)
    }
}
impl From<TsDeclareFunctionDeclaration> for JsAnyDeclaration {
    fn from(node: TsDeclareFunctionDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::TsDeclareFunctionDeclaration(node)
    }
}
impl From<TsEnumDeclaration> for JsAnyDeclaration {
    fn from(node: TsEnumDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::TsEnumDeclaration(node)
    }
}
impl From<TsExternalModuleDeclaration> for JsAnyDeclaration {
    fn from(node: TsExternalModuleDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::TsExternalModuleDeclaration(node)
    }
}
impl From<TsGlobalDeclaration> for JsAnyDeclaration {
    fn from(node: TsGlobalDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::TsGlobalDeclaration(node)
    }
}
impl From<TsInterfaceDeclaration> for JsAnyDeclaration {
    fn from(node: TsInterfaceDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::TsInterfaceDeclaration(node)
    }
}
impl From<TsModuleDeclaration> for JsAnyDeclaration {
    fn from(node: TsModuleDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::TsModuleDeclaration(node)
    }
}
impl From<TsTypeAliasDeclaration> for JsAnyDeclaration {
    fn from(node: TsTypeAliasDeclaration) -> JsAnyDeclaration {
        JsAnyDeclaration::TsTypeAliasDeclaration(node)
    }
}
impl AstNode for JsAnyDeclaration {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_CLASS_DECLARATION
                | JS_FUNCTION_DECLARATION
                | JS_VARIABLE_DECLARATION
                | TS_DECLARE_FUNCTION_DECLARATION
                | TS_ENUM_DECLARATION
                | TS_EXTERNAL_MODULE_DECLARATION
                | TS_GLOBAL_DECLARATION
                | TS_INTERFACE_DECLARATION
                | TS_MODULE_DECLARATION
                | TS_TYPE_ALIAS_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_CLASS_DECLARATION => {
                JsAnyDeclaration::JsClassDeclaration(JsClassDeclaration { syntax })
            }
            JS_FUNCTION_DECLARATION => {
                JsAnyDeclaration::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
            }
            JS_VARIABLE_DECLARATION => {
                JsAnyDeclaration::JsVariableDeclaration(JsVariableDeclaration { syntax })
            }
            TS_DECLARE_FUNCTION_DECLARATION => {
                JsAnyDeclaration::TsDeclareFunctionDeclaration(TsDeclareFunctionDeclaration {
                    syntax,
                })
            }
            TS_ENUM_DECLARATION => {
                JsAnyDeclaration::TsEnumDeclaration(TsEnumDeclaration { syntax })
            }
            TS_EXTERNAL_MODULE_DECLARATION => {
                JsAnyDeclaration::TsExternalModuleDeclaration(TsExternalModuleDeclaration {
                    syntax,
                })
            }
            TS_GLOBAL_DECLARATION => {
                JsAnyDeclaration::TsGlobalDeclaration(TsGlobalDeclaration { syntax })
            }
            TS_INTERFACE_DECLARATION => {
                JsAnyDeclaration::TsInterfaceDeclaration(TsInterfaceDeclaration { syntax })
            }
            TS_MODULE_DECLARATION => {
                JsAnyDeclaration::TsModuleDeclaration(TsModuleDeclaration { syntax })
            }
            TS_TYPE_ALIAS_DECLARATION => {
                JsAnyDeclaration::TsTypeAliasDeclaration(TsTypeAliasDeclaration { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyDeclaration::JsClassDeclaration(it) => &it.syntax,
            JsAnyDeclaration::JsFunctionDeclaration(it) => &it.syntax,
            JsAnyDeclaration::JsVariableDeclaration(it) => &it.syntax,
            JsAnyDeclaration::TsDeclareFunctionDeclaration(it) => &it.syntax,
            JsAnyDeclaration::TsEnumDeclaration(it) => &it.syntax,
            JsAnyDeclaration::TsExternalModuleDeclaration(it) => &it.syntax,
            JsAnyDeclaration::TsGlobalDeclaration(it) => &it.syntax,
            JsAnyDeclaration::TsInterfaceDeclaration(it) => &it.syntax,
            JsAnyDeclaration::TsModuleDeclaration(it) => &it.syntax,
            JsAnyDeclaration::TsTypeAliasDeclaration(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyDeclaration::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::JsVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::TsDeclareFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::TsEnumDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::TsExternalModuleDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::TsGlobalDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::TsInterfaceDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::TsModuleDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclaration::TsTypeAliasDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyDeclaration> for SyntaxNode {
    fn from(n: JsAnyDeclaration) -> SyntaxNode {
        match n {
            JsAnyDeclaration::JsClassDeclaration(it) => it.into(),
            JsAnyDeclaration::JsFunctionDeclaration(it) => it.into(),
            JsAnyDeclaration::JsVariableDeclaration(it) => it.into(),
            JsAnyDeclaration::TsDeclareFunctionDeclaration(it) => it.into(),
            JsAnyDeclaration::TsEnumDeclaration(it) => it.into(),
            JsAnyDeclaration::TsExternalModuleDeclaration(it) => it.into(),
            JsAnyDeclaration::TsGlobalDeclaration(it) => it.into(),
            JsAnyDeclaration::TsInterfaceDeclaration(it) => it.into(),
            JsAnyDeclaration::TsModuleDeclaration(it) => it.into(),
            JsAnyDeclaration::TsTypeAliasDeclaration(it) => it.into(),
        }
    }
}
impl From<JsAnyDeclaration> for SyntaxElement {
    fn from(n: JsAnyDeclaration) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsClassDeclaration> for JsAnyDeclarationClause {
    fn from(node: JsClassDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::JsClassDeclaration(node)
    }
}
impl From<JsFunctionDeclaration> for JsAnyDeclarationClause {
    fn from(node: JsFunctionDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::JsFunctionDeclaration(node)
    }
}
impl From<JsVariableDeclarationClause> for JsAnyDeclarationClause {
    fn from(node: JsVariableDeclarationClause) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::JsVariableDeclarationClause(node)
    }
}
impl From<TsDeclareFunctionDeclaration> for JsAnyDeclarationClause {
    fn from(node: TsDeclareFunctionDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::TsDeclareFunctionDeclaration(node)
    }
}
impl From<TsEnumDeclaration> for JsAnyDeclarationClause {
    fn from(node: TsEnumDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::TsEnumDeclaration(node)
    }
}
impl From<TsExternalModuleDeclaration> for JsAnyDeclarationClause {
    fn from(node: TsExternalModuleDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::TsExternalModuleDeclaration(node)
    }
}
impl From<TsGlobalDeclaration> for JsAnyDeclarationClause {
    fn from(node: TsGlobalDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::TsGlobalDeclaration(node)
    }
}
impl From<TsInterfaceDeclaration> for JsAnyDeclarationClause {
    fn from(node: TsInterfaceDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::TsInterfaceDeclaration(node)
    }
}
impl From<TsModuleDeclaration> for JsAnyDeclarationClause {
    fn from(node: TsModuleDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::TsModuleDeclaration(node)
    }
}
impl From<TsTypeAliasDeclaration> for JsAnyDeclarationClause {
    fn from(node: TsTypeAliasDeclaration) -> JsAnyDeclarationClause {
        JsAnyDeclarationClause::TsTypeAliasDeclaration(node)
    }
}
impl AstNode for JsAnyDeclarationClause {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_CLASS_DECLARATION
                | JS_FUNCTION_DECLARATION
                | JS_VARIABLE_DECLARATION_CLAUSE
                | TS_DECLARE_FUNCTION_DECLARATION
                | TS_ENUM_DECLARATION
                | TS_EXTERNAL_MODULE_DECLARATION
                | TS_GLOBAL_DECLARATION
                | TS_INTERFACE_DECLARATION
                | TS_MODULE_DECLARATION
                | TS_TYPE_ALIAS_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_CLASS_DECLARATION => {
                JsAnyDeclarationClause::JsClassDeclaration(JsClassDeclaration { syntax })
            }
            JS_FUNCTION_DECLARATION => {
                JsAnyDeclarationClause::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
            }
            JS_VARIABLE_DECLARATION_CLAUSE => {
                JsAnyDeclarationClause::JsVariableDeclarationClause(JsVariableDeclarationClause {
                    syntax,
                })
            }
            TS_DECLARE_FUNCTION_DECLARATION => {
                JsAnyDeclarationClause::TsDeclareFunctionDeclaration(TsDeclareFunctionDeclaration {
                    syntax,
                })
            }
            TS_ENUM_DECLARATION => {
                JsAnyDeclarationClause::TsEnumDeclaration(TsEnumDeclaration { syntax })
            }
            TS_EXTERNAL_MODULE_DECLARATION => {
                JsAnyDeclarationClause::TsExternalModuleDeclaration(TsExternalModuleDeclaration {
                    syntax,
                })
            }
            TS_GLOBAL_DECLARATION => {
                JsAnyDeclarationClause::TsGlobalDeclaration(TsGlobalDeclaration { syntax })
            }
            TS_INTERFACE_DECLARATION => {
                JsAnyDeclarationClause::TsInterfaceDeclaration(TsInterfaceDeclaration { syntax })
            }
            TS_MODULE_DECLARATION => {
                JsAnyDeclarationClause::TsModuleDeclaration(TsModuleDeclaration { syntax })
            }
            TS_TYPE_ALIAS_DECLARATION => {
                JsAnyDeclarationClause::TsTypeAliasDeclaration(TsTypeAliasDeclaration { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyDeclarationClause::JsClassDeclaration(it) => &it.syntax,
            JsAnyDeclarationClause::JsFunctionDeclaration(it) => &it.syntax,
            JsAnyDeclarationClause::JsVariableDeclarationClause(it) => &it.syntax,
            JsAnyDeclarationClause::TsDeclareFunctionDeclaration(it) => &it.syntax,
            JsAnyDeclarationClause::TsEnumDeclaration(it) => &it.syntax,
            JsAnyDeclarationClause::TsExternalModuleDeclaration(it) => &it.syntax,
            JsAnyDeclarationClause::TsGlobalDeclaration(it) => &it.syntax,
            JsAnyDeclarationClause::TsInterfaceDeclaration(it) => &it.syntax,
            JsAnyDeclarationClause::TsModuleDeclaration(it) => &it.syntax,
            JsAnyDeclarationClause::TsTypeAliasDeclaration(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyDeclarationClause::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::JsVariableDeclarationClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::TsDeclareFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::TsEnumDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::TsExternalModuleDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::TsGlobalDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::TsInterfaceDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::TsModuleDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyDeclarationClause::TsTypeAliasDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyDeclarationClause> for SyntaxNode {
    fn from(n: JsAnyDeclarationClause) -> SyntaxNode {
        match n {
            JsAnyDeclarationClause::JsClassDeclaration(it) => it.into(),
            JsAnyDeclarationClause::JsFunctionDeclaration(it) => it.into(),
            JsAnyDeclarationClause::JsVariableDeclarationClause(it) => it.into(),
            JsAnyDeclarationClause::TsDeclareFunctionDeclaration(it) => it.into(),
            JsAnyDeclarationClause::TsEnumDeclaration(it) => it.into(),
            JsAnyDeclarationClause::TsExternalModuleDeclaration(it) => it.into(),
            JsAnyDeclarationClause::TsGlobalDeclaration(it) => it.into(),
            JsAnyDeclarationClause::TsInterfaceDeclaration(it) => it.into(),
            JsAnyDeclarationClause::TsModuleDeclaration(it) => it.into(),
            JsAnyDeclarationClause::TsTypeAliasDeclaration(it) => it.into(),
        }
    }
}
impl From<JsAnyDeclarationClause> for SyntaxElement {
    fn from(n: JsAnyDeclarationClause) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsExportDefaultClassClause> for JsAnyExportClause {
    fn from(node: JsExportDefaultClassClause) -> JsAnyExportClause {
        JsAnyExportClause::JsExportDefaultClassClause(node)
    }
}
impl From<JsExportDefaultExpressionClause> for JsAnyExportClause {
    fn from(node: JsExportDefaultExpressionClause) -> JsAnyExportClause {
        JsAnyExportClause::JsExportDefaultExpressionClause(node)
    }
}
impl From<JsExportDefaultFunctionClause> for JsAnyExportClause {
    fn from(node: JsExportDefaultFunctionClause) -> JsAnyExportClause {
        JsAnyExportClause::JsExportDefaultFunctionClause(node)
    }
}
impl From<JsExportFromClause> for JsAnyExportClause {
    fn from(node: JsExportFromClause) -> JsAnyExportClause {
        JsAnyExportClause::JsExportFromClause(node)
    }
}
impl From<JsExportNamedClause> for JsAnyExportClause {
    fn from(node: JsExportNamedClause) -> JsAnyExportClause {
        JsAnyExportClause::JsExportNamedClause(node)
    }
}
impl From<JsExportNamedFromClause> for JsAnyExportClause {
    fn from(node: JsExportNamedFromClause) -> JsAnyExportClause {
        JsAnyExportClause::JsExportNamedFromClause(node)
    }
}
impl AstNode for JsAnyExportClause {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_EXPORT_DEFAULT_CLASS_CLAUSE
            | JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
            | JS_EXPORT_DEFAULT_FUNCTION_CLAUSE
            | JS_EXPORT_FROM_CLAUSE
            | JS_EXPORT_NAMED_CLAUSE
            | JS_EXPORT_NAMED_FROM_CLAUSE => true,
            k if JsAnyDeclarationClause::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_EXPORT_DEFAULT_CLASS_CLAUSE => {
                JsAnyExportClause::JsExportDefaultClassClause(JsExportDefaultClassClause { syntax })
            }
            JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
                JsAnyExportClause::JsExportDefaultExpressionClause(
                    JsExportDefaultExpressionClause { syntax },
                )
            }
            JS_EXPORT_DEFAULT_FUNCTION_CLAUSE => {
                JsAnyExportClause::JsExportDefaultFunctionClause(JsExportDefaultFunctionClause {
                    syntax,
                })
            }
            JS_EXPORT_FROM_CLAUSE => {
                JsAnyExportClause::JsExportFromClause(JsExportFromClause { syntax })
            }
            JS_EXPORT_NAMED_CLAUSE => {
                JsAnyExportClause::JsExportNamedClause(JsExportNamedClause { syntax })
            }
            JS_EXPORT_NAMED_FROM_CLAUSE => {
                JsAnyExportClause::JsExportNamedFromClause(JsExportNamedFromClause { syntax })
            }
            _ => {
                if let Some(js_any_declaration_clause) = JsAnyDeclarationClause::cast(syntax) {
                    return Some(JsAnyExportClause::JsAnyDeclarationClause(
                        js_any_declaration_clause,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyExportClause::JsExportDefaultClassClause(it) => &it.syntax,
            JsAnyExportClause::JsExportDefaultExpressionClause(it) => &it.syntax,
            JsAnyExportClause::JsExportDefaultFunctionClause(it) => &it.syntax,
            JsAnyExportClause::JsExportFromClause(it) => &it.syntax,
            JsAnyExportClause::JsExportNamedClause(it) => &it.syntax,
            JsAnyExportClause::JsExportNamedFromClause(it) => &it.syntax,
            JsAnyExportClause::JsAnyDeclarationClause(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyExportClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyExportClause::JsAnyDeclarationClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExportClause::JsExportDefaultClassClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExportClause::JsExportDefaultExpressionClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExportClause::JsExportDefaultFunctionClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExportClause::JsExportFromClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExportClause::JsExportNamedClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExportClause::JsExportNamedFromClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyExportClause> for SyntaxNode {
    fn from(n: JsAnyExportClause) -> SyntaxNode {
        match n {
            JsAnyExportClause::JsAnyDeclarationClause(it) => it.into(),
            JsAnyExportClause::JsExportDefaultClassClause(it) => it.into(),
            JsAnyExportClause::JsExportDefaultExpressionClause(it) => it.into(),
            JsAnyExportClause::JsExportDefaultFunctionClause(it) => it.into(),
            JsAnyExportClause::JsExportFromClause(it) => it.into(),
            JsAnyExportClause::JsExportNamedClause(it) => it.into(),
            JsAnyExportClause::JsExportNamedFromClause(it) => it.into(),
        }
    }
}
impl From<JsAnyExportClause> for SyntaxElement {
    fn from(n: JsAnyExportClause) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsExportNamedShorthandSpecifier> for JsAnyExportNamedSpecifier {
    fn from(node: JsExportNamedShorthandSpecifier) -> JsAnyExportNamedSpecifier {
        JsAnyExportNamedSpecifier::JsExportNamedShorthandSpecifier(node)
    }
}
impl From<JsExportNamedSpecifier> for JsAnyExportNamedSpecifier {
    fn from(node: JsExportNamedSpecifier) -> JsAnyExportNamedSpecifier {
        JsAnyExportNamedSpecifier::JsExportNamedSpecifier(node)
    }
}
impl AstNode for JsAnyExportNamedSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_EXPORT_NAMED_SHORTHAND_SPECIFIER | JS_EXPORT_NAMED_SPECIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
                JsAnyExportNamedSpecifier::JsExportNamedShorthandSpecifier(
                    JsExportNamedShorthandSpecifier { syntax },
                )
            }
            JS_EXPORT_NAMED_SPECIFIER => {
                JsAnyExportNamedSpecifier::JsExportNamedSpecifier(JsExportNamedSpecifier { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyExportNamedSpecifier::JsExportNamedShorthandSpecifier(it) => &it.syntax,
            JsAnyExportNamedSpecifier::JsExportNamedSpecifier(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyExportNamedSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyExportNamedSpecifier::JsExportNamedShorthandSpecifier(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyExportNamedSpecifier::JsExportNamedSpecifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyExportNamedSpecifier> for SyntaxNode {
    fn from(n: JsAnyExportNamedSpecifier) -> SyntaxNode {
        match n {
            JsAnyExportNamedSpecifier::JsExportNamedShorthandSpecifier(it) => it.into(),
            JsAnyExportNamedSpecifier::JsExportNamedSpecifier(it) => it.into(),
        }
    }
}
impl From<JsAnyExportNamedSpecifier> for SyntaxElement {
    fn from(n: JsAnyExportNamedSpecifier) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<ImportMeta> for JsAnyExpression {
    fn from(node: ImportMeta) -> JsAnyExpression { JsAnyExpression::ImportMeta(node) }
}
impl From<JsArrayExpression> for JsAnyExpression {
    fn from(node: JsArrayExpression) -> JsAnyExpression { JsAnyExpression::JsArrayExpression(node) }
}
impl From<JsArrowFunctionExpression> for JsAnyExpression {
    fn from(node: JsArrowFunctionExpression) -> JsAnyExpression {
        JsAnyExpression::JsArrowFunctionExpression(node)
    }
}
impl From<JsAssignmentExpression> for JsAnyExpression {
    fn from(node: JsAssignmentExpression) -> JsAnyExpression {
        JsAnyExpression::JsAssignmentExpression(node)
    }
}
impl From<JsAwaitExpression> for JsAnyExpression {
    fn from(node: JsAwaitExpression) -> JsAnyExpression { JsAnyExpression::JsAwaitExpression(node) }
}
impl From<JsBinaryExpression> for JsAnyExpression {
    fn from(node: JsBinaryExpression) -> JsAnyExpression {
        JsAnyExpression::JsBinaryExpression(node)
    }
}
impl From<JsCallExpression> for JsAnyExpression {
    fn from(node: JsCallExpression) -> JsAnyExpression { JsAnyExpression::JsCallExpression(node) }
}
impl From<JsClassExpression> for JsAnyExpression {
    fn from(node: JsClassExpression) -> JsAnyExpression { JsAnyExpression::JsClassExpression(node) }
}
impl From<JsComputedMemberExpression> for JsAnyExpression {
    fn from(node: JsComputedMemberExpression) -> JsAnyExpression {
        JsAnyExpression::JsComputedMemberExpression(node)
    }
}
impl From<JsConditionalExpression> for JsAnyExpression {
    fn from(node: JsConditionalExpression) -> JsAnyExpression {
        JsAnyExpression::JsConditionalExpression(node)
    }
}
impl From<JsFunctionExpression> for JsAnyExpression {
    fn from(node: JsFunctionExpression) -> JsAnyExpression {
        JsAnyExpression::JsFunctionExpression(node)
    }
}
impl From<JsIdentifierExpression> for JsAnyExpression {
    fn from(node: JsIdentifierExpression) -> JsAnyExpression {
        JsAnyExpression::JsIdentifierExpression(node)
    }
}
impl From<JsImportCallExpression> for JsAnyExpression {
    fn from(node: JsImportCallExpression) -> JsAnyExpression {
        JsAnyExpression::JsImportCallExpression(node)
    }
}
impl From<JsInExpression> for JsAnyExpression {
    fn from(node: JsInExpression) -> JsAnyExpression { JsAnyExpression::JsInExpression(node) }
}
impl From<JsInstanceofExpression> for JsAnyExpression {
    fn from(node: JsInstanceofExpression) -> JsAnyExpression {
        JsAnyExpression::JsInstanceofExpression(node)
    }
}
impl From<JsLogicalExpression> for JsAnyExpression {
    fn from(node: JsLogicalExpression) -> JsAnyExpression {
        JsAnyExpression::JsLogicalExpression(node)
    }
}
impl From<JsNewExpression> for JsAnyExpression {
    fn from(node: JsNewExpression) -> JsAnyExpression { JsAnyExpression::JsNewExpression(node) }
}
impl From<JsObjectExpression> for JsAnyExpression {
    fn from(node: JsObjectExpression) -> JsAnyExpression {
        JsAnyExpression::JsObjectExpression(node)
    }
}
impl From<JsParenthesizedExpression> for JsAnyExpression {
    fn from(node: JsParenthesizedExpression) -> JsAnyExpression {
        JsAnyExpression::JsParenthesizedExpression(node)
    }
}
impl From<JsPostUpdateExpression> for JsAnyExpression {
    fn from(node: JsPostUpdateExpression) -> JsAnyExpression {
        JsAnyExpression::JsPostUpdateExpression(node)
    }
}
impl From<JsPreUpdateExpression> for JsAnyExpression {
    fn from(node: JsPreUpdateExpression) -> JsAnyExpression {
        JsAnyExpression::JsPreUpdateExpression(node)
    }
}
impl From<JsSequenceExpression> for JsAnyExpression {
    fn from(node: JsSequenceExpression) -> JsAnyExpression {
        JsAnyExpression::JsSequenceExpression(node)
    }
}
impl From<JsStaticMemberExpression> for JsAnyExpression {
    fn from(node: JsStaticMemberExpression) -> JsAnyExpression {
        JsAnyExpression::JsStaticMemberExpression(node)
    }
}
impl From<JsSuperExpression> for JsAnyExpression {
    fn from(node: JsSuperExpression) -> JsAnyExpression { JsAnyExpression::JsSuperExpression(node) }
}
impl From<JsTemplate> for JsAnyExpression {
    fn from(node: JsTemplate) -> JsAnyExpression { JsAnyExpression::JsTemplate(node) }
}
impl From<JsThisExpression> for JsAnyExpression {
    fn from(node: JsThisExpression) -> JsAnyExpression { JsAnyExpression::JsThisExpression(node) }
}
impl From<JsUnaryExpression> for JsAnyExpression {
    fn from(node: JsUnaryExpression) -> JsAnyExpression { JsAnyExpression::JsUnaryExpression(node) }
}
impl From<JsUnknownExpression> for JsAnyExpression {
    fn from(node: JsUnknownExpression) -> JsAnyExpression {
        JsAnyExpression::JsUnknownExpression(node)
    }
}
impl From<JsYieldExpression> for JsAnyExpression {
    fn from(node: JsYieldExpression) -> JsAnyExpression { JsAnyExpression::JsYieldExpression(node) }
}
impl From<NewTarget> for JsAnyExpression {
    fn from(node: NewTarget) -> JsAnyExpression { JsAnyExpression::NewTarget(node) }
}
impl From<TsAsExpression> for JsAnyExpression {
    fn from(node: TsAsExpression) -> JsAnyExpression { JsAnyExpression::TsAsExpression(node) }
}
impl From<TsNonNullAssertionExpression> for JsAnyExpression {
    fn from(node: TsNonNullAssertionExpression) -> JsAnyExpression {
        JsAnyExpression::TsNonNullAssertionExpression(node)
    }
}
impl From<TsTypeAssertionExpression> for JsAnyExpression {
    fn from(node: TsTypeAssertionExpression) -> JsAnyExpression {
        JsAnyExpression::TsTypeAssertionExpression(node)
    }
}
impl AstNode for JsAnyExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            IMPORT_META
            | JS_ARRAY_EXPRESSION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_ASSIGNMENT_EXPRESSION
            | JS_AWAIT_EXPRESSION
            | JS_BINARY_EXPRESSION
            | JS_CALL_EXPRESSION
            | JS_CLASS_EXPRESSION
            | JS_COMPUTED_MEMBER_EXPRESSION
            | JS_CONDITIONAL_EXPRESSION
            | JS_FUNCTION_EXPRESSION
            | JS_IDENTIFIER_EXPRESSION
            | JS_IMPORT_CALL_EXPRESSION
            | JS_IN_EXPRESSION
            | JS_INSTANCEOF_EXPRESSION
            | JS_LOGICAL_EXPRESSION
            | JS_NEW_EXPRESSION
            | JS_OBJECT_EXPRESSION
            | JS_PARENTHESIZED_EXPRESSION
            | JS_POST_UPDATE_EXPRESSION
            | JS_PRE_UPDATE_EXPRESSION
            | JS_SEQUENCE_EXPRESSION
            | JS_STATIC_MEMBER_EXPRESSION
            | JS_SUPER_EXPRESSION
            | JS_TEMPLATE
            | JS_THIS_EXPRESSION
            | JS_UNARY_EXPRESSION
            | JS_UNKNOWN_EXPRESSION
            | JS_YIELD_EXPRESSION
            | NEW_TARGET
            | TS_AS_EXPRESSION
            | TS_NON_NULL_ASSERTION_EXPRESSION
            | TS_TYPE_ASSERTION_EXPRESSION => true,
            k if JsAnyLiteralExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            IMPORT_META => JsAnyExpression::ImportMeta(ImportMeta { syntax }),
            JS_ARRAY_EXPRESSION => JsAnyExpression::JsArrayExpression(JsArrayExpression { syntax }),
            JS_ARROW_FUNCTION_EXPRESSION => {
                JsAnyExpression::JsArrowFunctionExpression(JsArrowFunctionExpression { syntax })
            }
            JS_ASSIGNMENT_EXPRESSION => {
                JsAnyExpression::JsAssignmentExpression(JsAssignmentExpression { syntax })
            }
            JS_AWAIT_EXPRESSION => JsAnyExpression::JsAwaitExpression(JsAwaitExpression { syntax }),
            JS_BINARY_EXPRESSION => {
                JsAnyExpression::JsBinaryExpression(JsBinaryExpression { syntax })
            }
            JS_CALL_EXPRESSION => JsAnyExpression::JsCallExpression(JsCallExpression { syntax }),
            JS_CLASS_EXPRESSION => JsAnyExpression::JsClassExpression(JsClassExpression { syntax }),
            JS_COMPUTED_MEMBER_EXPRESSION => {
                JsAnyExpression::JsComputedMemberExpression(JsComputedMemberExpression { syntax })
            }
            JS_CONDITIONAL_EXPRESSION => {
                JsAnyExpression::JsConditionalExpression(JsConditionalExpression { syntax })
            }
            JS_FUNCTION_EXPRESSION => {
                JsAnyExpression::JsFunctionExpression(JsFunctionExpression { syntax })
            }
            JS_IDENTIFIER_EXPRESSION => {
                JsAnyExpression::JsIdentifierExpression(JsIdentifierExpression { syntax })
            }
            JS_IMPORT_CALL_EXPRESSION => {
                JsAnyExpression::JsImportCallExpression(JsImportCallExpression { syntax })
            }
            JS_IN_EXPRESSION => JsAnyExpression::JsInExpression(JsInExpression { syntax }),
            JS_INSTANCEOF_EXPRESSION => {
                JsAnyExpression::JsInstanceofExpression(JsInstanceofExpression { syntax })
            }
            JS_LOGICAL_EXPRESSION => {
                JsAnyExpression::JsLogicalExpression(JsLogicalExpression { syntax })
            }
            JS_NEW_EXPRESSION => JsAnyExpression::JsNewExpression(JsNewExpression { syntax }),
            JS_OBJECT_EXPRESSION => {
                JsAnyExpression::JsObjectExpression(JsObjectExpression { syntax })
            }
            JS_PARENTHESIZED_EXPRESSION => {
                JsAnyExpression::JsParenthesizedExpression(JsParenthesizedExpression { syntax })
            }
            JS_POST_UPDATE_EXPRESSION => {
                JsAnyExpression::JsPostUpdateExpression(JsPostUpdateExpression { syntax })
            }
            JS_PRE_UPDATE_EXPRESSION => {
                JsAnyExpression::JsPreUpdateExpression(JsPreUpdateExpression { syntax })
            }
            JS_SEQUENCE_EXPRESSION => {
                JsAnyExpression::JsSequenceExpression(JsSequenceExpression { syntax })
            }
            JS_STATIC_MEMBER_EXPRESSION => {
                JsAnyExpression::JsStaticMemberExpression(JsStaticMemberExpression { syntax })
            }
            JS_SUPER_EXPRESSION => JsAnyExpression::JsSuperExpression(JsSuperExpression { syntax }),
            JS_TEMPLATE => JsAnyExpression::JsTemplate(JsTemplate { syntax }),
            JS_THIS_EXPRESSION => JsAnyExpression::JsThisExpression(JsThisExpression { syntax }),
            JS_UNARY_EXPRESSION => JsAnyExpression::JsUnaryExpression(JsUnaryExpression { syntax }),
            JS_UNKNOWN_EXPRESSION => {
                JsAnyExpression::JsUnknownExpression(JsUnknownExpression { syntax })
            }
            JS_YIELD_EXPRESSION => JsAnyExpression::JsYieldExpression(JsYieldExpression { syntax }),
            NEW_TARGET => JsAnyExpression::NewTarget(NewTarget { syntax }),
            TS_AS_EXPRESSION => JsAnyExpression::TsAsExpression(TsAsExpression { syntax }),
            TS_NON_NULL_ASSERTION_EXPRESSION => {
                JsAnyExpression::TsNonNullAssertionExpression(TsNonNullAssertionExpression {
                    syntax,
                })
            }
            TS_TYPE_ASSERTION_EXPRESSION => {
                JsAnyExpression::TsTypeAssertionExpression(TsTypeAssertionExpression { syntax })
            }
            _ => {
                if let Some(js_any_literal_expression) = JsAnyLiteralExpression::cast(syntax) {
                    return Some(JsAnyExpression::JsAnyLiteralExpression(
                        js_any_literal_expression,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyExpression::ImportMeta(it) => &it.syntax,
            JsAnyExpression::JsArrayExpression(it) => &it.syntax,
            JsAnyExpression::JsArrowFunctionExpression(it) => &it.syntax,
            JsAnyExpression::JsAssignmentExpression(it) => &it.syntax,
            JsAnyExpression::JsAwaitExpression(it) => &it.syntax,
            JsAnyExpression::JsBinaryExpression(it) => &it.syntax,
            JsAnyExpression::JsCallExpression(it) => &it.syntax,
            JsAnyExpression::JsClassExpression(it) => &it.syntax,
            JsAnyExpression::JsComputedMemberExpression(it) => &it.syntax,
            JsAnyExpression::JsConditionalExpression(it) => &it.syntax,
            JsAnyExpression::JsFunctionExpression(it) => &it.syntax,
            JsAnyExpression::JsIdentifierExpression(it) => &it.syntax,
            JsAnyExpression::JsImportCallExpression(it) => &it.syntax,
            JsAnyExpression::JsInExpression(it) => &it.syntax,
            JsAnyExpression::JsInstanceofExpression(it) => &it.syntax,
            JsAnyExpression::JsLogicalExpression(it) => &it.syntax,
            JsAnyExpression::JsNewExpression(it) => &it.syntax,
            JsAnyExpression::JsObjectExpression(it) => &it.syntax,
            JsAnyExpression::JsParenthesizedExpression(it) => &it.syntax,
            JsAnyExpression::JsPostUpdateExpression(it) => &it.syntax,
            JsAnyExpression::JsPreUpdateExpression(it) => &it.syntax,
            JsAnyExpression::JsSequenceExpression(it) => &it.syntax,
            JsAnyExpression::JsStaticMemberExpression(it) => &it.syntax,
            JsAnyExpression::JsSuperExpression(it) => &it.syntax,
            JsAnyExpression::JsTemplate(it) => &it.syntax,
            JsAnyExpression::JsThisExpression(it) => &it.syntax,
            JsAnyExpression::JsUnaryExpression(it) => &it.syntax,
            JsAnyExpression::JsUnknownExpression(it) => &it.syntax,
            JsAnyExpression::JsYieldExpression(it) => &it.syntax,
            JsAnyExpression::NewTarget(it) => &it.syntax,
            JsAnyExpression::TsAsExpression(it) => &it.syntax,
            JsAnyExpression::TsNonNullAssertionExpression(it) => &it.syntax,
            JsAnyExpression::TsTypeAssertionExpression(it) => &it.syntax,
            JsAnyExpression::JsAnyLiteralExpression(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyExpression::ImportMeta(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsAnyLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsArrayExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsArrowFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsAssignmentExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsAwaitExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsCallExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsClassExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsComputedMemberExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsConditionalExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsIdentifierExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsImportCallExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsInExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsInstanceofExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsLogicalExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsNewExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsObjectExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsPostUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsPreUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsSequenceExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsStaticMemberExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsSuperExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsTemplate(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsThisExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsUnaryExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsUnknownExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::JsYieldExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::NewTarget(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::TsAsExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::TsNonNullAssertionExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyExpression::TsTypeAssertionExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyExpression> for SyntaxNode {
    fn from(n: JsAnyExpression) -> SyntaxNode {
        match n {
            JsAnyExpression::ImportMeta(it) => it.into(),
            JsAnyExpression::JsAnyLiteralExpression(it) => it.into(),
            JsAnyExpression::JsArrayExpression(it) => it.into(),
            JsAnyExpression::JsArrowFunctionExpression(it) => it.into(),
            JsAnyExpression::JsAssignmentExpression(it) => it.into(),
            JsAnyExpression::JsAwaitExpression(it) => it.into(),
            JsAnyExpression::JsBinaryExpression(it) => it.into(),
            JsAnyExpression::JsCallExpression(it) => it.into(),
            JsAnyExpression::JsClassExpression(it) => it.into(),
            JsAnyExpression::JsComputedMemberExpression(it) => it.into(),
            JsAnyExpression::JsConditionalExpression(it) => it.into(),
            JsAnyExpression::JsFunctionExpression(it) => it.into(),
            JsAnyExpression::JsIdentifierExpression(it) => it.into(),
            JsAnyExpression::JsImportCallExpression(it) => it.into(),
            JsAnyExpression::JsInExpression(it) => it.into(),
            JsAnyExpression::JsInstanceofExpression(it) => it.into(),
            JsAnyExpression::JsLogicalExpression(it) => it.into(),
            JsAnyExpression::JsNewExpression(it) => it.into(),
            JsAnyExpression::JsObjectExpression(it) => it.into(),
            JsAnyExpression::JsParenthesizedExpression(it) => it.into(),
            JsAnyExpression::JsPostUpdateExpression(it) => it.into(),
            JsAnyExpression::JsPreUpdateExpression(it) => it.into(),
            JsAnyExpression::JsSequenceExpression(it) => it.into(),
            JsAnyExpression::JsStaticMemberExpression(it) => it.into(),
            JsAnyExpression::JsSuperExpression(it) => it.into(),
            JsAnyExpression::JsTemplate(it) => it.into(),
            JsAnyExpression::JsThisExpression(it) => it.into(),
            JsAnyExpression::JsUnaryExpression(it) => it.into(),
            JsAnyExpression::JsUnknownExpression(it) => it.into(),
            JsAnyExpression::JsYieldExpression(it) => it.into(),
            JsAnyExpression::NewTarget(it) => it.into(),
            JsAnyExpression::TsAsExpression(it) => it.into(),
            JsAnyExpression::TsNonNullAssertionExpression(it) => it.into(),
            JsAnyExpression::TsTypeAssertionExpression(it) => it.into(),
        }
    }
}
impl From<JsAnyExpression> for SyntaxElement {
    fn from(n: JsAnyExpression) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsForVariableDeclaration> for JsAnyForInOrOfInitializer {
    fn from(node: JsForVariableDeclaration) -> JsAnyForInOrOfInitializer {
        JsAnyForInOrOfInitializer::JsForVariableDeclaration(node)
    }
}
impl AstNode for JsAnyForInOrOfInitializer {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_FOR_VARIABLE_DECLARATION => true,
            k if JsAnyAssignmentPattern::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_FOR_VARIABLE_DECLARATION => {
                JsAnyForInOrOfInitializer::JsForVariableDeclaration(JsForVariableDeclaration {
                    syntax,
                })
            }
            _ => {
                if let Some(js_any_assignment_pattern) = JsAnyAssignmentPattern::cast(syntax) {
                    return Some(JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(
                        js_any_assignment_pattern,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyForInOrOfInitializer::JsForVariableDeclaration(it) => &it.syntax,
            JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyForInOrOfInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(it) => std::fmt::Debug::fmt(it, f),
            JsAnyForInOrOfInitializer::JsForVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyForInOrOfInitializer> for SyntaxNode {
    fn from(n: JsAnyForInOrOfInitializer) -> SyntaxNode {
        match n {
            JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(it) => it.into(),
            JsAnyForInOrOfInitializer::JsForVariableDeclaration(it) => it.into(),
        }
    }
}
impl From<JsAnyForInOrOfInitializer> for SyntaxElement {
    fn from(n: JsAnyForInOrOfInitializer) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsVariableDeclaration> for JsAnyForInitializer {
    fn from(node: JsVariableDeclaration) -> JsAnyForInitializer {
        JsAnyForInitializer::JsVariableDeclaration(node)
    }
}
impl AstNode for JsAnyForInitializer {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_VARIABLE_DECLARATION => true,
            k if JsAnyExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_VARIABLE_DECLARATION => {
                JsAnyForInitializer::JsVariableDeclaration(JsVariableDeclaration { syntax })
            }
            _ => {
                if let Some(js_any_expression) = JsAnyExpression::cast(syntax) {
                    return Some(JsAnyForInitializer::JsAnyExpression(js_any_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyForInitializer::JsVariableDeclaration(it) => &it.syntax,
            JsAnyForInitializer::JsAnyExpression(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyForInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyForInitializer::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyForInitializer::JsVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyForInitializer> for SyntaxNode {
    fn from(n: JsAnyForInitializer) -> SyntaxNode {
        match n {
            JsAnyForInitializer::JsAnyExpression(it) => it.into(),
            JsAnyForInitializer::JsVariableDeclaration(it) => it.into(),
        }
    }
}
impl From<JsAnyForInitializer> for SyntaxElement {
    fn from(n: JsAnyForInitializer) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsFormalParameter> for JsAnyFormalParameter {
    fn from(node: JsFormalParameter) -> JsAnyFormalParameter {
        JsAnyFormalParameter::JsFormalParameter(node)
    }
}
impl From<JsUnknownParameter> for JsAnyFormalParameter {
    fn from(node: JsUnknownParameter) -> JsAnyFormalParameter {
        JsAnyFormalParameter::JsUnknownParameter(node)
    }
}
impl AstNode for JsAnyFormalParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, JS_FORMAL_PARAMETER | JS_UNKNOWN_PARAMETER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_FORMAL_PARAMETER => {
                JsAnyFormalParameter::JsFormalParameter(JsFormalParameter { syntax })
            }
            JS_UNKNOWN_PARAMETER => {
                JsAnyFormalParameter::JsUnknownParameter(JsUnknownParameter { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyFormalParameter::JsFormalParameter(it) => &it.syntax,
            JsAnyFormalParameter::JsUnknownParameter(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyFormalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyFormalParameter::JsFormalParameter(it) => std::fmt::Debug::fmt(it, f),
            JsAnyFormalParameter::JsUnknownParameter(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyFormalParameter> for SyntaxNode {
    fn from(n: JsAnyFormalParameter) -> SyntaxNode {
        match n {
            JsAnyFormalParameter::JsFormalParameter(it) => it.into(),
            JsAnyFormalParameter::JsUnknownParameter(it) => it.into(),
        }
    }
}
impl From<JsAnyFormalParameter> for SyntaxElement {
    fn from(n: JsAnyFormalParameter) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsArrowFunctionExpression> for JsAnyFunction {
    fn from(node: JsArrowFunctionExpression) -> JsAnyFunction {
        JsAnyFunction::JsArrowFunctionExpression(node)
    }
}
impl From<JsExportDefaultFunctionClause> for JsAnyFunction {
    fn from(node: JsExportDefaultFunctionClause) -> JsAnyFunction {
        JsAnyFunction::JsExportDefaultFunctionClause(node)
    }
}
impl From<JsFunctionDeclaration> for JsAnyFunction {
    fn from(node: JsFunctionDeclaration) -> JsAnyFunction {
        JsAnyFunction::JsFunctionDeclaration(node)
    }
}
impl From<JsFunctionExpression> for JsAnyFunction {
    fn from(node: JsFunctionExpression) -> JsAnyFunction {
        JsAnyFunction::JsFunctionExpression(node)
    }
}
impl AstNode for JsAnyFunction {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_ARROW_FUNCTION_EXPRESSION
                | JS_EXPORT_DEFAULT_FUNCTION_CLAUSE
                | JS_FUNCTION_DECLARATION
                | JS_FUNCTION_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_ARROW_FUNCTION_EXPRESSION => {
                JsAnyFunction::JsArrowFunctionExpression(JsArrowFunctionExpression { syntax })
            }
            JS_EXPORT_DEFAULT_FUNCTION_CLAUSE => {
                JsAnyFunction::JsExportDefaultFunctionClause(JsExportDefaultFunctionClause {
                    syntax,
                })
            }
            JS_FUNCTION_DECLARATION => {
                JsAnyFunction::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
            }
            JS_FUNCTION_EXPRESSION => {
                JsAnyFunction::JsFunctionExpression(JsFunctionExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(it) => &it.syntax,
            JsAnyFunction::JsExportDefaultFunctionClause(it) => &it.syntax,
            JsAnyFunction::JsFunctionDeclaration(it) => &it.syntax,
            JsAnyFunction::JsFunctionExpression(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyFunction::JsArrowFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyFunction::JsExportDefaultFunctionClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyFunction::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyFunction::JsFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyFunction> for SyntaxNode {
    fn from(n: JsAnyFunction) -> SyntaxNode {
        match n {
            JsAnyFunction::JsArrowFunctionExpression(it) => it.into(),
            JsAnyFunction::JsExportDefaultFunctionClause(it) => it.into(),
            JsAnyFunction::JsFunctionDeclaration(it) => it.into(),
            JsAnyFunction::JsFunctionExpression(it) => it.into(),
        }
    }
}
impl From<JsAnyFunction> for SyntaxElement {
    fn from(n: JsAnyFunction) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsFunctionBody> for JsAnyFunctionBody {
    fn from(node: JsFunctionBody) -> JsAnyFunctionBody { JsAnyFunctionBody::JsFunctionBody(node) }
}
impl AstNode for JsAnyFunctionBody {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_FUNCTION_BODY => true,
            k if JsAnyExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_FUNCTION_BODY => JsAnyFunctionBody::JsFunctionBody(JsFunctionBody { syntax }),
            _ => {
                if let Some(js_any_expression) = JsAnyExpression::cast(syntax) {
                    return Some(JsAnyFunctionBody::JsAnyExpression(js_any_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyFunctionBody::JsFunctionBody(it) => &it.syntax,
            JsAnyFunctionBody::JsAnyExpression(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyFunctionBody::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyFunctionBody::JsFunctionBody(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyFunctionBody> for SyntaxNode {
    fn from(n: JsAnyFunctionBody) -> SyntaxNode {
        match n {
            JsAnyFunctionBody::JsAnyExpression(it) => it.into(),
            JsAnyFunctionBody::JsFunctionBody(it) => it.into(),
        }
    }
}
impl From<JsAnyFunctionBody> for SyntaxElement {
    fn from(n: JsAnyFunctionBody) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsImportAssertionEntry> for JsAnyImportAssertionEntry {
    fn from(node: JsImportAssertionEntry) -> JsAnyImportAssertionEntry {
        JsAnyImportAssertionEntry::JsImportAssertionEntry(node)
    }
}
impl From<JsUnknownImportAssertionEntry> for JsAnyImportAssertionEntry {
    fn from(node: JsUnknownImportAssertionEntry) -> JsAnyImportAssertionEntry {
        JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(node)
    }
}
impl AstNode for JsAnyImportAssertionEntry {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_IMPORT_ASSERTION_ENTRY | JS_UNKNOWN_IMPORT_ASSERTION_ENTRY
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_IMPORT_ASSERTION_ENTRY => {
                JsAnyImportAssertionEntry::JsImportAssertionEntry(JsImportAssertionEntry { syntax })
            }
            JS_UNKNOWN_IMPORT_ASSERTION_ENTRY => {
                JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(
                    JsUnknownImportAssertionEntry { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyImportAssertionEntry::JsImportAssertionEntry(it) => &it.syntax,
            JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyImportAssertionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyImportAssertionEntry::JsImportAssertionEntry(it) => std::fmt::Debug::fmt(it, f),
            JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<JsAnyImportAssertionEntry> for SyntaxNode {
    fn from(n: JsAnyImportAssertionEntry) -> SyntaxNode {
        match n {
            JsAnyImportAssertionEntry::JsImportAssertionEntry(it) => it.into(),
            JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(it) => it.into(),
        }
    }
}
impl From<JsAnyImportAssertionEntry> for SyntaxElement {
    fn from(n: JsAnyImportAssertionEntry) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsImportBareClause> for JsAnyImportClause {
    fn from(node: JsImportBareClause) -> JsAnyImportClause {
        JsAnyImportClause::JsImportBareClause(node)
    }
}
impl From<JsImportDefaultClause> for JsAnyImportClause {
    fn from(node: JsImportDefaultClause) -> JsAnyImportClause {
        JsAnyImportClause::JsImportDefaultClause(node)
    }
}
impl From<JsImportNamedClause> for JsAnyImportClause {
    fn from(node: JsImportNamedClause) -> JsAnyImportClause {
        JsAnyImportClause::JsImportNamedClause(node)
    }
}
impl From<JsImportNamespaceClause> for JsAnyImportClause {
    fn from(node: JsImportNamespaceClause) -> JsAnyImportClause {
        JsAnyImportClause::JsImportNamespaceClause(node)
    }
}
impl AstNode for JsAnyImportClause {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_IMPORT_BARE_CLAUSE
                | JS_IMPORT_DEFAULT_CLAUSE
                | JS_IMPORT_NAMED_CLAUSE
                | JS_IMPORT_NAMESPACE_CLAUSE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_IMPORT_BARE_CLAUSE => {
                JsAnyImportClause::JsImportBareClause(JsImportBareClause { syntax })
            }
            JS_IMPORT_DEFAULT_CLAUSE => {
                JsAnyImportClause::JsImportDefaultClause(JsImportDefaultClause { syntax })
            }
            JS_IMPORT_NAMED_CLAUSE => {
                JsAnyImportClause::JsImportNamedClause(JsImportNamedClause { syntax })
            }
            JS_IMPORT_NAMESPACE_CLAUSE => {
                JsAnyImportClause::JsImportNamespaceClause(JsImportNamespaceClause { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyImportClause::JsImportBareClause(it) => &it.syntax,
            JsAnyImportClause::JsImportDefaultClause(it) => &it.syntax,
            JsAnyImportClause::JsImportNamedClause(it) => &it.syntax,
            JsAnyImportClause::JsImportNamespaceClause(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyImportClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyImportClause::JsImportBareClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyImportClause::JsImportDefaultClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyImportClause::JsImportNamedClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnyImportClause::JsImportNamespaceClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyImportClause> for SyntaxNode {
    fn from(n: JsAnyImportClause) -> SyntaxNode {
        match n {
            JsAnyImportClause::JsImportBareClause(it) => it.into(),
            JsAnyImportClause::JsImportDefaultClause(it) => it.into(),
            JsAnyImportClause::JsImportNamedClause(it) => it.into(),
            JsAnyImportClause::JsImportNamespaceClause(it) => it.into(),
        }
    }
}
impl From<JsAnyImportClause> for SyntaxElement {
    fn from(n: JsAnyImportClause) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsPrivateName> for JsAnyInProperty {
    fn from(node: JsPrivateName) -> JsAnyInProperty { JsAnyInProperty::JsPrivateName(node) }
}
impl AstNode for JsAnyInProperty {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_PRIVATE_NAME => true,
            k if JsAnyExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_PRIVATE_NAME => JsAnyInProperty::JsPrivateName(JsPrivateName { syntax }),
            _ => {
                if let Some(js_any_expression) = JsAnyExpression::cast(syntax) {
                    return Some(JsAnyInProperty::JsAnyExpression(js_any_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyInProperty::JsPrivateName(it) => &it.syntax,
            JsAnyInProperty::JsAnyExpression(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyInProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyInProperty::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyInProperty::JsPrivateName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyInProperty> for SyntaxNode {
    fn from(n: JsAnyInProperty) -> SyntaxNode {
        match n {
            JsAnyInProperty::JsAnyExpression(it) => it.into(),
            JsAnyInProperty::JsPrivateName(it) => it.into(),
        }
    }
}
impl From<JsAnyInProperty> for SyntaxElement {
    fn from(n: JsAnyInProperty) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsBigIntLiteralExpression> for JsAnyLiteralExpression {
    fn from(node: JsBigIntLiteralExpression) -> JsAnyLiteralExpression {
        JsAnyLiteralExpression::JsBigIntLiteralExpression(node)
    }
}
impl From<JsBooleanLiteralExpression> for JsAnyLiteralExpression {
    fn from(node: JsBooleanLiteralExpression) -> JsAnyLiteralExpression {
        JsAnyLiteralExpression::JsBooleanLiteralExpression(node)
    }
}
impl From<JsNullLiteralExpression> for JsAnyLiteralExpression {
    fn from(node: JsNullLiteralExpression) -> JsAnyLiteralExpression {
        JsAnyLiteralExpression::JsNullLiteralExpression(node)
    }
}
impl From<JsNumberLiteralExpression> for JsAnyLiteralExpression {
    fn from(node: JsNumberLiteralExpression) -> JsAnyLiteralExpression {
        JsAnyLiteralExpression::JsNumberLiteralExpression(node)
    }
}
impl From<JsRegexLiteralExpression> for JsAnyLiteralExpression {
    fn from(node: JsRegexLiteralExpression) -> JsAnyLiteralExpression {
        JsAnyLiteralExpression::JsRegexLiteralExpression(node)
    }
}
impl From<JsStringLiteralExpression> for JsAnyLiteralExpression {
    fn from(node: JsStringLiteralExpression) -> JsAnyLiteralExpression {
        JsAnyLiteralExpression::JsStringLiteralExpression(node)
    }
}
impl AstNode for JsAnyLiteralExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_BIG_INT_LITERAL_EXPRESSION
                | JS_BOOLEAN_LITERAL_EXPRESSION
                | JS_NULL_LITERAL_EXPRESSION
                | JS_NUMBER_LITERAL_EXPRESSION
                | JS_REGEX_LITERAL_EXPRESSION
                | JS_STRING_LITERAL_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_BIG_INT_LITERAL_EXPRESSION => {
                JsAnyLiteralExpression::JsBigIntLiteralExpression(JsBigIntLiteralExpression {
                    syntax,
                })
            }
            JS_BOOLEAN_LITERAL_EXPRESSION => {
                JsAnyLiteralExpression::JsBooleanLiteralExpression(JsBooleanLiteralExpression {
                    syntax,
                })
            }
            JS_NULL_LITERAL_EXPRESSION => {
                JsAnyLiteralExpression::JsNullLiteralExpression(JsNullLiteralExpression { syntax })
            }
            JS_NUMBER_LITERAL_EXPRESSION => {
                JsAnyLiteralExpression::JsNumberLiteralExpression(JsNumberLiteralExpression {
                    syntax,
                })
            }
            JS_REGEX_LITERAL_EXPRESSION => {
                JsAnyLiteralExpression::JsRegexLiteralExpression(JsRegexLiteralExpression {
                    syntax,
                })
            }
            JS_STRING_LITERAL_EXPRESSION => {
                JsAnyLiteralExpression::JsStringLiteralExpression(JsStringLiteralExpression {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyLiteralExpression::JsBigIntLiteralExpression(it) => &it.syntax,
            JsAnyLiteralExpression::JsBooleanLiteralExpression(it) => &it.syntax,
            JsAnyLiteralExpression::JsNullLiteralExpression(it) => &it.syntax,
            JsAnyLiteralExpression::JsNumberLiteralExpression(it) => &it.syntax,
            JsAnyLiteralExpression::JsRegexLiteralExpression(it) => &it.syntax,
            JsAnyLiteralExpression::JsStringLiteralExpression(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyLiteralExpression::JsBigIntLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyLiteralExpression::JsBooleanLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyLiteralExpression::JsNullLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyLiteralExpression::JsNumberLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyLiteralExpression::JsRegexLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            JsAnyLiteralExpression::JsStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyLiteralExpression> for SyntaxNode {
    fn from(n: JsAnyLiteralExpression) -> SyntaxNode {
        match n {
            JsAnyLiteralExpression::JsBigIntLiteralExpression(it) => it.into(),
            JsAnyLiteralExpression::JsBooleanLiteralExpression(it) => it.into(),
            JsAnyLiteralExpression::JsNullLiteralExpression(it) => it.into(),
            JsAnyLiteralExpression::JsNumberLiteralExpression(it) => it.into(),
            JsAnyLiteralExpression::JsRegexLiteralExpression(it) => it.into(),
            JsAnyLiteralExpression::JsStringLiteralExpression(it) => it.into(),
        }
    }
}
impl From<JsAnyLiteralExpression> for SyntaxElement {
    fn from(n: JsAnyLiteralExpression) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsExport> for JsAnyModuleItem {
    fn from(node: JsExport) -> JsAnyModuleItem { JsAnyModuleItem::JsExport(node) }
}
impl From<JsImport> for JsAnyModuleItem {
    fn from(node: JsImport) -> JsAnyModuleItem { JsAnyModuleItem::JsImport(node) }
}
impl AstNode for JsAnyModuleItem {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_EXPORT | JS_IMPORT => true,
            k if JsAnyStatement::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_EXPORT => JsAnyModuleItem::JsExport(JsExport { syntax }),
            JS_IMPORT => JsAnyModuleItem::JsImport(JsImport { syntax }),
            _ => {
                if let Some(js_any_statement) = JsAnyStatement::cast(syntax) {
                    return Some(JsAnyModuleItem::JsAnyStatement(js_any_statement));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyModuleItem::JsExport(it) => &it.syntax,
            JsAnyModuleItem::JsImport(it) => &it.syntax,
            JsAnyModuleItem::JsAnyStatement(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyModuleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyModuleItem::JsAnyStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyModuleItem::JsExport(it) => std::fmt::Debug::fmt(it, f),
            JsAnyModuleItem::JsImport(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyModuleItem> for SyntaxNode {
    fn from(n: JsAnyModuleItem) -> SyntaxNode {
        match n {
            JsAnyModuleItem::JsAnyStatement(it) => it.into(),
            JsAnyModuleItem::JsExport(it) => it.into(),
            JsAnyModuleItem::JsImport(it) => it.into(),
        }
    }
}
impl From<JsAnyModuleItem> for SyntaxElement {
    fn from(n: JsAnyModuleItem) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsName> for JsAnyName {
    fn from(node: JsName) -> JsAnyName { JsAnyName::JsName(node) }
}
impl From<JsPrivateName> for JsAnyName {
    fn from(node: JsPrivateName) -> JsAnyName { JsAnyName::JsPrivateName(node) }
}
impl AstNode for JsAnyName {
    fn can_cast(kind: JsSyntaxKind) -> bool { matches!(kind, JS_NAME | JS_PRIVATE_NAME) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_NAME => JsAnyName::JsName(JsName { syntax }),
            JS_PRIVATE_NAME => JsAnyName::JsPrivateName(JsPrivateName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyName::JsName(it) => &it.syntax,
            JsAnyName::JsPrivateName(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyName::JsName(it) => std::fmt::Debug::fmt(it, f),
            JsAnyName::JsPrivateName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyName> for SyntaxNode {
    fn from(n: JsAnyName) -> SyntaxNode {
        match n {
            JsAnyName::JsName(it) => it.into(),
            JsAnyName::JsPrivateName(it) => it.into(),
        }
    }
}
impl From<JsAnyName> for SyntaxElement {
    fn from(n: JsAnyName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsNamedImportSpecifiers> for JsAnyNamedImport {
    fn from(node: JsNamedImportSpecifiers) -> JsAnyNamedImport {
        JsAnyNamedImport::JsNamedImportSpecifiers(node)
    }
}
impl From<JsNamespaceImportSpecifier> for JsAnyNamedImport {
    fn from(node: JsNamespaceImportSpecifier) -> JsAnyNamedImport {
        JsAnyNamedImport::JsNamespaceImportSpecifier(node)
    }
}
impl AstNode for JsAnyNamedImport {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_NAMED_IMPORT_SPECIFIERS | JS_NAMESPACE_IMPORT_SPECIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_NAMED_IMPORT_SPECIFIERS => {
                JsAnyNamedImport::JsNamedImportSpecifiers(JsNamedImportSpecifiers { syntax })
            }
            JS_NAMESPACE_IMPORT_SPECIFIER => {
                JsAnyNamedImport::JsNamespaceImportSpecifier(JsNamespaceImportSpecifier { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyNamedImport::JsNamedImportSpecifiers(it) => &it.syntax,
            JsAnyNamedImport::JsNamespaceImportSpecifier(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyNamedImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyNamedImport::JsNamedImportSpecifiers(it) => std::fmt::Debug::fmt(it, f),
            JsAnyNamedImport::JsNamespaceImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyNamedImport> for SyntaxNode {
    fn from(n: JsAnyNamedImport) -> SyntaxNode {
        match n {
            JsAnyNamedImport::JsNamedImportSpecifiers(it) => it.into(),
            JsAnyNamedImport::JsNamespaceImportSpecifier(it) => it.into(),
        }
    }
}
impl From<JsAnyNamedImport> for SyntaxElement {
    fn from(n: JsAnyNamedImport) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsNamedImportSpecifier> for JsAnyNamedImportSpecifier {
    fn from(node: JsNamedImportSpecifier) -> JsAnyNamedImportSpecifier {
        JsAnyNamedImportSpecifier::JsNamedImportSpecifier(node)
    }
}
impl From<JsShorthandNamedImportSpecifier> for JsAnyNamedImportSpecifier {
    fn from(node: JsShorthandNamedImportSpecifier) -> JsAnyNamedImportSpecifier {
        JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(node)
    }
}
impl From<JsUnknownNamedImportSpecifier> for JsAnyNamedImportSpecifier {
    fn from(node: JsUnknownNamedImportSpecifier) -> JsAnyNamedImportSpecifier {
        JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(node)
    }
}
impl AstNode for JsAnyNamedImportSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_NAMED_IMPORT_SPECIFIER
                | JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
                | JS_UNKNOWN_NAMED_IMPORT_SPECIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_NAMED_IMPORT_SPECIFIER => {
                JsAnyNamedImportSpecifier::JsNamedImportSpecifier(JsNamedImportSpecifier { syntax })
            }
            JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(
                    JsShorthandNamedImportSpecifier { syntax },
                )
            }
            JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => {
                JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(
                    JsUnknownNamedImportSpecifier { syntax },
                )
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyNamedImportSpecifier::JsNamedImportSpecifier(it) => &it.syntax,
            JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(it) => &it.syntax,
            JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyNamedImportSpecifier::JsNamedImportSpecifier(it) => std::fmt::Debug::fmt(it, f),
            JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<JsAnyNamedImportSpecifier> for SyntaxNode {
    fn from(n: JsAnyNamedImportSpecifier) -> SyntaxNode {
        match n {
            JsAnyNamedImportSpecifier::JsNamedImportSpecifier(it) => it.into(),
            JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(it) => it.into(),
            JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(it) => it.into(),
        }
    }
}
impl From<JsAnyNamedImportSpecifier> for SyntaxElement {
    fn from(n: JsAnyNamedImportSpecifier) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsObjectAssignmentPatternProperty> for JsAnyObjectAssignmentPatternMember {
    fn from(node: JsObjectAssignmentPatternProperty) -> JsAnyObjectAssignmentPatternMember {
        JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(node)
    }
}
impl From<JsObjectAssignmentPatternRest> for JsAnyObjectAssignmentPatternMember {
    fn from(node: JsObjectAssignmentPatternRest) -> JsAnyObjectAssignmentPatternMember {
        JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(node)
    }
}
impl From<JsObjectAssignmentPatternShorthandProperty> for JsAnyObjectAssignmentPatternMember {
    fn from(
        node: JsObjectAssignmentPatternShorthandProperty,
    ) -> JsAnyObjectAssignmentPatternMember {
        JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(node)
    }
}
impl From<JsUnknownAssignment> for JsAnyObjectAssignmentPatternMember {
    fn from(node: JsUnknownAssignment) -> JsAnyObjectAssignmentPatternMember {
        JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(node)
    }
}
impl AstNode for JsAnyObjectAssignmentPatternMember {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
                | JS_OBJECT_ASSIGNMENT_PATTERN_REST
                | JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
                | JS_UNKNOWN_ASSIGNMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
                JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(
                    JsObjectAssignmentPatternProperty { syntax },
                )
            }
            JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
                JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(
                    JsObjectAssignmentPatternRest { syntax },
                )
            }
            JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
                JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                    JsObjectAssignmentPatternShorthandProperty { syntax },
                )
            }
            JS_UNKNOWN_ASSIGNMENT => {
                JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(JsUnknownAssignment {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(it) => &it.syntax,
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(it) => &it.syntax,
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(it) => {
                &it.syntax
            }
            JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyObjectAssignmentPatternMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(it) => {
                std::fmt::Debug::fmt(it, f)
            }
        }
    }
}
impl From<JsAnyObjectAssignmentPatternMember> for SyntaxNode {
    fn from(n: JsAnyObjectAssignmentPatternMember) -> SyntaxNode {
        match n {
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(it) => it.into(),
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(it) => it.into(),
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(it) => {
                it.into()
            }
            JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(it) => it.into(),
        }
    }
}
impl From<JsAnyObjectAssignmentPatternMember> for SyntaxElement {
    fn from(n: JsAnyObjectAssignmentPatternMember) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsIdentifierBinding> for JsAnyObjectBindingPatternMember {
    fn from(node: JsIdentifierBinding) -> JsAnyObjectBindingPatternMember {
        JsAnyObjectBindingPatternMember::JsIdentifierBinding(node)
    }
}
impl From<JsObjectBindingPatternProperty> for JsAnyObjectBindingPatternMember {
    fn from(node: JsObjectBindingPatternProperty) -> JsAnyObjectBindingPatternMember {
        JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(node)
    }
}
impl From<JsObjectBindingPatternRest> for JsAnyObjectBindingPatternMember {
    fn from(node: JsObjectBindingPatternRest) -> JsAnyObjectBindingPatternMember {
        JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(node)
    }
}
impl From<JsObjectBindingPatternShorthandProperty> for JsAnyObjectBindingPatternMember {
    fn from(node: JsObjectBindingPatternShorthandProperty) -> JsAnyObjectBindingPatternMember {
        JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(node)
    }
}
impl From<JsUnknownBinding> for JsAnyObjectBindingPatternMember {
    fn from(node: JsUnknownBinding) -> JsAnyObjectBindingPatternMember {
        JsAnyObjectBindingPatternMember::JsUnknownBinding(node)
    }
}
impl AstNode for JsAnyObjectBindingPatternMember {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_IDENTIFIER_BINDING
                | JS_OBJECT_BINDING_PATTERN_PROPERTY
                | JS_OBJECT_BINDING_PATTERN_REST
                | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
                | JS_UNKNOWN_BINDING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_IDENTIFIER_BINDING => {
                JsAnyObjectBindingPatternMember::JsIdentifierBinding(JsIdentifierBinding { syntax })
            }
            JS_OBJECT_BINDING_PATTERN_PROPERTY => {
                JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(
                    JsObjectBindingPatternProperty { syntax },
                )
            }
            JS_OBJECT_BINDING_PATTERN_REST => {
                JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(
                    JsObjectBindingPatternRest { syntax },
                )
            }
            JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
                JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                    JsObjectBindingPatternShorthandProperty { syntax },
                )
            }
            JS_UNKNOWN_BINDING => {
                JsAnyObjectBindingPatternMember::JsUnknownBinding(JsUnknownBinding { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyObjectBindingPatternMember::JsIdentifierBinding(it) => &it.syntax,
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(it) => &it.syntax,
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(it) => &it.syntax,
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(it) => {
                &it.syntax
            }
            JsAnyObjectBindingPatternMember::JsUnknownBinding(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyObjectBindingPatternMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyObjectBindingPatternMember::JsIdentifierBinding(it) => std::fmt::Debug::fmt(it, f),
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            JsAnyObjectBindingPatternMember::JsUnknownBinding(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyObjectBindingPatternMember> for SyntaxNode {
    fn from(n: JsAnyObjectBindingPatternMember) -> SyntaxNode {
        match n {
            JsAnyObjectBindingPatternMember::JsIdentifierBinding(it) => it.into(),
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(it) => it.into(),
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(it) => it.into(),
            JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(it) => {
                it.into()
            }
            JsAnyObjectBindingPatternMember::JsUnknownBinding(it) => it.into(),
        }
    }
}
impl From<JsAnyObjectBindingPatternMember> for SyntaxElement {
    fn from(n: JsAnyObjectBindingPatternMember) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsGetterObjectMember> for JsAnyObjectMember {
    fn from(node: JsGetterObjectMember) -> JsAnyObjectMember {
        JsAnyObjectMember::JsGetterObjectMember(node)
    }
}
impl From<JsMethodObjectMember> for JsAnyObjectMember {
    fn from(node: JsMethodObjectMember) -> JsAnyObjectMember {
        JsAnyObjectMember::JsMethodObjectMember(node)
    }
}
impl From<JsPropertyObjectMember> for JsAnyObjectMember {
    fn from(node: JsPropertyObjectMember) -> JsAnyObjectMember {
        JsAnyObjectMember::JsPropertyObjectMember(node)
    }
}
impl From<JsSetterObjectMember> for JsAnyObjectMember {
    fn from(node: JsSetterObjectMember) -> JsAnyObjectMember {
        JsAnyObjectMember::JsSetterObjectMember(node)
    }
}
impl From<JsShorthandPropertyObjectMember> for JsAnyObjectMember {
    fn from(node: JsShorthandPropertyObjectMember) -> JsAnyObjectMember {
        JsAnyObjectMember::JsShorthandPropertyObjectMember(node)
    }
}
impl From<JsSpread> for JsAnyObjectMember {
    fn from(node: JsSpread) -> JsAnyObjectMember { JsAnyObjectMember::JsSpread(node) }
}
impl From<JsUnknownMember> for JsAnyObjectMember {
    fn from(node: JsUnknownMember) -> JsAnyObjectMember { JsAnyObjectMember::JsUnknownMember(node) }
}
impl AstNode for JsAnyObjectMember {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_GETTER_OBJECT_MEMBER
                | JS_METHOD_OBJECT_MEMBER
                | JS_PROPERTY_OBJECT_MEMBER
                | JS_SETTER_OBJECT_MEMBER
                | JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
                | JS_SPREAD
                | JS_UNKNOWN_MEMBER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_GETTER_OBJECT_MEMBER => {
                JsAnyObjectMember::JsGetterObjectMember(JsGetterObjectMember { syntax })
            }
            JS_METHOD_OBJECT_MEMBER => {
                JsAnyObjectMember::JsMethodObjectMember(JsMethodObjectMember { syntax })
            }
            JS_PROPERTY_OBJECT_MEMBER => {
                JsAnyObjectMember::JsPropertyObjectMember(JsPropertyObjectMember { syntax })
            }
            JS_SETTER_OBJECT_MEMBER => {
                JsAnyObjectMember::JsSetterObjectMember(JsSetterObjectMember { syntax })
            }
            JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                JsAnyObjectMember::JsShorthandPropertyObjectMember(
                    JsShorthandPropertyObjectMember { syntax },
                )
            }
            JS_SPREAD => JsAnyObjectMember::JsSpread(JsSpread { syntax }),
            JS_UNKNOWN_MEMBER => JsAnyObjectMember::JsUnknownMember(JsUnknownMember { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyObjectMember::JsGetterObjectMember(it) => &it.syntax,
            JsAnyObjectMember::JsMethodObjectMember(it) => &it.syntax,
            JsAnyObjectMember::JsPropertyObjectMember(it) => &it.syntax,
            JsAnyObjectMember::JsSetterObjectMember(it) => &it.syntax,
            JsAnyObjectMember::JsShorthandPropertyObjectMember(it) => &it.syntax,
            JsAnyObjectMember::JsSpread(it) => &it.syntax,
            JsAnyObjectMember::JsUnknownMember(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyObjectMember::JsGetterObjectMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyObjectMember::JsMethodObjectMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyObjectMember::JsPropertyObjectMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyObjectMember::JsSetterObjectMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyObjectMember::JsShorthandPropertyObjectMember(it) => std::fmt::Debug::fmt(it, f),
            JsAnyObjectMember::JsSpread(it) => std::fmt::Debug::fmt(it, f),
            JsAnyObjectMember::JsUnknownMember(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyObjectMember> for SyntaxNode {
    fn from(n: JsAnyObjectMember) -> SyntaxNode {
        match n {
            JsAnyObjectMember::JsGetterObjectMember(it) => it.into(),
            JsAnyObjectMember::JsMethodObjectMember(it) => it.into(),
            JsAnyObjectMember::JsPropertyObjectMember(it) => it.into(),
            JsAnyObjectMember::JsSetterObjectMember(it) => it.into(),
            JsAnyObjectMember::JsShorthandPropertyObjectMember(it) => it.into(),
            JsAnyObjectMember::JsSpread(it) => it.into(),
            JsAnyObjectMember::JsUnknownMember(it) => it.into(),
        }
    }
}
impl From<JsAnyObjectMember> for SyntaxElement {
    fn from(n: JsAnyObjectMember) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsComputedMemberName> for JsAnyObjectMemberName {
    fn from(node: JsComputedMemberName) -> JsAnyObjectMemberName {
        JsAnyObjectMemberName::JsComputedMemberName(node)
    }
}
impl From<JsLiteralMemberName> for JsAnyObjectMemberName {
    fn from(node: JsLiteralMemberName) -> JsAnyObjectMemberName {
        JsAnyObjectMemberName::JsLiteralMemberName(node)
    }
}
impl AstNode for JsAnyObjectMemberName {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, JS_COMPUTED_MEMBER_NAME | JS_LITERAL_MEMBER_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_COMPUTED_MEMBER_NAME => {
                JsAnyObjectMemberName::JsComputedMemberName(JsComputedMemberName { syntax })
            }
            JS_LITERAL_MEMBER_NAME => {
                JsAnyObjectMemberName::JsLiteralMemberName(JsLiteralMemberName { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyObjectMemberName::JsComputedMemberName(it) => &it.syntax,
            JsAnyObjectMemberName::JsLiteralMemberName(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyObjectMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyObjectMemberName::JsComputedMemberName(it) => std::fmt::Debug::fmt(it, f),
            JsAnyObjectMemberName::JsLiteralMemberName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyObjectMemberName> for SyntaxNode {
    fn from(n: JsAnyObjectMemberName) -> SyntaxNode {
        match n {
            JsAnyObjectMemberName::JsComputedMemberName(it) => it.into(),
            JsAnyObjectMemberName::JsLiteralMemberName(it) => it.into(),
        }
    }
}
impl From<JsAnyObjectMemberName> for SyntaxElement {
    fn from(n: JsAnyObjectMemberName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsRestParameter> for JsAnyParameter {
    fn from(node: JsRestParameter) -> JsAnyParameter { JsAnyParameter::JsRestParameter(node) }
}
impl From<TsThisParameter> for JsAnyParameter {
    fn from(node: TsThisParameter) -> JsAnyParameter { JsAnyParameter::TsThisParameter(node) }
}
impl AstNode for JsAnyParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            JS_REST_PARAMETER | TS_THIS_PARAMETER => true,
            k if JsAnyFormalParameter::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_REST_PARAMETER => JsAnyParameter::JsRestParameter(JsRestParameter { syntax }),
            TS_THIS_PARAMETER => JsAnyParameter::TsThisParameter(TsThisParameter { syntax }),
            _ => {
                if let Some(js_any_formal_parameter) = JsAnyFormalParameter::cast(syntax) {
                    return Some(JsAnyParameter::JsAnyFormalParameter(
                        js_any_formal_parameter,
                    ));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyParameter::JsRestParameter(it) => &it.syntax,
            JsAnyParameter::TsThisParameter(it) => &it.syntax,
            JsAnyParameter::JsAnyFormalParameter(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for JsAnyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyParameter::JsAnyFormalParameter(it) => std::fmt::Debug::fmt(it, f),
            JsAnyParameter::JsRestParameter(it) => std::fmt::Debug::fmt(it, f),
            JsAnyParameter::TsThisParameter(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyParameter> for SyntaxNode {
    fn from(n: JsAnyParameter) -> SyntaxNode {
        match n {
            JsAnyParameter::JsAnyFormalParameter(it) => it.into(),
            JsAnyParameter::JsRestParameter(it) => it.into(),
            JsAnyParameter::TsThisParameter(it) => it.into(),
        }
    }
}
impl From<JsAnyParameter> for SyntaxElement {
    fn from(n: JsAnyParameter) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsExpressionSnipped> for JsAnyRoot {
    fn from(node: JsExpressionSnipped) -> JsAnyRoot { JsAnyRoot::JsExpressionSnipped(node) }
}
impl From<JsModule> for JsAnyRoot {
    fn from(node: JsModule) -> JsAnyRoot { JsAnyRoot::JsModule(node) }
}
impl From<JsScript> for JsAnyRoot {
    fn from(node: JsScript) -> JsAnyRoot { JsAnyRoot::JsScript(node) }
}
impl AstNode for JsAnyRoot {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, JS_EXPRESSION_SNIPPED | JS_MODULE | JS_SCRIPT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_EXPRESSION_SNIPPED => JsAnyRoot::JsExpressionSnipped(JsExpressionSnipped { syntax }),
            JS_MODULE => JsAnyRoot::JsModule(JsModule { syntax }),
            JS_SCRIPT => JsAnyRoot::JsScript(JsScript { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyRoot::JsExpressionSnipped(it) => &it.syntax,
            JsAnyRoot::JsModule(it) => &it.syntax,
            JsAnyRoot::JsScript(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyRoot::JsExpressionSnipped(it) => std::fmt::Debug::fmt(it, f),
            JsAnyRoot::JsModule(it) => std::fmt::Debug::fmt(it, f),
            JsAnyRoot::JsScript(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyRoot> for SyntaxNode {
    fn from(n: JsAnyRoot) -> SyntaxNode {
        match n {
            JsAnyRoot::JsExpressionSnipped(it) => it.into(),
            JsAnyRoot::JsModule(it) => it.into(),
            JsAnyRoot::JsScript(it) => it.into(),
        }
    }
}
impl From<JsAnyRoot> for SyntaxElement {
    fn from(n: JsAnyRoot) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsBlockStatement> for JsAnyStatement {
    fn from(node: JsBlockStatement) -> JsAnyStatement { JsAnyStatement::JsBlockStatement(node) }
}
impl From<JsBreakStatement> for JsAnyStatement {
    fn from(node: JsBreakStatement) -> JsAnyStatement { JsAnyStatement::JsBreakStatement(node) }
}
impl From<JsClassDeclaration> for JsAnyStatement {
    fn from(node: JsClassDeclaration) -> JsAnyStatement { JsAnyStatement::JsClassDeclaration(node) }
}
impl From<JsContinueStatement> for JsAnyStatement {
    fn from(node: JsContinueStatement) -> JsAnyStatement {
        JsAnyStatement::JsContinueStatement(node)
    }
}
impl From<JsDebuggerStatement> for JsAnyStatement {
    fn from(node: JsDebuggerStatement) -> JsAnyStatement {
        JsAnyStatement::JsDebuggerStatement(node)
    }
}
impl From<JsDoWhileStatement> for JsAnyStatement {
    fn from(node: JsDoWhileStatement) -> JsAnyStatement { JsAnyStatement::JsDoWhileStatement(node) }
}
impl From<JsEmptyStatement> for JsAnyStatement {
    fn from(node: JsEmptyStatement) -> JsAnyStatement { JsAnyStatement::JsEmptyStatement(node) }
}
impl From<JsExpressionStatement> for JsAnyStatement {
    fn from(node: JsExpressionStatement) -> JsAnyStatement {
        JsAnyStatement::JsExpressionStatement(node)
    }
}
impl From<JsForInStatement> for JsAnyStatement {
    fn from(node: JsForInStatement) -> JsAnyStatement { JsAnyStatement::JsForInStatement(node) }
}
impl From<JsForOfStatement> for JsAnyStatement {
    fn from(node: JsForOfStatement) -> JsAnyStatement { JsAnyStatement::JsForOfStatement(node) }
}
impl From<JsForStatement> for JsAnyStatement {
    fn from(node: JsForStatement) -> JsAnyStatement { JsAnyStatement::JsForStatement(node) }
}
impl From<JsFunctionDeclaration> for JsAnyStatement {
    fn from(node: JsFunctionDeclaration) -> JsAnyStatement {
        JsAnyStatement::JsFunctionDeclaration(node)
    }
}
impl From<JsIfStatement> for JsAnyStatement {
    fn from(node: JsIfStatement) -> JsAnyStatement { JsAnyStatement::JsIfStatement(node) }
}
impl From<JsLabeledStatement> for JsAnyStatement {
    fn from(node: JsLabeledStatement) -> JsAnyStatement { JsAnyStatement::JsLabeledStatement(node) }
}
impl From<JsReturnStatement> for JsAnyStatement {
    fn from(node: JsReturnStatement) -> JsAnyStatement { JsAnyStatement::JsReturnStatement(node) }
}
impl From<JsSwitchStatement> for JsAnyStatement {
    fn from(node: JsSwitchStatement) -> JsAnyStatement { JsAnyStatement::JsSwitchStatement(node) }
}
impl From<JsThrowStatement> for JsAnyStatement {
    fn from(node: JsThrowStatement) -> JsAnyStatement { JsAnyStatement::JsThrowStatement(node) }
}
impl From<JsTryFinallyStatement> for JsAnyStatement {
    fn from(node: JsTryFinallyStatement) -> JsAnyStatement {
        JsAnyStatement::JsTryFinallyStatement(node)
    }
}
impl From<JsTryStatement> for JsAnyStatement {
    fn from(node: JsTryStatement) -> JsAnyStatement { JsAnyStatement::JsTryStatement(node) }
}
impl From<JsUnknownStatement> for JsAnyStatement {
    fn from(node: JsUnknownStatement) -> JsAnyStatement { JsAnyStatement::JsUnknownStatement(node) }
}
impl From<JsVariableStatement> for JsAnyStatement {
    fn from(node: JsVariableStatement) -> JsAnyStatement {
        JsAnyStatement::JsVariableStatement(node)
    }
}
impl From<JsWhileStatement> for JsAnyStatement {
    fn from(node: JsWhileStatement) -> JsAnyStatement { JsAnyStatement::JsWhileStatement(node) }
}
impl From<JsWithStatement> for JsAnyStatement {
    fn from(node: JsWithStatement) -> JsAnyStatement { JsAnyStatement::JsWithStatement(node) }
}
impl From<TsDeclareFunctionDeclaration> for JsAnyStatement {
    fn from(node: TsDeclareFunctionDeclaration) -> JsAnyStatement {
        JsAnyStatement::TsDeclareFunctionDeclaration(node)
    }
}
impl From<TsDeclareStatement> for JsAnyStatement {
    fn from(node: TsDeclareStatement) -> JsAnyStatement { JsAnyStatement::TsDeclareStatement(node) }
}
impl From<TsEnumDeclaration> for JsAnyStatement {
    fn from(node: TsEnumDeclaration) -> JsAnyStatement { JsAnyStatement::TsEnumDeclaration(node) }
}
impl From<TsExternalModuleDeclaration> for JsAnyStatement {
    fn from(node: TsExternalModuleDeclaration) -> JsAnyStatement {
        JsAnyStatement::TsExternalModuleDeclaration(node)
    }
}
impl From<TsGlobalDeclaration> for JsAnyStatement {
    fn from(node: TsGlobalDeclaration) -> JsAnyStatement {
        JsAnyStatement::TsGlobalDeclaration(node)
    }
}
impl From<TsInterfaceDeclaration> for JsAnyStatement {
    fn from(node: TsInterfaceDeclaration) -> JsAnyStatement {
        JsAnyStatement::TsInterfaceDeclaration(node)
    }
}
impl From<TsModuleDeclaration> for JsAnyStatement {
    fn from(node: TsModuleDeclaration) -> JsAnyStatement {
        JsAnyStatement::TsModuleDeclaration(node)
    }
}
impl From<TsTypeAliasDeclaration> for JsAnyStatement {
    fn from(node: TsTypeAliasDeclaration) -> JsAnyStatement {
        JsAnyStatement::TsTypeAliasDeclaration(node)
    }
}
impl AstNode for JsAnyStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            JS_BLOCK_STATEMENT
                | JS_BREAK_STATEMENT
                | JS_CLASS_DECLARATION
                | JS_CONTINUE_STATEMENT
                | JS_DEBUGGER_STATEMENT
                | JS_DO_WHILE_STATEMENT
                | JS_EMPTY_STATEMENT
                | JS_EXPRESSION_STATEMENT
                | JS_FOR_IN_STATEMENT
                | JS_FOR_OF_STATEMENT
                | JS_FOR_STATEMENT
                | JS_FUNCTION_DECLARATION
                | JS_IF_STATEMENT
                | JS_LABELED_STATEMENT
                | JS_RETURN_STATEMENT
                | JS_SWITCH_STATEMENT
                | JS_THROW_STATEMENT
                | JS_TRY_FINALLY_STATEMENT
                | JS_TRY_STATEMENT
                | JS_UNKNOWN_STATEMENT
                | JS_VARIABLE_STATEMENT
                | JS_WHILE_STATEMENT
                | JS_WITH_STATEMENT
                | TS_DECLARE_FUNCTION_DECLARATION
                | TS_DECLARE_STATEMENT
                | TS_ENUM_DECLARATION
                | TS_EXTERNAL_MODULE_DECLARATION
                | TS_GLOBAL_DECLARATION
                | TS_INTERFACE_DECLARATION
                | TS_MODULE_DECLARATION
                | TS_TYPE_ALIAS_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_BLOCK_STATEMENT => JsAnyStatement::JsBlockStatement(JsBlockStatement { syntax }),
            JS_BREAK_STATEMENT => JsAnyStatement::JsBreakStatement(JsBreakStatement { syntax }),
            JS_CLASS_DECLARATION => {
                JsAnyStatement::JsClassDeclaration(JsClassDeclaration { syntax })
            }
            JS_CONTINUE_STATEMENT => {
                JsAnyStatement::JsContinueStatement(JsContinueStatement { syntax })
            }
            JS_DEBUGGER_STATEMENT => {
                JsAnyStatement::JsDebuggerStatement(JsDebuggerStatement { syntax })
            }
            JS_DO_WHILE_STATEMENT => {
                JsAnyStatement::JsDoWhileStatement(JsDoWhileStatement { syntax })
            }
            JS_EMPTY_STATEMENT => JsAnyStatement::JsEmptyStatement(JsEmptyStatement { syntax }),
            JS_EXPRESSION_STATEMENT => {
                JsAnyStatement::JsExpressionStatement(JsExpressionStatement { syntax })
            }
            JS_FOR_IN_STATEMENT => JsAnyStatement::JsForInStatement(JsForInStatement { syntax }),
            JS_FOR_OF_STATEMENT => JsAnyStatement::JsForOfStatement(JsForOfStatement { syntax }),
            JS_FOR_STATEMENT => JsAnyStatement::JsForStatement(JsForStatement { syntax }),
            JS_FUNCTION_DECLARATION => {
                JsAnyStatement::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
            }
            JS_IF_STATEMENT => JsAnyStatement::JsIfStatement(JsIfStatement { syntax }),
            JS_LABELED_STATEMENT => {
                JsAnyStatement::JsLabeledStatement(JsLabeledStatement { syntax })
            }
            JS_RETURN_STATEMENT => JsAnyStatement::JsReturnStatement(JsReturnStatement { syntax }),
            JS_SWITCH_STATEMENT => JsAnyStatement::JsSwitchStatement(JsSwitchStatement { syntax }),
            JS_THROW_STATEMENT => JsAnyStatement::JsThrowStatement(JsThrowStatement { syntax }),
            JS_TRY_FINALLY_STATEMENT => {
                JsAnyStatement::JsTryFinallyStatement(JsTryFinallyStatement { syntax })
            }
            JS_TRY_STATEMENT => JsAnyStatement::JsTryStatement(JsTryStatement { syntax }),
            JS_UNKNOWN_STATEMENT => {
                JsAnyStatement::JsUnknownStatement(JsUnknownStatement { syntax })
            }
            JS_VARIABLE_STATEMENT => {
                JsAnyStatement::JsVariableStatement(JsVariableStatement { syntax })
            }
            JS_WHILE_STATEMENT => JsAnyStatement::JsWhileStatement(JsWhileStatement { syntax }),
            JS_WITH_STATEMENT => JsAnyStatement::JsWithStatement(JsWithStatement { syntax }),
            TS_DECLARE_FUNCTION_DECLARATION => {
                JsAnyStatement::TsDeclareFunctionDeclaration(TsDeclareFunctionDeclaration {
                    syntax,
                })
            }
            TS_DECLARE_STATEMENT => {
                JsAnyStatement::TsDeclareStatement(TsDeclareStatement { syntax })
            }
            TS_ENUM_DECLARATION => JsAnyStatement::TsEnumDeclaration(TsEnumDeclaration { syntax }),
            TS_EXTERNAL_MODULE_DECLARATION => {
                JsAnyStatement::TsExternalModuleDeclaration(TsExternalModuleDeclaration { syntax })
            }
            TS_GLOBAL_DECLARATION => {
                JsAnyStatement::TsGlobalDeclaration(TsGlobalDeclaration { syntax })
            }
            TS_INTERFACE_DECLARATION => {
                JsAnyStatement::TsInterfaceDeclaration(TsInterfaceDeclaration { syntax })
            }
            TS_MODULE_DECLARATION => {
                JsAnyStatement::TsModuleDeclaration(TsModuleDeclaration { syntax })
            }
            TS_TYPE_ALIAS_DECLARATION => {
                JsAnyStatement::TsTypeAliasDeclaration(TsTypeAliasDeclaration { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyStatement::JsBlockStatement(it) => &it.syntax,
            JsAnyStatement::JsBreakStatement(it) => &it.syntax,
            JsAnyStatement::JsClassDeclaration(it) => &it.syntax,
            JsAnyStatement::JsContinueStatement(it) => &it.syntax,
            JsAnyStatement::JsDebuggerStatement(it) => &it.syntax,
            JsAnyStatement::JsDoWhileStatement(it) => &it.syntax,
            JsAnyStatement::JsEmptyStatement(it) => &it.syntax,
            JsAnyStatement::JsExpressionStatement(it) => &it.syntax,
            JsAnyStatement::JsForInStatement(it) => &it.syntax,
            JsAnyStatement::JsForOfStatement(it) => &it.syntax,
            JsAnyStatement::JsForStatement(it) => &it.syntax,
            JsAnyStatement::JsFunctionDeclaration(it) => &it.syntax,
            JsAnyStatement::JsIfStatement(it) => &it.syntax,
            JsAnyStatement::JsLabeledStatement(it) => &it.syntax,
            JsAnyStatement::JsReturnStatement(it) => &it.syntax,
            JsAnyStatement::JsSwitchStatement(it) => &it.syntax,
            JsAnyStatement::JsThrowStatement(it) => &it.syntax,
            JsAnyStatement::JsTryFinallyStatement(it) => &it.syntax,
            JsAnyStatement::JsTryStatement(it) => &it.syntax,
            JsAnyStatement::JsUnknownStatement(it) => &it.syntax,
            JsAnyStatement::JsVariableStatement(it) => &it.syntax,
            JsAnyStatement::JsWhileStatement(it) => &it.syntax,
            JsAnyStatement::JsWithStatement(it) => &it.syntax,
            JsAnyStatement::TsDeclareFunctionDeclaration(it) => &it.syntax,
            JsAnyStatement::TsDeclareStatement(it) => &it.syntax,
            JsAnyStatement::TsEnumDeclaration(it) => &it.syntax,
            JsAnyStatement::TsExternalModuleDeclaration(it) => &it.syntax,
            JsAnyStatement::TsGlobalDeclaration(it) => &it.syntax,
            JsAnyStatement::TsInterfaceDeclaration(it) => &it.syntax,
            JsAnyStatement::TsModuleDeclaration(it) => &it.syntax,
            JsAnyStatement::TsTypeAliasDeclaration(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyStatement::JsBlockStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsBreakStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsContinueStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsDebuggerStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsDoWhileStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsEmptyStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsExpressionStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsForInStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsForOfStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsForStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsIfStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsLabeledStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsReturnStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsSwitchStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsThrowStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsTryFinallyStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsTryStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsUnknownStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsVariableStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsWhileStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::JsWithStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::TsDeclareFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::TsDeclareStatement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::TsEnumDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::TsExternalModuleDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::TsGlobalDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::TsInterfaceDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::TsModuleDeclaration(it) => std::fmt::Debug::fmt(it, f),
            JsAnyStatement::TsTypeAliasDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyStatement> for SyntaxNode {
    fn from(n: JsAnyStatement) -> SyntaxNode {
        match n {
            JsAnyStatement::JsBlockStatement(it) => it.into(),
            JsAnyStatement::JsBreakStatement(it) => it.into(),
            JsAnyStatement::JsClassDeclaration(it) => it.into(),
            JsAnyStatement::JsContinueStatement(it) => it.into(),
            JsAnyStatement::JsDebuggerStatement(it) => it.into(),
            JsAnyStatement::JsDoWhileStatement(it) => it.into(),
            JsAnyStatement::JsEmptyStatement(it) => it.into(),
            JsAnyStatement::JsExpressionStatement(it) => it.into(),
            JsAnyStatement::JsForInStatement(it) => it.into(),
            JsAnyStatement::JsForOfStatement(it) => it.into(),
            JsAnyStatement::JsForStatement(it) => it.into(),
            JsAnyStatement::JsFunctionDeclaration(it) => it.into(),
            JsAnyStatement::JsIfStatement(it) => it.into(),
            JsAnyStatement::JsLabeledStatement(it) => it.into(),
            JsAnyStatement::JsReturnStatement(it) => it.into(),
            JsAnyStatement::JsSwitchStatement(it) => it.into(),
            JsAnyStatement::JsThrowStatement(it) => it.into(),
            JsAnyStatement::JsTryFinallyStatement(it) => it.into(),
            JsAnyStatement::JsTryStatement(it) => it.into(),
            JsAnyStatement::JsUnknownStatement(it) => it.into(),
            JsAnyStatement::JsVariableStatement(it) => it.into(),
            JsAnyStatement::JsWhileStatement(it) => it.into(),
            JsAnyStatement::JsWithStatement(it) => it.into(),
            JsAnyStatement::TsDeclareFunctionDeclaration(it) => it.into(),
            JsAnyStatement::TsDeclareStatement(it) => it.into(),
            JsAnyStatement::TsEnumDeclaration(it) => it.into(),
            JsAnyStatement::TsExternalModuleDeclaration(it) => it.into(),
            JsAnyStatement::TsGlobalDeclaration(it) => it.into(),
            JsAnyStatement::TsInterfaceDeclaration(it) => it.into(),
            JsAnyStatement::TsModuleDeclaration(it) => it.into(),
            JsAnyStatement::TsTypeAliasDeclaration(it) => it.into(),
        }
    }
}
impl From<JsAnyStatement> for SyntaxElement {
    fn from(n: JsAnyStatement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsCaseClause> for JsAnySwitchClause {
    fn from(node: JsCaseClause) -> JsAnySwitchClause { JsAnySwitchClause::JsCaseClause(node) }
}
impl From<JsDefaultClause> for JsAnySwitchClause {
    fn from(node: JsDefaultClause) -> JsAnySwitchClause { JsAnySwitchClause::JsDefaultClause(node) }
}
impl AstNode for JsAnySwitchClause {
    fn can_cast(kind: JsSyntaxKind) -> bool { matches!(kind, JS_CASE_CLAUSE | JS_DEFAULT_CLAUSE) }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_CASE_CLAUSE => JsAnySwitchClause::JsCaseClause(JsCaseClause { syntax }),
            JS_DEFAULT_CLAUSE => JsAnySwitchClause::JsDefaultClause(JsDefaultClause { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnySwitchClause::JsCaseClause(it) => &it.syntax,
            JsAnySwitchClause::JsDefaultClause(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnySwitchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnySwitchClause::JsCaseClause(it) => std::fmt::Debug::fmt(it, f),
            JsAnySwitchClause::JsDefaultClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnySwitchClause> for SyntaxNode {
    fn from(n: JsAnySwitchClause) -> SyntaxNode {
        match n {
            JsAnySwitchClause::JsCaseClause(it) => it.into(),
            JsAnySwitchClause::JsDefaultClause(it) => it.into(),
        }
    }
}
impl From<JsAnySwitchClause> for SyntaxElement {
    fn from(n: JsAnySwitchClause) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsTemplateChunkElement> for JsAnyTemplateElement {
    fn from(node: JsTemplateChunkElement) -> JsAnyTemplateElement {
        JsAnyTemplateElement::JsTemplateChunkElement(node)
    }
}
impl From<JsTemplateElement> for JsAnyTemplateElement {
    fn from(node: JsTemplateElement) -> JsAnyTemplateElement {
        JsAnyTemplateElement::JsTemplateElement(node)
    }
}
impl AstNode for JsAnyTemplateElement {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, JS_TEMPLATE_CHUNK_ELEMENT | JS_TEMPLATE_ELEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_TEMPLATE_CHUNK_ELEMENT => {
                JsAnyTemplateElement::JsTemplateChunkElement(JsTemplateChunkElement { syntax })
            }
            JS_TEMPLATE_ELEMENT => {
                JsAnyTemplateElement::JsTemplateElement(JsTemplateElement { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            JsAnyTemplateElement::JsTemplateChunkElement(it) => &it.syntax,
            JsAnyTemplateElement::JsTemplateElement(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for JsAnyTemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsAnyTemplateElement::JsTemplateChunkElement(it) => std::fmt::Debug::fmt(it, f),
            JsAnyTemplateElement::JsTemplateElement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<JsAnyTemplateElement> for SyntaxNode {
    fn from(n: JsAnyTemplateElement) -> SyntaxNode {
        match n {
            JsAnyTemplateElement::JsTemplateChunkElement(it) => it.into(),
            JsAnyTemplateElement::JsTemplateElement(it) => it.into(),
        }
    }
}
impl From<JsAnyTemplateElement> for SyntaxElement {
    fn from(n: JsAnyTemplateElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsEmptyExternalModuleDeclarationBody> for TsAnyExternalModuleDeclarationBody {
    fn from(node: TsEmptyExternalModuleDeclarationBody) -> TsAnyExternalModuleDeclarationBody {
        TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(node)
    }
}
impl From<TsModuleBlock> for TsAnyExternalModuleDeclarationBody {
    fn from(node: TsModuleBlock) -> TsAnyExternalModuleDeclarationBody {
        TsAnyExternalModuleDeclarationBody::TsModuleBlock(node)
    }
}
impl AstNode for TsAnyExternalModuleDeclarationBody {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY | TS_MODULE_BLOCK
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY => {
                TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(
                    TsEmptyExternalModuleDeclarationBody { syntax },
                )
            }
            TS_MODULE_BLOCK => {
                TsAnyExternalModuleDeclarationBody::TsModuleBlock(TsModuleBlock { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(it) => {
                &it.syntax
            }
            TsAnyExternalModuleDeclarationBody::TsModuleBlock(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyExternalModuleDeclarationBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            TsAnyExternalModuleDeclarationBody::TsModuleBlock(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyExternalModuleDeclarationBody> for SyntaxNode {
    fn from(n: TsAnyExternalModuleDeclarationBody) -> SyntaxNode {
        match n {
            TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(it) => {
                it.into()
            }
            TsAnyExternalModuleDeclarationBody::TsModuleBlock(it) => it.into(),
        }
    }
}
impl From<TsAnyExternalModuleDeclarationBody> for SyntaxElement {
    fn from(n: TsAnyExternalModuleDeclarationBody) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsIdentifierBinding> for TsAnyModuleName {
    fn from(node: TsIdentifierBinding) -> TsAnyModuleName {
        TsAnyModuleName::TsIdentifierBinding(node)
    }
}
impl From<TsQualifiedModuleName> for TsAnyModuleName {
    fn from(node: TsQualifiedModuleName) -> TsAnyModuleName {
        TsAnyModuleName::TsQualifiedModuleName(node)
    }
}
impl AstNode for TsAnyModuleName {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, TS_IDENTIFIER_BINDING | TS_QUALIFIED_MODULE_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_IDENTIFIER_BINDING => {
                TsAnyModuleName::TsIdentifierBinding(TsIdentifierBinding { syntax })
            }
            TS_QUALIFIED_MODULE_NAME => {
                TsAnyModuleName::TsQualifiedModuleName(TsQualifiedModuleName { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyModuleName::TsIdentifierBinding(it) => &it.syntax,
            TsAnyModuleName::TsQualifiedModuleName(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyModuleName::TsIdentifierBinding(it) => std::fmt::Debug::fmt(it, f),
            TsAnyModuleName::TsQualifiedModuleName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyModuleName> for SyntaxNode {
    fn from(n: TsAnyModuleName) -> SyntaxNode {
        match n {
            TsAnyModuleName::TsIdentifierBinding(it) => it.into(),
            TsAnyModuleName::TsQualifiedModuleName(it) => it.into(),
        }
    }
}
impl From<TsAnyModuleName> for SyntaxElement {
    fn from(n: TsAnyModuleName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsReferenceIdentifier> for TsAnyName {
    fn from(node: JsReferenceIdentifier) -> TsAnyName { TsAnyName::JsReferenceIdentifier(node) }
}
impl From<TsQualifiedName> for TsAnyName {
    fn from(node: TsQualifiedName) -> TsAnyName { TsAnyName::TsQualifiedName(node) }
}
impl AstNode for TsAnyName {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, JS_REFERENCE_IDENTIFIER | TS_QUALIFIED_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_REFERENCE_IDENTIFIER => {
                TsAnyName::JsReferenceIdentifier(JsReferenceIdentifier { syntax })
            }
            TS_QUALIFIED_NAME => TsAnyName::TsQualifiedName(TsQualifiedName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyName::JsReferenceIdentifier(it) => &it.syntax,
            TsAnyName::TsQualifiedName(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyName::JsReferenceIdentifier(it) => std::fmt::Debug::fmt(it, f),
            TsAnyName::TsQualifiedName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyName> for SyntaxNode {
    fn from(n: TsAnyName) -> SyntaxNode {
        match n {
            TsAnyName::JsReferenceIdentifier(it) => it.into(),
            TsAnyName::TsQualifiedName(it) => it.into(),
        }
    }
}
impl From<TsAnyName> for SyntaxElement {
    fn from(n: TsAnyName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsDefinitePropertyAnnotation> for TsAnyPropertyAnnotation {
    fn from(node: TsDefinitePropertyAnnotation) -> TsAnyPropertyAnnotation {
        TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(node)
    }
}
impl From<TsOptionalPropertyAnnotation> for TsAnyPropertyAnnotation {
    fn from(node: TsOptionalPropertyAnnotation) -> TsAnyPropertyAnnotation {
        TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(node)
    }
}
impl From<TsTypeAnnotation> for TsAnyPropertyAnnotation {
    fn from(node: TsTypeAnnotation) -> TsAnyPropertyAnnotation {
        TsAnyPropertyAnnotation::TsTypeAnnotation(node)
    }
}
impl AstNode for TsAnyPropertyAnnotation {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            TS_DEFINITE_PROPERTY_ANNOTATION | TS_OPTIONAL_PROPERTY_ANNOTATION | TS_TYPE_ANNOTATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_DEFINITE_PROPERTY_ANNOTATION => {
                TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(
                    TsDefinitePropertyAnnotation { syntax },
                )
            }
            TS_OPTIONAL_PROPERTY_ANNOTATION => {
                TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(
                    TsOptionalPropertyAnnotation { syntax },
                )
            }
            TS_TYPE_ANNOTATION => {
                TsAnyPropertyAnnotation::TsTypeAnnotation(TsTypeAnnotation { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(it) => &it.syntax,
            TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(it) => &it.syntax,
            TsAnyPropertyAnnotation::TsTypeAnnotation(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyPropertyAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            TsAnyPropertyAnnotation::TsTypeAnnotation(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyPropertyAnnotation> for SyntaxNode {
    fn from(n: TsAnyPropertyAnnotation) -> SyntaxNode {
        match n {
            TsAnyPropertyAnnotation::TsDefinitePropertyAnnotation(it) => it.into(),
            TsAnyPropertyAnnotation::TsOptionalPropertyAnnotation(it) => it.into(),
            TsAnyPropertyAnnotation::TsTypeAnnotation(it) => it.into(),
        }
    }
}
impl From<TsAnyPropertyAnnotation> for SyntaxElement {
    fn from(n: TsAnyPropertyAnnotation) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsPropertyParameter> for TsAnyPropertyParameter {
    fn from(node: TsPropertyParameter) -> TsAnyPropertyParameter {
        TsAnyPropertyParameter::TsPropertyParameter(node)
    }
}
impl From<TsReadonlyPropertyParameter> for TsAnyPropertyParameter {
    fn from(node: TsReadonlyPropertyParameter) -> TsAnyPropertyParameter {
        TsAnyPropertyParameter::TsReadonlyPropertyParameter(node)
    }
}
impl AstNode for TsAnyPropertyParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, TS_PROPERTY_PARAMETER | TS_READONLY_PROPERTY_PARAMETER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_PROPERTY_PARAMETER => {
                TsAnyPropertyParameter::TsPropertyParameter(TsPropertyParameter { syntax })
            }
            TS_READONLY_PROPERTY_PARAMETER => {
                TsAnyPropertyParameter::TsReadonlyPropertyParameter(TsReadonlyPropertyParameter {
                    syntax,
                })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyPropertyParameter::TsPropertyParameter(it) => &it.syntax,
            TsAnyPropertyParameter::TsReadonlyPropertyParameter(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyPropertyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyPropertyParameter::TsPropertyParameter(it) => std::fmt::Debug::fmt(it, f),
            TsAnyPropertyParameter::TsReadonlyPropertyParameter(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyPropertyParameter> for SyntaxNode {
    fn from(n: TsAnyPropertyParameter) -> SyntaxNode {
        match n {
            TsAnyPropertyParameter::TsPropertyParameter(it) => it.into(),
            TsAnyPropertyParameter::TsReadonlyPropertyParameter(it) => it.into(),
        }
    }
}
impl From<TsAnyPropertyParameter> for SyntaxElement {
    fn from(n: TsAnyPropertyParameter) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsTypePredicate> for TsAnyReturnType {
    fn from(node: TsTypePredicate) -> TsAnyReturnType { TsAnyReturnType::TsTypePredicate(node) }
}
impl AstNode for TsAnyReturnType {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            TS_TYPE_PREDICATE => true,
            k if TsType::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_TYPE_PREDICATE => TsAnyReturnType::TsTypePredicate(TsTypePredicate { syntax }),
            _ => {
                if let Some(ts_type) = TsType::cast(syntax) {
                    return Some(TsAnyReturnType::TsType(ts_type));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyReturnType::TsTypePredicate(it) => &it.syntax,
            TsAnyReturnType::TsType(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for TsAnyReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyReturnType::TsType(it) => std::fmt::Debug::fmt(it, f),
            TsAnyReturnType::TsTypePredicate(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyReturnType> for SyntaxNode {
    fn from(n: TsAnyReturnType) -> SyntaxNode {
        match n {
            TsAnyReturnType::TsType(it) => it.into(),
            TsAnyReturnType::TsTypePredicate(it) => it.into(),
        }
    }
}
impl From<TsAnyReturnType> for SyntaxElement {
    fn from(n: TsAnyReturnType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsTemplateChunkElement> for TsAnyTemplateElement {
    fn from(node: TsTemplateChunkElement) -> TsAnyTemplateElement {
        TsAnyTemplateElement::TsTemplateChunkElement(node)
    }
}
impl From<TsTemplateElement> for TsAnyTemplateElement {
    fn from(node: TsTemplateElement) -> TsAnyTemplateElement {
        TsAnyTemplateElement::TsTemplateElement(node)
    }
}
impl AstNode for TsAnyTemplateElement {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, TS_TEMPLATE_CHUNK_ELEMENT | TS_TEMPLATE_ELEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_TEMPLATE_CHUNK_ELEMENT => {
                TsAnyTemplateElement::TsTemplateChunkElement(TsTemplateChunkElement { syntax })
            }
            TS_TEMPLATE_ELEMENT => {
                TsAnyTemplateElement::TsTemplateElement(TsTemplateElement { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyTemplateElement::TsTemplateChunkElement(it) => &it.syntax,
            TsAnyTemplateElement::TsTemplateElement(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyTemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyTemplateElement::TsTemplateChunkElement(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTemplateElement::TsTemplateElement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyTemplateElement> for SyntaxNode {
    fn from(n: TsAnyTemplateElement) -> SyntaxNode {
        match n {
            TsAnyTemplateElement::TsTemplateChunkElement(it) => it.into(),
            TsAnyTemplateElement::TsTemplateElement(it) => it.into(),
        }
    }
}
impl From<TsAnyTemplateElement> for SyntaxElement {
    fn from(n: TsAnyTemplateElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsNamedTupleTypeElement> for TsAnyTupleTypeElement {
    fn from(node: TsNamedTupleTypeElement) -> TsAnyTupleTypeElement {
        TsAnyTupleTypeElement::TsNamedTupleTypeElement(node)
    }
}
impl From<TsOptionalTupleTypeElement> for TsAnyTupleTypeElement {
    fn from(node: TsOptionalTupleTypeElement) -> TsAnyTupleTypeElement {
        TsAnyTupleTypeElement::TsOptionalTupleTypeElement(node)
    }
}
impl From<TsRestTupleTypeElement> for TsAnyTupleTypeElement {
    fn from(node: TsRestTupleTypeElement) -> TsAnyTupleTypeElement {
        TsAnyTupleTypeElement::TsRestTupleTypeElement(node)
    }
}
impl AstNode for TsAnyTupleTypeElement {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            TS_NAMED_TUPLE_TYPE_ELEMENT
            | TS_OPTIONAL_TUPLE_TYPE_ELEMENT
            | TS_REST_TUPLE_TYPE_ELEMENT => true,
            k if TsType::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_NAMED_TUPLE_TYPE_ELEMENT => {
                TsAnyTupleTypeElement::TsNamedTupleTypeElement(TsNamedTupleTypeElement { syntax })
            }
            TS_OPTIONAL_TUPLE_TYPE_ELEMENT => {
                TsAnyTupleTypeElement::TsOptionalTupleTypeElement(TsOptionalTupleTypeElement {
                    syntax,
                })
            }
            TS_REST_TUPLE_TYPE_ELEMENT => {
                TsAnyTupleTypeElement::TsRestTupleTypeElement(TsRestTupleTypeElement { syntax })
            }
            _ => {
                if let Some(ts_type) = TsType::cast(syntax) {
                    return Some(TsAnyTupleTypeElement::TsType(ts_type));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyTupleTypeElement::TsNamedTupleTypeElement(it) => &it.syntax,
            TsAnyTupleTypeElement::TsOptionalTupleTypeElement(it) => &it.syntax,
            TsAnyTupleTypeElement::TsRestTupleTypeElement(it) => &it.syntax,
            TsAnyTupleTypeElement::TsType(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for TsAnyTupleTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyTupleTypeElement::TsNamedTupleTypeElement(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTupleTypeElement::TsOptionalTupleTypeElement(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTupleTypeElement::TsRestTupleTypeElement(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTupleTypeElement::TsType(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyTupleTypeElement> for SyntaxNode {
    fn from(n: TsAnyTupleTypeElement) -> SyntaxNode {
        match n {
            TsAnyTupleTypeElement::TsNamedTupleTypeElement(it) => it.into(),
            TsAnyTupleTypeElement::TsOptionalTupleTypeElement(it) => it.into(),
            TsAnyTupleTypeElement::TsRestTupleTypeElement(it) => it.into(),
            TsAnyTupleTypeElement::TsType(it) => it.into(),
        }
    }
}
impl From<TsAnyTupleTypeElement> for SyntaxElement {
    fn from(n: TsAnyTupleTypeElement) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsCallSignatureTypeMember> for TsAnyTypeMember {
    fn from(node: TsCallSignatureTypeMember) -> TsAnyTypeMember {
        TsAnyTypeMember::TsCallSignatureTypeMember(node)
    }
}
impl From<TsConstructSignatureTypeMember> for TsAnyTypeMember {
    fn from(node: TsConstructSignatureTypeMember) -> TsAnyTypeMember {
        TsAnyTypeMember::TsConstructSignatureTypeMember(node)
    }
}
impl From<TsGetterSignatureTypeMember> for TsAnyTypeMember {
    fn from(node: TsGetterSignatureTypeMember) -> TsAnyTypeMember {
        TsAnyTypeMember::TsGetterSignatureTypeMember(node)
    }
}
impl From<TsIndexSignatureTypeMember> for TsAnyTypeMember {
    fn from(node: TsIndexSignatureTypeMember) -> TsAnyTypeMember {
        TsAnyTypeMember::TsIndexSignatureTypeMember(node)
    }
}
impl From<TsMethodSignatureTypeMember> for TsAnyTypeMember {
    fn from(node: TsMethodSignatureTypeMember) -> TsAnyTypeMember {
        TsAnyTypeMember::TsMethodSignatureTypeMember(node)
    }
}
impl From<TsPropertySignatureTypeMember> for TsAnyTypeMember {
    fn from(node: TsPropertySignatureTypeMember) -> TsAnyTypeMember {
        TsAnyTypeMember::TsPropertySignatureTypeMember(node)
    }
}
impl From<TsSetterSignatureTypeMember> for TsAnyTypeMember {
    fn from(node: TsSetterSignatureTypeMember) -> TsAnyTypeMember {
        TsAnyTypeMember::TsSetterSignatureTypeMember(node)
    }
}
impl AstNode for TsAnyTypeMember {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            TS_CALL_SIGNATURE_TYPE_MEMBER
                | TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER
                | TS_GETTER_SIGNATURE_TYPE_MEMBER
                | TS_INDEX_SIGNATURE_TYPE_MEMBER
                | TS_METHOD_SIGNATURE_TYPE_MEMBER
                | TS_PROPERTY_SIGNATURE_TYPE_MEMBER
                | TS_SETTER_SIGNATURE_TYPE_MEMBER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_CALL_SIGNATURE_TYPE_MEMBER => {
                TsAnyTypeMember::TsCallSignatureTypeMember(TsCallSignatureTypeMember { syntax })
            }
            TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER => {
                TsAnyTypeMember::TsConstructSignatureTypeMember(TsConstructSignatureTypeMember {
                    syntax,
                })
            }
            TS_GETTER_SIGNATURE_TYPE_MEMBER => {
                TsAnyTypeMember::TsGetterSignatureTypeMember(TsGetterSignatureTypeMember { syntax })
            }
            TS_INDEX_SIGNATURE_TYPE_MEMBER => {
                TsAnyTypeMember::TsIndexSignatureTypeMember(TsIndexSignatureTypeMember { syntax })
            }
            TS_METHOD_SIGNATURE_TYPE_MEMBER => {
                TsAnyTypeMember::TsMethodSignatureTypeMember(TsMethodSignatureTypeMember { syntax })
            }
            TS_PROPERTY_SIGNATURE_TYPE_MEMBER => {
                TsAnyTypeMember::TsPropertySignatureTypeMember(TsPropertySignatureTypeMember {
                    syntax,
                })
            }
            TS_SETTER_SIGNATURE_TYPE_MEMBER => {
                TsAnyTypeMember::TsSetterSignatureTypeMember(TsSetterSignatureTypeMember { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyTypeMember::TsCallSignatureTypeMember(it) => &it.syntax,
            TsAnyTypeMember::TsConstructSignatureTypeMember(it) => &it.syntax,
            TsAnyTypeMember::TsGetterSignatureTypeMember(it) => &it.syntax,
            TsAnyTypeMember::TsIndexSignatureTypeMember(it) => &it.syntax,
            TsAnyTypeMember::TsMethodSignatureTypeMember(it) => &it.syntax,
            TsAnyTypeMember::TsPropertySignatureTypeMember(it) => &it.syntax,
            TsAnyTypeMember::TsSetterSignatureTypeMember(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyTypeMember::TsCallSignatureTypeMember(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTypeMember::TsConstructSignatureTypeMember(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTypeMember::TsGetterSignatureTypeMember(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTypeMember::TsIndexSignatureTypeMember(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTypeMember::TsMethodSignatureTypeMember(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTypeMember::TsPropertySignatureTypeMember(it) => std::fmt::Debug::fmt(it, f),
            TsAnyTypeMember::TsSetterSignatureTypeMember(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyTypeMember> for SyntaxNode {
    fn from(n: TsAnyTypeMember) -> SyntaxNode {
        match n {
            TsAnyTypeMember::TsCallSignatureTypeMember(it) => it.into(),
            TsAnyTypeMember::TsConstructSignatureTypeMember(it) => it.into(),
            TsAnyTypeMember::TsGetterSignatureTypeMember(it) => it.into(),
            TsAnyTypeMember::TsIndexSignatureTypeMember(it) => it.into(),
            TsAnyTypeMember::TsMethodSignatureTypeMember(it) => it.into(),
            TsAnyTypeMember::TsPropertySignatureTypeMember(it) => it.into(),
            TsAnyTypeMember::TsSetterSignatureTypeMember(it) => it.into(),
        }
    }
}
impl From<TsAnyTypeMember> for SyntaxElement {
    fn from(n: TsAnyTypeMember) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<JsReferenceIdentifier> for TsAnyTypePredicateParameterName {
    fn from(node: JsReferenceIdentifier) -> TsAnyTypePredicateParameterName {
        TsAnyTypePredicateParameterName::JsReferenceIdentifier(node)
    }
}
impl From<TsThisType> for TsAnyTypePredicateParameterName {
    fn from(node: TsThisType) -> TsAnyTypePredicateParameterName {
        TsAnyTypePredicateParameterName::TsThisType(node)
    }
}
impl AstNode for TsAnyTypePredicateParameterName {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, JS_REFERENCE_IDENTIFIER | TS_THIS_TYPE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            JS_REFERENCE_IDENTIFIER => {
                TsAnyTypePredicateParameterName::JsReferenceIdentifier(JsReferenceIdentifier {
                    syntax,
                })
            }
            TS_THIS_TYPE => TsAnyTypePredicateParameterName::TsThisType(TsThisType { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyTypePredicateParameterName::JsReferenceIdentifier(it) => &it.syntax,
            TsAnyTypePredicateParameterName::TsThisType(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyTypePredicateParameterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyTypePredicateParameterName::JsReferenceIdentifier(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            TsAnyTypePredicateParameterName::TsThisType(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyTypePredicateParameterName> for SyntaxNode {
    fn from(n: TsAnyTypePredicateParameterName) -> SyntaxNode {
        match n {
            TsAnyTypePredicateParameterName::JsReferenceIdentifier(it) => it.into(),
            TsAnyTypePredicateParameterName::TsThisType(it) => it.into(),
        }
    }
}
impl From<TsAnyTypePredicateParameterName> for SyntaxElement {
    fn from(n: TsAnyTypePredicateParameterName) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsDefiniteVariableAnnotation> for TsAnyVariableAnnotation {
    fn from(node: TsDefiniteVariableAnnotation) -> TsAnyVariableAnnotation {
        TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(node)
    }
}
impl From<TsTypeAnnotation> for TsAnyVariableAnnotation {
    fn from(node: TsTypeAnnotation) -> TsAnyVariableAnnotation {
        TsAnyVariableAnnotation::TsTypeAnnotation(node)
    }
}
impl AstNode for TsAnyVariableAnnotation {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(kind, TS_DEFINITE_VARIABLE_ANNOTATION | TS_TYPE_ANNOTATION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_DEFINITE_VARIABLE_ANNOTATION => {
                TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(
                    TsDefiniteVariableAnnotation { syntax },
                )
            }
            TS_TYPE_ANNOTATION => {
                TsAnyVariableAnnotation::TsTypeAnnotation(TsTypeAnnotation { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(it) => &it.syntax,
            TsAnyVariableAnnotation::TsTypeAnnotation(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsAnyVariableAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(it) => {
                std::fmt::Debug::fmt(it, f)
            }
            TsAnyVariableAnnotation::TsTypeAnnotation(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsAnyVariableAnnotation> for SyntaxNode {
    fn from(n: TsAnyVariableAnnotation) -> SyntaxNode {
        match n {
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(it) => it.into(),
            TsAnyVariableAnnotation::TsTypeAnnotation(it) => it.into(),
        }
    }
}
impl From<TsAnyVariableAnnotation> for SyntaxElement {
    fn from(n: TsAnyVariableAnnotation) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsExternalModuleRef> for TsModuleRef {
    fn from(node: TsExternalModuleRef) -> TsModuleRef { TsModuleRef::TsExternalModuleRef(node) }
}
impl AstNode for TsModuleRef {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        match kind {
            TS_EXTERNAL_MODULE_REF => true,
            k if TsAnyName::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_EXTERNAL_MODULE_REF => {
                TsModuleRef::TsExternalModuleRef(TsExternalModuleRef { syntax })
            }
            _ => {
                if let Some(ts_any_name) = TsAnyName::cast(syntax) {
                    return Some(TsModuleRef::TsAnyName(ts_any_name));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsModuleRef::TsExternalModuleRef(it) => &it.syntax,
            TsModuleRef::TsAnyName(it) => it.syntax(),
        }
    }
}
impl std::fmt::Debug for TsModuleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsModuleRef::TsAnyName(it) => std::fmt::Debug::fmt(it, f),
            TsModuleRef::TsExternalModuleRef(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsModuleRef> for SyntaxNode {
    fn from(n: TsModuleRef) -> SyntaxNode {
        match n {
            TsModuleRef::TsAnyName(it) => it.into(),
            TsModuleRef::TsExternalModuleRef(it) => it.into(),
        }
    }
}
impl From<TsModuleRef> for SyntaxElement {
    fn from(n: TsModuleRef) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<TsAnyType> for TsType {
    fn from(node: TsAnyType) -> TsType { TsType::TsAnyType(node) }
}
impl From<TsArrayType> for TsType {
    fn from(node: TsArrayType) -> TsType { TsType::TsArrayType(node) }
}
impl From<TsBigIntLiteralType> for TsType {
    fn from(node: TsBigIntLiteralType) -> TsType { TsType::TsBigIntLiteralType(node) }
}
impl From<TsBigintType> for TsType {
    fn from(node: TsBigintType) -> TsType { TsType::TsBigintType(node) }
}
impl From<TsBooleanLiteralType> for TsType {
    fn from(node: TsBooleanLiteralType) -> TsType { TsType::TsBooleanLiteralType(node) }
}
impl From<TsBooleanType> for TsType {
    fn from(node: TsBooleanType) -> TsType { TsType::TsBooleanType(node) }
}
impl From<TsConditionalType> for TsType {
    fn from(node: TsConditionalType) -> TsType { TsType::TsConditionalType(node) }
}
impl From<TsConstructorType> for TsType {
    fn from(node: TsConstructorType) -> TsType { TsType::TsConstructorType(node) }
}
impl From<TsFunctionType> for TsType {
    fn from(node: TsFunctionType) -> TsType { TsType::TsFunctionType(node) }
}
impl From<TsImportType> for TsType {
    fn from(node: TsImportType) -> TsType { TsType::TsImportType(node) }
}
impl From<TsIndexedAccessType> for TsType {
    fn from(node: TsIndexedAccessType) -> TsType { TsType::TsIndexedAccessType(node) }
}
impl From<TsInferType> for TsType {
    fn from(node: TsInferType) -> TsType { TsType::TsInferType(node) }
}
impl From<TsIntersectionType> for TsType {
    fn from(node: TsIntersectionType) -> TsType { TsType::TsIntersectionType(node) }
}
impl From<TsMappedType> for TsType {
    fn from(node: TsMappedType) -> TsType { TsType::TsMappedType(node) }
}
impl From<TsNeverType> for TsType {
    fn from(node: TsNeverType) -> TsType { TsType::TsNeverType(node) }
}
impl From<TsNonPrimitiveType> for TsType {
    fn from(node: TsNonPrimitiveType) -> TsType { TsType::TsNonPrimitiveType(node) }
}
impl From<TsNullLiteralType> for TsType {
    fn from(node: TsNullLiteralType) -> TsType { TsType::TsNullLiteralType(node) }
}
impl From<TsNumberLiteralType> for TsType {
    fn from(node: TsNumberLiteralType) -> TsType { TsType::TsNumberLiteralType(node) }
}
impl From<TsNumberType> for TsType {
    fn from(node: TsNumberType) -> TsType { TsType::TsNumberType(node) }
}
impl From<TsObjectType> for TsType {
    fn from(node: TsObjectType) -> TsType { TsType::TsObjectType(node) }
}
impl From<TsParenthesizedType> for TsType {
    fn from(node: TsParenthesizedType) -> TsType { TsType::TsParenthesizedType(node) }
}
impl From<TsReferenceType> for TsType {
    fn from(node: TsReferenceType) -> TsType { TsType::TsReferenceType(node) }
}
impl From<TsStringLiteralType> for TsType {
    fn from(node: TsStringLiteralType) -> TsType { TsType::TsStringLiteralType(node) }
}
impl From<TsStringType> for TsType {
    fn from(node: TsStringType) -> TsType { TsType::TsStringType(node) }
}
impl From<TsSymbolType> for TsType {
    fn from(node: TsSymbolType) -> TsType { TsType::TsSymbolType(node) }
}
impl From<TsTemplateLiteralType> for TsType {
    fn from(node: TsTemplateLiteralType) -> TsType { TsType::TsTemplateLiteralType(node) }
}
impl From<TsThisType> for TsType {
    fn from(node: TsThisType) -> TsType { TsType::TsThisType(node) }
}
impl From<TsTupleType> for TsType {
    fn from(node: TsTupleType) -> TsType { TsType::TsTupleType(node) }
}
impl From<TsTypeOperatorType> for TsType {
    fn from(node: TsTypeOperatorType) -> TsType { TsType::TsTypeOperatorType(node) }
}
impl From<TsTypeofType> for TsType {
    fn from(node: TsTypeofType) -> TsType { TsType::TsTypeofType(node) }
}
impl From<TsUndefinedType> for TsType {
    fn from(node: TsUndefinedType) -> TsType { TsType::TsUndefinedType(node) }
}
impl From<TsUnionType> for TsType {
    fn from(node: TsUnionType) -> TsType { TsType::TsUnionType(node) }
}
impl From<TsUnknownType> for TsType {
    fn from(node: TsUnknownType) -> TsType { TsType::TsUnknownType(node) }
}
impl From<TsVoidType> for TsType {
    fn from(node: TsVoidType) -> TsType { TsType::TsVoidType(node) }
}
impl AstNode for TsType {
    fn can_cast(kind: JsSyntaxKind) -> bool {
        matches!(
            kind,
            TS_ANY_TYPE
                | TS_ARRAY_TYPE
                | TS_BIG_INT_LITERAL_TYPE
                | TS_BIGINT_TYPE
                | TS_BOOLEAN_LITERAL_TYPE
                | TS_BOOLEAN_TYPE
                | TS_CONDITIONAL_TYPE
                | TS_CONSTRUCTOR_TYPE
                | TS_FUNCTION_TYPE
                | TS_IMPORT_TYPE
                | TS_INDEXED_ACCESS_TYPE
                | TS_INFER_TYPE
                | TS_INTERSECTION_TYPE
                | TS_MAPPED_TYPE
                | TS_NEVER_TYPE
                | TS_NON_PRIMITIVE_TYPE
                | TS_NULL_LITERAL_TYPE
                | TS_NUMBER_LITERAL_TYPE
                | TS_NUMBER_TYPE
                | TS_OBJECT_TYPE
                | TS_PARENTHESIZED_TYPE
                | TS_REFERENCE_TYPE
                | TS_STRING_LITERAL_TYPE
                | TS_STRING_TYPE
                | TS_SYMBOL_TYPE
                | TS_TEMPLATE_LITERAL_TYPE
                | TS_THIS_TYPE
                | TS_TUPLE_TYPE
                | TS_TYPE_OPERATOR_TYPE
                | TS_TYPEOF_TYPE
                | TS_UNDEFINED_TYPE
                | TS_UNION_TYPE
                | TS_UNKNOWN_TYPE
                | TS_VOID_TYPE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TS_ANY_TYPE => TsType::TsAnyType(TsAnyType { syntax }),
            TS_ARRAY_TYPE => TsType::TsArrayType(TsArrayType { syntax }),
            TS_BIG_INT_LITERAL_TYPE => TsType::TsBigIntLiteralType(TsBigIntLiteralType { syntax }),
            TS_BIGINT_TYPE => TsType::TsBigintType(TsBigintType { syntax }),
            TS_BOOLEAN_LITERAL_TYPE => {
                TsType::TsBooleanLiteralType(TsBooleanLiteralType { syntax })
            }
            TS_BOOLEAN_TYPE => TsType::TsBooleanType(TsBooleanType { syntax }),
            TS_CONDITIONAL_TYPE => TsType::TsConditionalType(TsConditionalType { syntax }),
            TS_CONSTRUCTOR_TYPE => TsType::TsConstructorType(TsConstructorType { syntax }),
            TS_FUNCTION_TYPE => TsType::TsFunctionType(TsFunctionType { syntax }),
            TS_IMPORT_TYPE => TsType::TsImportType(TsImportType { syntax }),
            TS_INDEXED_ACCESS_TYPE => TsType::TsIndexedAccessType(TsIndexedAccessType { syntax }),
            TS_INFER_TYPE => TsType::TsInferType(TsInferType { syntax }),
            TS_INTERSECTION_TYPE => TsType::TsIntersectionType(TsIntersectionType { syntax }),
            TS_MAPPED_TYPE => TsType::TsMappedType(TsMappedType { syntax }),
            TS_NEVER_TYPE => TsType::TsNeverType(TsNeverType { syntax }),
            TS_NON_PRIMITIVE_TYPE => TsType::TsNonPrimitiveType(TsNonPrimitiveType { syntax }),
            TS_NULL_LITERAL_TYPE => TsType::TsNullLiteralType(TsNullLiteralType { syntax }),
            TS_NUMBER_LITERAL_TYPE => TsType::TsNumberLiteralType(TsNumberLiteralType { syntax }),
            TS_NUMBER_TYPE => TsType::TsNumberType(TsNumberType { syntax }),
            TS_OBJECT_TYPE => TsType::TsObjectType(TsObjectType { syntax }),
            TS_PARENTHESIZED_TYPE => TsType::TsParenthesizedType(TsParenthesizedType { syntax }),
            TS_REFERENCE_TYPE => TsType::TsReferenceType(TsReferenceType { syntax }),
            TS_STRING_LITERAL_TYPE => TsType::TsStringLiteralType(TsStringLiteralType { syntax }),
            TS_STRING_TYPE => TsType::TsStringType(TsStringType { syntax }),
            TS_SYMBOL_TYPE => TsType::TsSymbolType(TsSymbolType { syntax }),
            TS_TEMPLATE_LITERAL_TYPE => {
                TsType::TsTemplateLiteralType(TsTemplateLiteralType { syntax })
            }
            TS_THIS_TYPE => TsType::TsThisType(TsThisType { syntax }),
            TS_TUPLE_TYPE => TsType::TsTupleType(TsTupleType { syntax }),
            TS_TYPE_OPERATOR_TYPE => TsType::TsTypeOperatorType(TsTypeOperatorType { syntax }),
            TS_TYPEOF_TYPE => TsType::TsTypeofType(TsTypeofType { syntax }),
            TS_UNDEFINED_TYPE => TsType::TsUndefinedType(TsUndefinedType { syntax }),
            TS_UNION_TYPE => TsType::TsUnionType(TsUnionType { syntax }),
            TS_UNKNOWN_TYPE => TsType::TsUnknownType(TsUnknownType { syntax }),
            TS_VOID_TYPE => TsType::TsVoidType(TsVoidType { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TsType::TsAnyType(it) => &it.syntax,
            TsType::TsArrayType(it) => &it.syntax,
            TsType::TsBigIntLiteralType(it) => &it.syntax,
            TsType::TsBigintType(it) => &it.syntax,
            TsType::TsBooleanLiteralType(it) => &it.syntax,
            TsType::TsBooleanType(it) => &it.syntax,
            TsType::TsConditionalType(it) => &it.syntax,
            TsType::TsConstructorType(it) => &it.syntax,
            TsType::TsFunctionType(it) => &it.syntax,
            TsType::TsImportType(it) => &it.syntax,
            TsType::TsIndexedAccessType(it) => &it.syntax,
            TsType::TsInferType(it) => &it.syntax,
            TsType::TsIntersectionType(it) => &it.syntax,
            TsType::TsMappedType(it) => &it.syntax,
            TsType::TsNeverType(it) => &it.syntax,
            TsType::TsNonPrimitiveType(it) => &it.syntax,
            TsType::TsNullLiteralType(it) => &it.syntax,
            TsType::TsNumberLiteralType(it) => &it.syntax,
            TsType::TsNumberType(it) => &it.syntax,
            TsType::TsObjectType(it) => &it.syntax,
            TsType::TsParenthesizedType(it) => &it.syntax,
            TsType::TsReferenceType(it) => &it.syntax,
            TsType::TsStringLiteralType(it) => &it.syntax,
            TsType::TsStringType(it) => &it.syntax,
            TsType::TsSymbolType(it) => &it.syntax,
            TsType::TsTemplateLiteralType(it) => &it.syntax,
            TsType::TsThisType(it) => &it.syntax,
            TsType::TsTupleType(it) => &it.syntax,
            TsType::TsTypeOperatorType(it) => &it.syntax,
            TsType::TsTypeofType(it) => &it.syntax,
            TsType::TsUndefinedType(it) => &it.syntax,
            TsType::TsUnionType(it) => &it.syntax,
            TsType::TsUnknownType(it) => &it.syntax,
            TsType::TsVoidType(it) => &it.syntax,
        }
    }
}
impl std::fmt::Debug for TsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TsType::TsAnyType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsArrayType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsBigIntLiteralType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsBigintType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsBooleanLiteralType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsBooleanType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsConditionalType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsConstructorType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsFunctionType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsImportType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsIndexedAccessType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsInferType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsIntersectionType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsMappedType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsNeverType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsNonPrimitiveType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsNullLiteralType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsNumberLiteralType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsNumberType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsObjectType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsParenthesizedType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsReferenceType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsStringLiteralType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsStringType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsSymbolType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsTemplateLiteralType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsThisType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsTupleType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsTypeOperatorType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsTypeofType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsUndefinedType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsUnionType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsUnknownType(it) => std::fmt::Debug::fmt(it, f),
            TsType::TsVoidType(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<TsType> for SyntaxNode {
    fn from(n: TsType) -> SyntaxNode {
        match n {
            TsType::TsAnyType(it) => it.into(),
            TsType::TsArrayType(it) => it.into(),
            TsType::TsBigIntLiteralType(it) => it.into(),
            TsType::TsBigintType(it) => it.into(),
            TsType::TsBooleanLiteralType(it) => it.into(),
            TsType::TsBooleanType(it) => it.into(),
            TsType::TsConditionalType(it) => it.into(),
            TsType::TsConstructorType(it) => it.into(),
            TsType::TsFunctionType(it) => it.into(),
            TsType::TsImportType(it) => it.into(),
            TsType::TsIndexedAccessType(it) => it.into(),
            TsType::TsInferType(it) => it.into(),
            TsType::TsIntersectionType(it) => it.into(),
            TsType::TsMappedType(it) => it.into(),
            TsType::TsNeverType(it) => it.into(),
            TsType::TsNonPrimitiveType(it) => it.into(),
            TsType::TsNullLiteralType(it) => it.into(),
            TsType::TsNumberLiteralType(it) => it.into(),
            TsType::TsNumberType(it) => it.into(),
            TsType::TsObjectType(it) => it.into(),
            TsType::TsParenthesizedType(it) => it.into(),
            TsType::TsReferenceType(it) => it.into(),
            TsType::TsStringLiteralType(it) => it.into(),
            TsType::TsStringType(it) => it.into(),
            TsType::TsSymbolType(it) => it.into(),
            TsType::TsTemplateLiteralType(it) => it.into(),
            TsType::TsThisType(it) => it.into(),
            TsType::TsTupleType(it) => it.into(),
            TsType::TsTypeOperatorType(it) => it.into(),
            TsType::TsTypeofType(it) => it.into(),
            TsType::TsUndefinedType(it) => it.into(),
            TsType::TsUnionType(it) => it.into(),
            TsType::TsUnknownType(it) => it.into(),
            TsType::TsVoidType(it) => it.into(),
        }
    }
}
impl From<TsType> for SyntaxElement {
    fn from(n: TsType) -> SyntaxElement {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for JsAnyArrayAssignmentPatternElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyArrayBindingPatternElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyArrayElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyArrowFunctionParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyAssignmentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyBindingPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyClassMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyConstructorParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyExportClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyExportNamedSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyForInOrOfInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyForInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyFormalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyImportAssertionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyImportClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyInProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyModuleItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyNamedImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyObjectAssignmentPatternMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyObjectBindingPatternMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyObjectMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnySwitchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAnyTemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyExternalModuleDeclarationBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyPropertyAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyPropertyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyReturnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyTemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyTupleTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyTypePredicateParameterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyVariableAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsModuleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ImportMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsArrayAssignmentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsArrayAssignmentPatternRestElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsArrayBindingPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsArrayBindingPatternRestElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsArrayHole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsArrowFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAssignmentExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAssignmentWithDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsAwaitExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsBigIntLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsBindingPatternWithDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsBlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsBreakStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsCallArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsCaseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsCatchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsCatchDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsClassDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsClassExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsComputedMemberAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsComputedMemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsComputedMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsConditionalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsConstructorClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsConstructorParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsContinueStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsDebuggerStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsDefaultClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsDefaultImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsDoWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsEmptyClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsEmptyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportAsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportDefaultClassClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportDefaultExpressionClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportDefaultFunctionClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportNamedClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportNamedFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportNamedFromSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportNamedShorthandSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExportNamedSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExpressionSnipped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsExtendsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsFinallyClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsForInStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsForOfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsForVariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsFormalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsFunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsGetterClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsGetterObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsIdentifierAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsIdentifierBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsIdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsIfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsImport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsImportAssertion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsImportAssertionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsImportBareClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsImportCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsImportDefaultClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsImportNamedClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsImportNamespaceClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsInExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsInitializerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsInstanceofExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsLabeledStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsLiteralExportName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsLiteralMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsLogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsMethodClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsMethodObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsModuleSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsNamedImportSpecifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsNamespaceImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsNewExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectAssignmentPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectAssignmentPatternProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectAssignmentPatternRest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectAssignmentPatternShorthandProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectBindingPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectBindingPatternProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectBindingPatternRest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectBindingPatternShorthandProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsObjectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsParenthesizedAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsPostUpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsPreUpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsPrivateClassMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsPrivateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsPropertyClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsPropertyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsReferenceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsRegexLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsRestParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsSequenceExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsSetterClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsSetterObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsShorthandNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsShorthandPropertyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsSpread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsStaticInitializationBlockClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsStaticMemberAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsStaticMemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsSuperExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsSwitchStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsTemplateChunkElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsTemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsThisExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsThrowStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsTryFinallyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsTryStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsVariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsVariableDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsVariableDeclarator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsVariableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsWithStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsYieldArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for JsYieldExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NewTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAnyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsAsExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsBigIntLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsBigintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsBooleanLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsBooleanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsCallSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsConditionalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsConstructSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsConstructorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsDeclareFunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsDeclareStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsDefaultTypeClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsDefinitePropertyAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsDefiniteVariableAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsEmptyExternalModuleDeclarationBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsEnumDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsEnumMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsExtendsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsExternalModuleDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsExternalModuleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsFunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsGetterSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsGlobalDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsIdentifierBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsImplementsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsImportEqualsDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsImportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsImportTypeQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsIndexSignatureParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsIndexSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsIndexedAccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsInferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsInterfaceDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsIntersectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsMappedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsMappedTypeAsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsMappedTypeOptionalModifierClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsMappedTypeReadonlyModifierClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsMethodSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsModuleBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsModuleDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNameWithTypeArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNamedTupleTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNeverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNonNullAssertionAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNonNullAssertionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNonPrimitiveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNullLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNumberLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsOptionalPropertyAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsOptionalTupleTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsParenthesizedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsPropertyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsPropertySignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsQualifiedModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsQualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsReadonlyPropertyParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsRestTupleTypeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsReturnTypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsSetterSignatureTypeMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsStringLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsStringType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsSymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTemplateChunkElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTemplateElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTemplateLiteralType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsThisParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsThisType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeAliasDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeAnnotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeAssertionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeConstraintClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeOperatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeParameterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypePredicate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsTypeofType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsUndefinedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsUnionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsUnknownType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TsVoidType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknown {
    syntax: SyntaxNode,
}
impl JsUnknown {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknown {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknown")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknown> for SyntaxNode {
    fn from(n: JsUnknown) -> SyntaxNode { n.syntax }
}
impl From<JsUnknown> for SyntaxElement {
    fn from(n: JsUnknown) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownAssignment {
    syntax: SyntaxNode,
}
impl JsUnknownAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownAssignment {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN_ASSIGNMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknownAssignment")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknownAssignment> for SyntaxNode {
    fn from(n: JsUnknownAssignment) -> SyntaxNode { n.syntax }
}
impl From<JsUnknownAssignment> for SyntaxElement {
    fn from(n: JsUnknownAssignment) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownBinding {
    syntax: SyntaxNode,
}
impl JsUnknownBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownBinding {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN_BINDING }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknownBinding")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknownBinding> for SyntaxNode {
    fn from(n: JsUnknownBinding) -> SyntaxNode { n.syntax }
}
impl From<JsUnknownBinding> for SyntaxElement {
    fn from(n: JsUnknownBinding) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownExpression {
    syntax: SyntaxNode,
}
impl JsUnknownExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownExpression {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN_EXPRESSION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknownExpression")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknownExpression> for SyntaxNode {
    fn from(n: JsUnknownExpression) -> SyntaxNode { n.syntax }
}
impl From<JsUnknownExpression> for SyntaxElement {
    fn from(n: JsUnknownExpression) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownImportAssertionEntry {
    syntax: SyntaxNode,
}
impl JsUnknownImportAssertionEntry {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownImportAssertionEntry {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN_IMPORT_ASSERTION_ENTRY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownImportAssertionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknownImportAssertionEntry")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknownImportAssertionEntry> for SyntaxNode {
    fn from(n: JsUnknownImportAssertionEntry) -> SyntaxNode { n.syntax }
}
impl From<JsUnknownImportAssertionEntry> for SyntaxElement {
    fn from(n: JsUnknownImportAssertionEntry) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownMember {
    syntax: SyntaxNode,
}
impl JsUnknownMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownMember {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN_MEMBER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknownMember")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknownMember> for SyntaxNode {
    fn from(n: JsUnknownMember) -> SyntaxNode { n.syntax }
}
impl From<JsUnknownMember> for SyntaxElement {
    fn from(n: JsUnknownMember) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownNamedImportSpecifier {
    syntax: SyntaxNode,
}
impl JsUnknownNamedImportSpecifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownNamedImportSpecifier {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN_NAMED_IMPORT_SPECIFIER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownNamedImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknownNamedImportSpecifier")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknownNamedImportSpecifier> for SyntaxNode {
    fn from(n: JsUnknownNamedImportSpecifier) -> SyntaxNode { n.syntax }
}
impl From<JsUnknownNamedImportSpecifier> for SyntaxElement {
    fn from(n: JsUnknownNamedImportSpecifier) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownParameter {
    syntax: SyntaxNode,
}
impl JsUnknownParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownParameter {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN_PARAMETER }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknownParameter")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknownParameter> for SyntaxNode {
    fn from(n: JsUnknownParameter) -> SyntaxNode { n.syntax }
}
impl From<JsUnknownParameter> for SyntaxElement {
    fn from(n: JsUnknownParameter) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownStatement {
    syntax: SyntaxNode,
}
impl JsUnknownStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self { Self { syntax } }
    pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownStatement {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_UNKNOWN_STATEMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsUnknownStatement")
            .field("items", &support::DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<JsUnknownStatement> for SyntaxNode {
    fn from(n: JsUnknownStatement) -> SyntaxNode { n.syntax }
}
impl From<JsUnknownStatement> for SyntaxElement {
    fn from(n: JsUnknownStatement) -> SyntaxElement { n.syntax.into() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsArrayAssignmentPatternElementList {
    syntax_list: SyntaxList,
}
impl JsArrayAssignmentPatternElementList {
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
impl AstNode for JsArrayAssignmentPatternElementList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsArrayAssignmentPatternElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsArrayAssignmentPatternElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyArrayAssignmentPatternElement> for JsArrayAssignmentPatternElementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsArrayAssignmentPatternElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsArrayAssignmentPatternElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsArrayAssignmentPatternElementList {
    type Item = SyntaxResult<JsAnyArrayAssignmentPatternElement>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyArrayAssignmentPatternElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsArrayAssignmentPatternElementList {
    type Item = SyntaxResult<JsAnyArrayAssignmentPatternElement>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyArrayAssignmentPatternElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsArrayBindingPatternElementList {
    syntax_list: SyntaxList,
}
impl JsArrayBindingPatternElementList {
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
impl AstNode for JsArrayBindingPatternElementList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsArrayBindingPatternElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsArrayBindingPatternElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyArrayBindingPatternElement> for JsArrayBindingPatternElementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsArrayBindingPatternElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsArrayBindingPatternElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsArrayBindingPatternElementList {
    type Item = SyntaxResult<JsAnyArrayBindingPatternElement>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyArrayBindingPatternElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsArrayBindingPatternElementList {
    type Item = SyntaxResult<JsAnyArrayBindingPatternElement>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyArrayBindingPatternElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsArrayElementList {
    syntax_list: SyntaxList,
}
impl JsArrayElementList {
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
impl AstNode for JsArrayElementList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_ARRAY_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsArrayElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsArrayElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyArrayElement> for JsArrayElementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsArrayElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsArrayElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsArrayElementList {
    type Item = SyntaxResult<JsAnyArrayElement>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyArrayElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsArrayElementList {
    type Item = SyntaxResult<JsAnyArrayElement>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyArrayElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsCallArgumentList {
    syntax_list: SyntaxList,
}
impl JsCallArgumentList {
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
impl AstNode for JsCallArgumentList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CALL_ARGUMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsCallArgumentList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsCallArgumentList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyExpression> for JsCallArgumentList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsCallArgumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsCallArgumentList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsCallArgumentList {
    type Item = SyntaxResult<JsAnyExpression>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyExpression>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsCallArgumentList {
    type Item = SyntaxResult<JsAnyExpression>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyExpression>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsClassMemberList {
    syntax_list: SyntaxList,
}
impl JsClassMemberList {
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
impl AstNode for JsClassMemberList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CLASS_MEMBER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsClassMemberList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsClassMemberList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstNodeList<JsAnyClassMember> for JsClassMemberList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsClassMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsClassMemberList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &JsClassMemberList {
    type Item = JsAnyClassMember;
    type IntoIter = AstNodeListIterator<JsAnyClassMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for JsClassMemberList {
    type Item = JsAnyClassMember;
    type IntoIter = AstNodeListIterator<JsAnyClassMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsConstructorParameterList {
    syntax_list: SyntaxList,
}
impl JsConstructorParameterList {
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
impl AstNode for JsConstructorParameterList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_CONSTRUCTOR_PARAMETER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsConstructorParameterList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsConstructorParameterList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyConstructorParameter> for JsConstructorParameterList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsConstructorParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsConstructorParameterList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsConstructorParameterList {
    type Item = SyntaxResult<JsAnyConstructorParameter>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyConstructorParameter>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsConstructorParameterList {
    type Item = SyntaxResult<JsAnyConstructorParameter>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyConstructorParameter>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsDirectiveList {
    syntax_list: SyntaxList,
}
impl JsDirectiveList {
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
impl AstNode for JsDirectiveList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_DIRECTIVE_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsDirectiveList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsDirectiveList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstNodeList<JsDirective> for JsDirectiveList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsDirectiveList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsDirectiveList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &JsDirectiveList {
    type Item = JsDirective;
    type IntoIter = AstNodeListIterator<JsDirective>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for JsDirectiveList {
    type Item = JsDirective;
    type IntoIter = AstNodeListIterator<JsDirective>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsExportNamedFromSpecifierList {
    syntax_list: SyntaxList,
}
impl JsExportNamedFromSpecifierList {
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
impl AstNode for JsExportNamedFromSpecifierList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_NAMED_FROM_SPECIFIER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsExportNamedFromSpecifierList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsExportNamedFromSpecifierList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsExportNamedFromSpecifier> for JsExportNamedFromSpecifierList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsExportNamedFromSpecifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsExportNamedFromSpecifierList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsExportNamedFromSpecifierList {
    type Item = SyntaxResult<JsExportNamedFromSpecifier>;
    type IntoIter = AstSeparatedListNodesIterator<JsExportNamedFromSpecifier>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsExportNamedFromSpecifierList {
    type Item = SyntaxResult<JsExportNamedFromSpecifier>;
    type IntoIter = AstSeparatedListNodesIterator<JsExportNamedFromSpecifier>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsExportNamedSpecifierList {
    syntax_list: SyntaxList,
}
impl JsExportNamedSpecifierList {
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
impl AstNode for JsExportNamedSpecifierList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_EXPORT_NAMED_SPECIFIER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsExportNamedSpecifierList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsExportNamedSpecifierList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyExportNamedSpecifier> for JsExportNamedSpecifierList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsExportNamedSpecifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsExportNamedSpecifierList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsExportNamedSpecifierList {
    type Item = SyntaxResult<JsAnyExportNamedSpecifier>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyExportNamedSpecifier>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsExportNamedSpecifierList {
    type Item = SyntaxResult<JsAnyExportNamedSpecifier>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyExportNamedSpecifier>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsImportAssertionEntryList {
    syntax_list: SyntaxList,
}
impl JsImportAssertionEntryList {
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
impl AstNode for JsImportAssertionEntryList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_IMPORT_ASSERTION_ENTRY_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsImportAssertionEntryList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsImportAssertionEntryList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyImportAssertionEntry> for JsImportAssertionEntryList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsImportAssertionEntryList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsImportAssertionEntryList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsImportAssertionEntryList {
    type Item = SyntaxResult<JsAnyImportAssertionEntry>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyImportAssertionEntry>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsImportAssertionEntryList {
    type Item = SyntaxResult<JsAnyImportAssertionEntry>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyImportAssertionEntry>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsModuleItemList {
    syntax_list: SyntaxList,
}
impl JsModuleItemList {
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
impl AstNode for JsModuleItemList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_MODULE_ITEM_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsModuleItemList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsModuleItemList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstNodeList<JsAnyModuleItem> for JsModuleItemList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsModuleItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsModuleItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &JsModuleItemList {
    type Item = JsAnyModuleItem;
    type IntoIter = AstNodeListIterator<JsAnyModuleItem>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for JsModuleItemList {
    type Item = JsAnyModuleItem;
    type IntoIter = AstNodeListIterator<JsAnyModuleItem>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsNamedImportSpecifierList {
    syntax_list: SyntaxList,
}
impl JsNamedImportSpecifierList {
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
impl AstNode for JsNamedImportSpecifierList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_NAMED_IMPORT_SPECIFIER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsNamedImportSpecifierList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsNamedImportSpecifierList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyNamedImportSpecifier> for JsNamedImportSpecifierList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsNamedImportSpecifierList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsNamedImportSpecifierList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsNamedImportSpecifierList {
    type Item = SyntaxResult<JsAnyNamedImportSpecifier>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyNamedImportSpecifier>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsNamedImportSpecifierList {
    type Item = SyntaxResult<JsAnyNamedImportSpecifier>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyNamedImportSpecifier>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsObjectAssignmentPatternPropertyList {
    syntax_list: SyntaxList,
}
impl JsObjectAssignmentPatternPropertyList {
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
impl AstNode for JsObjectAssignmentPatternPropertyList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsObjectAssignmentPatternPropertyList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsObjectAssignmentPatternPropertyList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyObjectAssignmentPatternMember>
    for JsObjectAssignmentPatternPropertyList
{
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsObjectAssignmentPatternPropertyList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsObjectAssignmentPatternPropertyList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsObjectAssignmentPatternPropertyList {
    type Item = SyntaxResult<JsAnyObjectAssignmentPatternMember>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyObjectAssignmentPatternMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsObjectAssignmentPatternPropertyList {
    type Item = SyntaxResult<JsAnyObjectAssignmentPatternMember>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyObjectAssignmentPatternMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsObjectBindingPatternPropertyList {
    syntax_list: SyntaxList,
}
impl JsObjectBindingPatternPropertyList {
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
impl AstNode for JsObjectBindingPatternPropertyList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsObjectBindingPatternPropertyList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsObjectBindingPatternPropertyList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyObjectBindingPatternMember> for JsObjectBindingPatternPropertyList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsObjectBindingPatternPropertyList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsObjectBindingPatternPropertyList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsObjectBindingPatternPropertyList {
    type Item = SyntaxResult<JsAnyObjectBindingPatternMember>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyObjectBindingPatternMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsObjectBindingPatternPropertyList {
    type Item = SyntaxResult<JsAnyObjectBindingPatternMember>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyObjectBindingPatternMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsObjectMemberList {
    syntax_list: SyntaxList,
}
impl JsObjectMemberList {
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
impl AstNode for JsObjectMemberList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_OBJECT_MEMBER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsObjectMemberList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsObjectMemberList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyObjectMember> for JsObjectMemberList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsObjectMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsObjectMemberList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsObjectMemberList {
    type Item = SyntaxResult<JsAnyObjectMember>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyObjectMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsObjectMemberList {
    type Item = SyntaxResult<JsAnyObjectMember>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyObjectMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsParameterList {
    syntax_list: SyntaxList,
}
impl JsParameterList {
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
impl AstNode for JsParameterList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_PARAMETER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsParameterList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsParameterList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsAnyParameter> for JsParameterList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsParameterList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsParameterList {
    type Item = SyntaxResult<JsAnyParameter>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyParameter>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsParameterList {
    type Item = SyntaxResult<JsAnyParameter>;
    type IntoIter = AstSeparatedListNodesIterator<JsAnyParameter>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsStatementList {
    syntax_list: SyntaxList,
}
impl JsStatementList {
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
impl AstNode for JsStatementList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_STATEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsStatementList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsStatementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstNodeList<JsAnyStatement> for JsStatementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsStatementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsStatementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &JsStatementList {
    type Item = JsAnyStatement;
    type IntoIter = AstNodeListIterator<JsAnyStatement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for JsStatementList {
    type Item = JsAnyStatement;
    type IntoIter = AstNodeListIterator<JsAnyStatement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsSwitchCaseList {
    syntax_list: SyntaxList,
}
impl JsSwitchCaseList {
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
impl AstNode for JsSwitchCaseList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_SWITCH_CASE_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsSwitchCaseList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsSwitchCaseList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstNodeList<JsAnySwitchClause> for JsSwitchCaseList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsSwitchCaseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsSwitchCaseList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &JsSwitchCaseList {
    type Item = JsAnySwitchClause;
    type IntoIter = AstNodeListIterator<JsAnySwitchClause>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for JsSwitchCaseList {
    type Item = JsAnySwitchClause;
    type IntoIter = AstNodeListIterator<JsAnySwitchClause>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsTemplateElementList {
    syntax_list: SyntaxList,
}
impl JsTemplateElementList {
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
impl AstNode for JsTemplateElementList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_TEMPLATE_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsTemplateElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsTemplateElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstNodeList<JsAnyTemplateElement> for JsTemplateElementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsTemplateElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsTemplateElementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &JsTemplateElementList {
    type Item = JsAnyTemplateElement;
    type IntoIter = AstNodeListIterator<JsAnyTemplateElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for JsTemplateElementList {
    type Item = JsAnyTemplateElement;
    type IntoIter = AstNodeListIterator<JsAnyTemplateElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct JsVariableDeclaratorList {
    syntax_list: SyntaxList,
}
impl JsVariableDeclaratorList {
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
impl AstNode for JsVariableDeclaratorList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATOR_LIST }
    fn cast(syntax: SyntaxNode) -> Option<JsVariableDeclaratorList> {
        if Self::can_cast(syntax.kind()) {
            Some(JsVariableDeclaratorList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<JsVariableDeclarator> for JsVariableDeclaratorList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for JsVariableDeclaratorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("JsVariableDeclaratorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for JsVariableDeclaratorList {
    type Item = SyntaxResult<JsVariableDeclarator>;
    type IntoIter = AstSeparatedListNodesIterator<JsVariableDeclarator>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsVariableDeclaratorList {
    type Item = SyntaxResult<JsVariableDeclarator>;
    type IntoIter = AstSeparatedListNodesIterator<JsVariableDeclarator>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsEnumMemberList {
    syntax_list: SyntaxList,
}
impl TsEnumMemberList {
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
impl AstNode for TsEnumMemberList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_ENUM_MEMBER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsEnumMemberList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsEnumMemberList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<TsEnumMember> for TsEnumMemberList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsEnumMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsEnumMemberList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TsEnumMemberList {
    type Item = SyntaxResult<TsEnumMember>;
    type IntoIter = AstSeparatedListNodesIterator<TsEnumMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsEnumMemberList {
    type Item = SyntaxResult<TsEnumMember>;
    type IntoIter = AstSeparatedListNodesIterator<TsEnumMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsIntersectionTypeElementList {
    syntax_list: SyntaxList,
}
impl TsIntersectionTypeElementList {
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
impl AstNode for TsIntersectionTypeElementList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_INTERSECTION_TYPE_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsIntersectionTypeElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsIntersectionTypeElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<TsType> for TsIntersectionTypeElementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsIntersectionTypeElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsIntersectionTypeElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TsIntersectionTypeElementList {
    type Item = SyntaxResult<TsType>;
    type IntoIter = AstSeparatedListNodesIterator<TsType>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsIntersectionTypeElementList {
    type Item = SyntaxResult<TsType>;
    type IntoIter = AstSeparatedListNodesIterator<TsType>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsTemplateElementList {
    syntax_list: SyntaxList,
}
impl TsTemplateElementList {
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
impl AstNode for TsTemplateElementList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TEMPLATE_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsTemplateElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsTemplateElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstNodeList<TsAnyTemplateElement> for TsTemplateElementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsTemplateElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsTemplateElementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &TsTemplateElementList {
    type Item = TsAnyTemplateElement;
    type IntoIter = AstNodeListIterator<TsAnyTemplateElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for TsTemplateElementList {
    type Item = TsAnyTemplateElement;
    type IntoIter = AstNodeListIterator<TsAnyTemplateElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsTupleTypeElementList {
    syntax_list: SyntaxList,
}
impl TsTupleTypeElementList {
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
impl AstNode for TsTupleTypeElementList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TUPLE_TYPE_ELEMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsTupleTypeElementList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsTupleTypeElementList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<TsAnyTupleTypeElement> for TsTupleTypeElementList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsTupleTypeElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsTupleTypeElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TsTupleTypeElementList {
    type Item = SyntaxResult<TsAnyTupleTypeElement>;
    type IntoIter = AstSeparatedListNodesIterator<TsAnyTupleTypeElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsTupleTypeElementList {
    type Item = SyntaxResult<TsAnyTupleTypeElement>;
    type IntoIter = AstSeparatedListNodesIterator<TsAnyTupleTypeElement>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsTypeArgumentList {
    syntax_list: SyntaxList,
}
impl TsTypeArgumentList {
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
impl AstNode for TsTypeArgumentList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_ARGUMENT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsTypeArgumentList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsTypeArgumentList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<TsType> for TsTypeArgumentList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsTypeArgumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsTypeArgumentList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TsTypeArgumentList {
    type Item = SyntaxResult<TsType>;
    type IntoIter = AstSeparatedListNodesIterator<TsType>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsTypeArgumentList {
    type Item = SyntaxResult<TsType>;
    type IntoIter = AstSeparatedListNodesIterator<TsType>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsTypeList {
    syntax_list: SyntaxList,
}
impl TsTypeList {
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
impl AstNode for TsTypeList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsTypeList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsTypeList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<TsNameWithTypeArguments> for TsTypeList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsTypeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsTypeList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TsTypeList {
    type Item = SyntaxResult<TsNameWithTypeArguments>;
    type IntoIter = AstSeparatedListNodesIterator<TsNameWithTypeArguments>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsTypeList {
    type Item = SyntaxResult<TsNameWithTypeArguments>;
    type IntoIter = AstSeparatedListNodesIterator<TsNameWithTypeArguments>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsTypeMemberList {
    syntax_list: SyntaxList,
}
impl TsTypeMemberList {
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
impl AstNode for TsTypeMemberList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_MEMBER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsTypeMemberList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsTypeMemberList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstNodeList<TsAnyTypeMember> for TsTypeMemberList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsTypeMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsTypeMemberList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &TsTypeMemberList {
    type Item = TsAnyTypeMember;
    type IntoIter = AstNodeListIterator<TsAnyTypeMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for TsTypeMemberList {
    type Item = TsAnyTypeMember;
    type IntoIter = AstNodeListIterator<TsAnyTypeMember>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsTypeParameterList {
    syntax_list: SyntaxList,
}
impl TsTypeParameterList {
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
impl AstNode for TsTypeParameterList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_TYPE_PARAMETER_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsTypeParameterList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsTypeParameterList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<TsTypeParameter> for TsTypeParameterList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsTypeParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsTypeParameterList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TsTypeParameterList {
    type Item = SyntaxResult<TsTypeParameter>;
    type IntoIter = AstSeparatedListNodesIterator<TsTypeParameter>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsTypeParameterList {
    type Item = SyntaxResult<TsTypeParameter>;
    type IntoIter = AstSeparatedListNodesIterator<TsTypeParameter>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct TsUnionTypeVariantList {
    syntax_list: SyntaxList,
}
impl TsUnionTypeVariantList {
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
impl AstNode for TsUnionTypeVariantList {
    fn can_cast(kind: JsSyntaxKind) -> bool { kind == TS_UNION_TYPE_VARIANT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<TsUnionTypeVariantList> {
        if Self::can_cast(syntax.kind()) {
            Some(TsUnionTypeVariantList {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { self.syntax_list.node() }
}
impl AstSeparatedList<TsType> for TsUnionTypeVariantList {
    fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
}
impl Debug for TsUnionTypeVariantList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("TsUnionTypeVariantList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for TsUnionTypeVariantList {
    type Item = SyntaxResult<TsType>;
    type IntoIter = AstSeparatedListNodesIterator<TsType>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsUnionTypeVariantList {
    type Item = SyntaxResult<TsType>;
    type IntoIter = AstSeparatedListNodesIterator<TsType>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
pub struct DebugSyntaxElement(pub(crate) SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            NodeOrToken::Node(node) => match node.kind() {
                IMPORT_META => std::fmt::Debug::fmt(&ImportMeta::cast(node.clone()).unwrap(), f),
                JS_ARRAY_ASSIGNMENT_PATTERN => {
                    std::fmt::Debug::fmt(&JsArrayAssignmentPattern::cast(node.clone()).unwrap(), f)
                }
                JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => std::fmt::Debug::fmt(
                    &JsArrayAssignmentPatternElementList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => std::fmt::Debug::fmt(
                    &JsArrayAssignmentPatternRestElement::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_ARRAY_BINDING_PATTERN => {
                    std::fmt::Debug::fmt(&JsArrayBindingPattern::cast(node.clone()).unwrap(), f)
                }
                JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => std::fmt::Debug::fmt(
                    &JsArrayBindingPatternElementList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => std::fmt::Debug::fmt(
                    &JsArrayBindingPatternRestElement::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_ARRAY_ELEMENT_LIST => {
                    std::fmt::Debug::fmt(&JsArrayElementList::cast(node.clone()).unwrap(), f)
                }
                JS_ARRAY_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsArrayExpression::cast(node.clone()).unwrap(), f)
                }
                JS_ARRAY_HOLE => std::fmt::Debug::fmt(&JsArrayHole::cast(node.clone()).unwrap(), f),
                JS_ARROW_FUNCTION_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsArrowFunctionExpression::cast(node.clone()).unwrap(), f)
                }
                JS_ASSIGNMENT_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsAssignmentExpression::cast(node.clone()).unwrap(), f)
                }
                JS_ASSIGNMENT_WITH_DEFAULT => {
                    std::fmt::Debug::fmt(&JsAssignmentWithDefault::cast(node.clone()).unwrap(), f)
                }
                JS_AWAIT_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsAwaitExpression::cast(node.clone()).unwrap(), f)
                }
                JS_BIG_INT_LITERAL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsBigIntLiteralExpression::cast(node.clone()).unwrap(), f)
                }
                JS_BINARY_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsBinaryExpression::cast(node.clone()).unwrap(), f)
                }
                JS_BINDING_PATTERN_WITH_DEFAULT => std::fmt::Debug::fmt(
                    &JsBindingPatternWithDefault::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_BLOCK_STATEMENT => {
                    std::fmt::Debug::fmt(&JsBlockStatement::cast(node.clone()).unwrap(), f)
                }
                JS_BOOLEAN_LITERAL_EXPRESSION => std::fmt::Debug::fmt(
                    &JsBooleanLiteralExpression::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_BREAK_STATEMENT => {
                    std::fmt::Debug::fmt(&JsBreakStatement::cast(node.clone()).unwrap(), f)
                }
                JS_CALL_ARGUMENT_LIST => {
                    std::fmt::Debug::fmt(&JsCallArgumentList::cast(node.clone()).unwrap(), f)
                }
                JS_CALL_ARGUMENTS => {
                    std::fmt::Debug::fmt(&JsCallArguments::cast(node.clone()).unwrap(), f)
                }
                JS_CALL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsCallExpression::cast(node.clone()).unwrap(), f)
                }
                JS_CASE_CLAUSE => {
                    std::fmt::Debug::fmt(&JsCaseClause::cast(node.clone()).unwrap(), f)
                }
                JS_CATCH_CLAUSE => {
                    std::fmt::Debug::fmt(&JsCatchClause::cast(node.clone()).unwrap(), f)
                }
                JS_CATCH_DECLARATION => {
                    std::fmt::Debug::fmt(&JsCatchDeclaration::cast(node.clone()).unwrap(), f)
                }
                JS_CLASS_DECLARATION => {
                    std::fmt::Debug::fmt(&JsClassDeclaration::cast(node.clone()).unwrap(), f)
                }
                JS_CLASS_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsClassExpression::cast(node.clone()).unwrap(), f)
                }
                JS_CLASS_MEMBER_LIST => {
                    std::fmt::Debug::fmt(&JsClassMemberList::cast(node.clone()).unwrap(), f)
                }
                JS_COMPUTED_MEMBER_ASSIGNMENT => std::fmt::Debug::fmt(
                    &JsComputedMemberAssignment::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_COMPUTED_MEMBER_EXPRESSION => std::fmt::Debug::fmt(
                    &JsComputedMemberExpression::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_COMPUTED_MEMBER_NAME => {
                    std::fmt::Debug::fmt(&JsComputedMemberName::cast(node.clone()).unwrap(), f)
                }
                JS_CONDITIONAL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsConditionalExpression::cast(node.clone()).unwrap(), f)
                }
                JS_CONSTRUCTOR_CLASS_MEMBER => {
                    std::fmt::Debug::fmt(&JsConstructorClassMember::cast(node.clone()).unwrap(), f)
                }
                JS_CONSTRUCTOR_PARAMETER_LIST => std::fmt::Debug::fmt(
                    &JsConstructorParameterList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_CONSTRUCTOR_PARAMETERS => {
                    std::fmt::Debug::fmt(&JsConstructorParameters::cast(node.clone()).unwrap(), f)
                }
                JS_CONTINUE_STATEMENT => {
                    std::fmt::Debug::fmt(&JsContinueStatement::cast(node.clone()).unwrap(), f)
                }
                JS_DEBUGGER_STATEMENT => {
                    std::fmt::Debug::fmt(&JsDebuggerStatement::cast(node.clone()).unwrap(), f)
                }
                JS_DEFAULT_CLAUSE => {
                    std::fmt::Debug::fmt(&JsDefaultClause::cast(node.clone()).unwrap(), f)
                }
                JS_DEFAULT_IMPORT_SPECIFIER => {
                    std::fmt::Debug::fmt(&JsDefaultImportSpecifier::cast(node.clone()).unwrap(), f)
                }
                JS_DIRECTIVE => std::fmt::Debug::fmt(&JsDirective::cast(node.clone()).unwrap(), f),
                JS_DIRECTIVE_LIST => {
                    std::fmt::Debug::fmt(&JsDirectiveList::cast(node.clone()).unwrap(), f)
                }
                JS_DO_WHILE_STATEMENT => {
                    std::fmt::Debug::fmt(&JsDoWhileStatement::cast(node.clone()).unwrap(), f)
                }
                JS_ELSE_CLAUSE => {
                    std::fmt::Debug::fmt(&JsElseClause::cast(node.clone()).unwrap(), f)
                }
                JS_EMPTY_CLASS_MEMBER => {
                    std::fmt::Debug::fmt(&JsEmptyClassMember::cast(node.clone()).unwrap(), f)
                }
                JS_EMPTY_STATEMENT => {
                    std::fmt::Debug::fmt(&JsEmptyStatement::cast(node.clone()).unwrap(), f)
                }
                JS_EXPORT => std::fmt::Debug::fmt(&JsExport::cast(node.clone()).unwrap(), f),
                JS_EXPORT_AS_CLAUSE => {
                    std::fmt::Debug::fmt(&JsExportAsClause::cast(node.clone()).unwrap(), f)
                }
                JS_EXPORT_DEFAULT_CLASS_CLAUSE => std::fmt::Debug::fmt(
                    &JsExportDefaultClassClause::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => std::fmt::Debug::fmt(
                    &JsExportDefaultExpressionClause::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_EXPORT_DEFAULT_FUNCTION_CLAUSE => std::fmt::Debug::fmt(
                    &JsExportDefaultFunctionClause::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_EXPORT_FROM_CLAUSE => {
                    std::fmt::Debug::fmt(&JsExportFromClause::cast(node.clone()).unwrap(), f)
                }
                JS_EXPORT_NAMED_CLAUSE => {
                    std::fmt::Debug::fmt(&JsExportNamedClause::cast(node.clone()).unwrap(), f)
                }
                JS_EXPORT_NAMED_FROM_CLAUSE => {
                    std::fmt::Debug::fmt(&JsExportNamedFromClause::cast(node.clone()).unwrap(), f)
                }
                JS_EXPORT_NAMED_FROM_SPECIFIER => std::fmt::Debug::fmt(
                    &JsExportNamedFromSpecifier::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_EXPORT_NAMED_FROM_SPECIFIER_LIST => std::fmt::Debug::fmt(
                    &JsExportNamedFromSpecifierList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => std::fmt::Debug::fmt(
                    &JsExportNamedShorthandSpecifier::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_EXPORT_NAMED_SPECIFIER => {
                    std::fmt::Debug::fmt(&JsExportNamedSpecifier::cast(node.clone()).unwrap(), f)
                }
                JS_EXPORT_NAMED_SPECIFIER_LIST => std::fmt::Debug::fmt(
                    &JsExportNamedSpecifierList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_EXPRESSION_SNIPPED => {
                    std::fmt::Debug::fmt(&JsExpressionSnipped::cast(node.clone()).unwrap(), f)
                }
                JS_EXPRESSION_STATEMENT => {
                    std::fmt::Debug::fmt(&JsExpressionStatement::cast(node.clone()).unwrap(), f)
                }
                JS_EXTENDS_CLAUSE => {
                    std::fmt::Debug::fmt(&JsExtendsClause::cast(node.clone()).unwrap(), f)
                }
                JS_FINALLY_CLAUSE => {
                    std::fmt::Debug::fmt(&JsFinallyClause::cast(node.clone()).unwrap(), f)
                }
                JS_FOR_IN_STATEMENT => {
                    std::fmt::Debug::fmt(&JsForInStatement::cast(node.clone()).unwrap(), f)
                }
                JS_FOR_OF_STATEMENT => {
                    std::fmt::Debug::fmt(&JsForOfStatement::cast(node.clone()).unwrap(), f)
                }
                JS_FOR_STATEMENT => {
                    std::fmt::Debug::fmt(&JsForStatement::cast(node.clone()).unwrap(), f)
                }
                JS_FOR_VARIABLE_DECLARATION => {
                    std::fmt::Debug::fmt(&JsForVariableDeclaration::cast(node.clone()).unwrap(), f)
                }
                JS_FORMAL_PARAMETER => {
                    std::fmt::Debug::fmt(&JsFormalParameter::cast(node.clone()).unwrap(), f)
                }
                JS_FUNCTION_BODY => {
                    std::fmt::Debug::fmt(&JsFunctionBody::cast(node.clone()).unwrap(), f)
                }
                JS_FUNCTION_DECLARATION => {
                    std::fmt::Debug::fmt(&JsFunctionDeclaration::cast(node.clone()).unwrap(), f)
                }
                JS_FUNCTION_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsFunctionExpression::cast(node.clone()).unwrap(), f)
                }
                JS_GETTER_CLASS_MEMBER => {
                    std::fmt::Debug::fmt(&JsGetterClassMember::cast(node.clone()).unwrap(), f)
                }
                JS_GETTER_OBJECT_MEMBER => {
                    std::fmt::Debug::fmt(&JsGetterObjectMember::cast(node.clone()).unwrap(), f)
                }
                JS_IDENTIFIER_ASSIGNMENT => {
                    std::fmt::Debug::fmt(&JsIdentifierAssignment::cast(node.clone()).unwrap(), f)
                }
                JS_IDENTIFIER_BINDING => {
                    std::fmt::Debug::fmt(&JsIdentifierBinding::cast(node.clone()).unwrap(), f)
                }
                JS_IDENTIFIER_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsIdentifierExpression::cast(node.clone()).unwrap(), f)
                }
                JS_IF_STATEMENT => {
                    std::fmt::Debug::fmt(&JsIfStatement::cast(node.clone()).unwrap(), f)
                }
                JS_IMPORT => std::fmt::Debug::fmt(&JsImport::cast(node.clone()).unwrap(), f),
                JS_IMPORT_ASSERTION => {
                    std::fmt::Debug::fmt(&JsImportAssertion::cast(node.clone()).unwrap(), f)
                }
                JS_IMPORT_ASSERTION_ENTRY => {
                    std::fmt::Debug::fmt(&JsImportAssertionEntry::cast(node.clone()).unwrap(), f)
                }
                JS_IMPORT_ASSERTION_ENTRY_LIST => std::fmt::Debug::fmt(
                    &JsImportAssertionEntryList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_IMPORT_BARE_CLAUSE => {
                    std::fmt::Debug::fmt(&JsImportBareClause::cast(node.clone()).unwrap(), f)
                }
                JS_IMPORT_CALL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsImportCallExpression::cast(node.clone()).unwrap(), f)
                }
                JS_IMPORT_DEFAULT_CLAUSE => {
                    std::fmt::Debug::fmt(&JsImportDefaultClause::cast(node.clone()).unwrap(), f)
                }
                JS_IMPORT_NAMED_CLAUSE => {
                    std::fmt::Debug::fmt(&JsImportNamedClause::cast(node.clone()).unwrap(), f)
                }
                JS_IMPORT_NAMESPACE_CLAUSE => {
                    std::fmt::Debug::fmt(&JsImportNamespaceClause::cast(node.clone()).unwrap(), f)
                }
                JS_IN_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsInExpression::cast(node.clone()).unwrap(), f)
                }
                JS_INITIALIZER_CLAUSE => {
                    std::fmt::Debug::fmt(&JsInitializerClause::cast(node.clone()).unwrap(), f)
                }
                JS_INSTANCEOF_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsInstanceofExpression::cast(node.clone()).unwrap(), f)
                }
                JS_LABELED_STATEMENT => {
                    std::fmt::Debug::fmt(&JsLabeledStatement::cast(node.clone()).unwrap(), f)
                }
                JS_LITERAL_EXPORT_NAME => {
                    std::fmt::Debug::fmt(&JsLiteralExportName::cast(node.clone()).unwrap(), f)
                }
                JS_LITERAL_MEMBER_NAME => {
                    std::fmt::Debug::fmt(&JsLiteralMemberName::cast(node.clone()).unwrap(), f)
                }
                JS_LOGICAL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsLogicalExpression::cast(node.clone()).unwrap(), f)
                }
                JS_METHOD_CLASS_MEMBER => {
                    std::fmt::Debug::fmt(&JsMethodClassMember::cast(node.clone()).unwrap(), f)
                }
                JS_METHOD_OBJECT_MEMBER => {
                    std::fmt::Debug::fmt(&JsMethodObjectMember::cast(node.clone()).unwrap(), f)
                }
                JS_MODULE => std::fmt::Debug::fmt(&JsModule::cast(node.clone()).unwrap(), f),
                JS_MODULE_ITEM_LIST => {
                    std::fmt::Debug::fmt(&JsModuleItemList::cast(node.clone()).unwrap(), f)
                }
                JS_MODULE_SOURCE => {
                    std::fmt::Debug::fmt(&JsModuleSource::cast(node.clone()).unwrap(), f)
                }
                JS_NAME => std::fmt::Debug::fmt(&JsName::cast(node.clone()).unwrap(), f),
                JS_NAMED_IMPORT_SPECIFIER => {
                    std::fmt::Debug::fmt(&JsNamedImportSpecifier::cast(node.clone()).unwrap(), f)
                }
                JS_NAMED_IMPORT_SPECIFIER_LIST => std::fmt::Debug::fmt(
                    &JsNamedImportSpecifierList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_NAMED_IMPORT_SPECIFIERS => {
                    std::fmt::Debug::fmt(&JsNamedImportSpecifiers::cast(node.clone()).unwrap(), f)
                }
                JS_NAMESPACE_IMPORT_SPECIFIER => std::fmt::Debug::fmt(
                    &JsNamespaceImportSpecifier::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_NEW_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsNewExpression::cast(node.clone()).unwrap(), f)
                }
                JS_NULL_LITERAL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsNullLiteralExpression::cast(node.clone()).unwrap(), f)
                }
                JS_NUMBER_LITERAL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsNumberLiteralExpression::cast(node.clone()).unwrap(), f)
                }
                JS_OBJECT_ASSIGNMENT_PATTERN => {
                    std::fmt::Debug::fmt(&JsObjectAssignmentPattern::cast(node.clone()).unwrap(), f)
                }
                JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => std::fmt::Debug::fmt(
                    &JsObjectAssignmentPatternProperty::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => std::fmt::Debug::fmt(
                    &JsObjectAssignmentPatternPropertyList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_OBJECT_ASSIGNMENT_PATTERN_REST => std::fmt::Debug::fmt(
                    &JsObjectAssignmentPatternRest::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => std::fmt::Debug::fmt(
                    &JsObjectAssignmentPatternShorthandProperty::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_OBJECT_BINDING_PATTERN => {
                    std::fmt::Debug::fmt(&JsObjectBindingPattern::cast(node.clone()).unwrap(), f)
                }
                JS_OBJECT_BINDING_PATTERN_PROPERTY => std::fmt::Debug::fmt(
                    &JsObjectBindingPatternProperty::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => std::fmt::Debug::fmt(
                    &JsObjectBindingPatternPropertyList::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_OBJECT_BINDING_PATTERN_REST => std::fmt::Debug::fmt(
                    &JsObjectBindingPatternRest::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => std::fmt::Debug::fmt(
                    &JsObjectBindingPatternShorthandProperty::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_OBJECT_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsObjectExpression::cast(node.clone()).unwrap(), f)
                }
                JS_OBJECT_MEMBER_LIST => {
                    std::fmt::Debug::fmt(&JsObjectMemberList::cast(node.clone()).unwrap(), f)
                }
                JS_PARAMETER_LIST => {
                    std::fmt::Debug::fmt(&JsParameterList::cast(node.clone()).unwrap(), f)
                }
                JS_PARAMETERS => {
                    std::fmt::Debug::fmt(&JsParameters::cast(node.clone()).unwrap(), f)
                }
                JS_PARENTHESIZED_ASSIGNMENT => {
                    std::fmt::Debug::fmt(&JsParenthesizedAssignment::cast(node.clone()).unwrap(), f)
                }
                JS_PARENTHESIZED_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsParenthesizedExpression::cast(node.clone()).unwrap(), f)
                }
                JS_POST_UPDATE_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsPostUpdateExpression::cast(node.clone()).unwrap(), f)
                }
                JS_PRE_UPDATE_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsPreUpdateExpression::cast(node.clone()).unwrap(), f)
                }
                JS_PRIVATE_CLASS_MEMBER_NAME => {
                    std::fmt::Debug::fmt(&JsPrivateClassMemberName::cast(node.clone()).unwrap(), f)
                }
                JS_PRIVATE_NAME => {
                    std::fmt::Debug::fmt(&JsPrivateName::cast(node.clone()).unwrap(), f)
                }
                JS_PROPERTY_CLASS_MEMBER => {
                    std::fmt::Debug::fmt(&JsPropertyClassMember::cast(node.clone()).unwrap(), f)
                }
                JS_PROPERTY_OBJECT_MEMBER => {
                    std::fmt::Debug::fmt(&JsPropertyObjectMember::cast(node.clone()).unwrap(), f)
                }
                JS_REFERENCE_IDENTIFIER => {
                    std::fmt::Debug::fmt(&JsReferenceIdentifier::cast(node.clone()).unwrap(), f)
                }
                JS_REGEX_LITERAL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsRegexLiteralExpression::cast(node.clone()).unwrap(), f)
                }
                JS_REST_PARAMETER => {
                    std::fmt::Debug::fmt(&JsRestParameter::cast(node.clone()).unwrap(), f)
                }
                JS_RETURN_STATEMENT => {
                    std::fmt::Debug::fmt(&JsReturnStatement::cast(node.clone()).unwrap(), f)
                }
                JS_SCRIPT => std::fmt::Debug::fmt(&JsScript::cast(node.clone()).unwrap(), f),
                JS_SEQUENCE_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsSequenceExpression::cast(node.clone()).unwrap(), f)
                }
                JS_SETTER_CLASS_MEMBER => {
                    std::fmt::Debug::fmt(&JsSetterClassMember::cast(node.clone()).unwrap(), f)
                }
                JS_SETTER_OBJECT_MEMBER => {
                    std::fmt::Debug::fmt(&JsSetterObjectMember::cast(node.clone()).unwrap(), f)
                }
                JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => std::fmt::Debug::fmt(
                    &JsShorthandNamedImportSpecifier::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => std::fmt::Debug::fmt(
                    &JsShorthandPropertyObjectMember::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_SPREAD => std::fmt::Debug::fmt(&JsSpread::cast(node.clone()).unwrap(), f),
                JS_STATEMENT_LIST => {
                    std::fmt::Debug::fmt(&JsStatementList::cast(node.clone()).unwrap(), f)
                }
                JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => std::fmt::Debug::fmt(
                    &JsStaticInitializationBlockClassMember::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_STATIC_MEMBER_ASSIGNMENT => {
                    std::fmt::Debug::fmt(&JsStaticMemberAssignment::cast(node.clone()).unwrap(), f)
                }
                JS_STATIC_MEMBER_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsStaticMemberExpression::cast(node.clone()).unwrap(), f)
                }
                JS_STRING_LITERAL_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsStringLiteralExpression::cast(node.clone()).unwrap(), f)
                }
                JS_SUPER_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsSuperExpression::cast(node.clone()).unwrap(), f)
                }
                JS_SWITCH_CASE_LIST => {
                    std::fmt::Debug::fmt(&JsSwitchCaseList::cast(node.clone()).unwrap(), f)
                }
                JS_SWITCH_STATEMENT => {
                    std::fmt::Debug::fmt(&JsSwitchStatement::cast(node.clone()).unwrap(), f)
                }
                JS_TEMPLATE => std::fmt::Debug::fmt(&JsTemplate::cast(node.clone()).unwrap(), f),
                JS_TEMPLATE_CHUNK_ELEMENT => {
                    std::fmt::Debug::fmt(&JsTemplateChunkElement::cast(node.clone()).unwrap(), f)
                }
                JS_TEMPLATE_ELEMENT => {
                    std::fmt::Debug::fmt(&JsTemplateElement::cast(node.clone()).unwrap(), f)
                }
                JS_TEMPLATE_ELEMENT_LIST => {
                    std::fmt::Debug::fmt(&JsTemplateElementList::cast(node.clone()).unwrap(), f)
                }
                JS_THIS_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsThisExpression::cast(node.clone()).unwrap(), f)
                }
                JS_THROW_STATEMENT => {
                    std::fmt::Debug::fmt(&JsThrowStatement::cast(node.clone()).unwrap(), f)
                }
                JS_TRY_FINALLY_STATEMENT => {
                    std::fmt::Debug::fmt(&JsTryFinallyStatement::cast(node.clone()).unwrap(), f)
                }
                JS_TRY_STATEMENT => {
                    std::fmt::Debug::fmt(&JsTryStatement::cast(node.clone()).unwrap(), f)
                }
                JS_UNARY_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsUnaryExpression::cast(node.clone()).unwrap(), f)
                }
                JS_UNKNOWN => std::fmt::Debug::fmt(&JsUnknown::cast(node.clone()).unwrap(), f),
                JS_UNKNOWN_ASSIGNMENT => {
                    std::fmt::Debug::fmt(&JsUnknownAssignment::cast(node.clone()).unwrap(), f)
                }
                JS_UNKNOWN_BINDING => {
                    std::fmt::Debug::fmt(&JsUnknownBinding::cast(node.clone()).unwrap(), f)
                }
                JS_UNKNOWN_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsUnknownExpression::cast(node.clone()).unwrap(), f)
                }
                JS_UNKNOWN_IMPORT_ASSERTION_ENTRY => std::fmt::Debug::fmt(
                    &JsUnknownImportAssertionEntry::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_UNKNOWN_MEMBER => {
                    std::fmt::Debug::fmt(&JsUnknownMember::cast(node.clone()).unwrap(), f)
                }
                JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => std::fmt::Debug::fmt(
                    &JsUnknownNamedImportSpecifier::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_UNKNOWN_PARAMETER => {
                    std::fmt::Debug::fmt(&JsUnknownParameter::cast(node.clone()).unwrap(), f)
                }
                JS_UNKNOWN_STATEMENT => {
                    std::fmt::Debug::fmt(&JsUnknownStatement::cast(node.clone()).unwrap(), f)
                }
                JS_VARIABLE_DECLARATION => {
                    std::fmt::Debug::fmt(&JsVariableDeclaration::cast(node.clone()).unwrap(), f)
                }
                JS_VARIABLE_DECLARATION_CLAUSE => std::fmt::Debug::fmt(
                    &JsVariableDeclarationClause::cast(node.clone()).unwrap(),
                    f,
                ),
                JS_VARIABLE_DECLARATOR => {
                    std::fmt::Debug::fmt(&JsVariableDeclarator::cast(node.clone()).unwrap(), f)
                }
                JS_VARIABLE_DECLARATOR_LIST => {
                    std::fmt::Debug::fmt(&JsVariableDeclaratorList::cast(node.clone()).unwrap(), f)
                }
                JS_VARIABLE_STATEMENT => {
                    std::fmt::Debug::fmt(&JsVariableStatement::cast(node.clone()).unwrap(), f)
                }
                JS_WHILE_STATEMENT => {
                    std::fmt::Debug::fmt(&JsWhileStatement::cast(node.clone()).unwrap(), f)
                }
                JS_WITH_STATEMENT => {
                    std::fmt::Debug::fmt(&JsWithStatement::cast(node.clone()).unwrap(), f)
                }
                JS_YIELD_ARGUMENT => {
                    std::fmt::Debug::fmt(&JsYieldArgument::cast(node.clone()).unwrap(), f)
                }
                JS_YIELD_EXPRESSION => {
                    std::fmt::Debug::fmt(&JsYieldExpression::cast(node.clone()).unwrap(), f)
                }
                NEW_TARGET => std::fmt::Debug::fmt(&NewTarget::cast(node.clone()).unwrap(), f),
                TS_ANY_TYPE => std::fmt::Debug::fmt(&TsAnyType::cast(node.clone()).unwrap(), f),
                TS_ARRAY_TYPE => std::fmt::Debug::fmt(&TsArrayType::cast(node.clone()).unwrap(), f),
                TS_AS_EXPRESSION => {
                    std::fmt::Debug::fmt(&TsAsExpression::cast(node.clone()).unwrap(), f)
                }
                TS_BIG_INT_LITERAL_TYPE => {
                    std::fmt::Debug::fmt(&TsBigIntLiteralType::cast(node.clone()).unwrap(), f)
                }
                TS_BIGINT_TYPE => {
                    std::fmt::Debug::fmt(&TsBigintType::cast(node.clone()).unwrap(), f)
                }
                TS_BOOLEAN_LITERAL_TYPE => {
                    std::fmt::Debug::fmt(&TsBooleanLiteralType::cast(node.clone()).unwrap(), f)
                }
                TS_BOOLEAN_TYPE => {
                    std::fmt::Debug::fmt(&TsBooleanType::cast(node.clone()).unwrap(), f)
                }
                TS_CALL_SIGNATURE_TYPE_MEMBER => {
                    std::fmt::Debug::fmt(&TsCallSignatureTypeMember::cast(node.clone()).unwrap(), f)
                }
                TS_CONDITIONAL_TYPE => {
                    std::fmt::Debug::fmt(&TsConditionalType::cast(node.clone()).unwrap(), f)
                }
                TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER => std::fmt::Debug::fmt(
                    &TsConstructSignatureTypeMember::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_CONSTRUCTOR_TYPE => {
                    std::fmt::Debug::fmt(&TsConstructorType::cast(node.clone()).unwrap(), f)
                }
                TS_DECLARE_FUNCTION_DECLARATION => std::fmt::Debug::fmt(
                    &TsDeclareFunctionDeclaration::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_DECLARE_STATEMENT => {
                    std::fmt::Debug::fmt(&TsDeclareStatement::cast(node.clone()).unwrap(), f)
                }
                TS_DEFAULT_TYPE_CLAUSE => {
                    std::fmt::Debug::fmt(&TsDefaultTypeClause::cast(node.clone()).unwrap(), f)
                }
                TS_DEFINITE_PROPERTY_ANNOTATION => std::fmt::Debug::fmt(
                    &TsDefinitePropertyAnnotation::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_DEFINITE_VARIABLE_ANNOTATION => std::fmt::Debug::fmt(
                    &TsDefiniteVariableAnnotation::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY => std::fmt::Debug::fmt(
                    &TsEmptyExternalModuleDeclarationBody::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_ENUM_DECLARATION => {
                    std::fmt::Debug::fmt(&TsEnumDeclaration::cast(node.clone()).unwrap(), f)
                }
                TS_ENUM_MEMBER => {
                    std::fmt::Debug::fmt(&TsEnumMember::cast(node.clone()).unwrap(), f)
                }
                TS_ENUM_MEMBER_LIST => {
                    std::fmt::Debug::fmt(&TsEnumMemberList::cast(node.clone()).unwrap(), f)
                }
                TS_EXTENDS_CLAUSE => {
                    std::fmt::Debug::fmt(&TsExtendsClause::cast(node.clone()).unwrap(), f)
                }
                TS_EXTERNAL_MODULE_DECLARATION => std::fmt::Debug::fmt(
                    &TsExternalModuleDeclaration::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_EXTERNAL_MODULE_REF => {
                    std::fmt::Debug::fmt(&TsExternalModuleRef::cast(node.clone()).unwrap(), f)
                }
                TS_FUNCTION_TYPE => {
                    std::fmt::Debug::fmt(&TsFunctionType::cast(node.clone()).unwrap(), f)
                }
                TS_GETTER_SIGNATURE_TYPE_MEMBER => std::fmt::Debug::fmt(
                    &TsGetterSignatureTypeMember::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_GLOBAL_DECLARATION => {
                    std::fmt::Debug::fmt(&TsGlobalDeclaration::cast(node.clone()).unwrap(), f)
                }
                TS_IDENTIFIER_BINDING => {
                    std::fmt::Debug::fmt(&TsIdentifierBinding::cast(node.clone()).unwrap(), f)
                }
                TS_IMPLEMENTS_CLAUSE => {
                    std::fmt::Debug::fmt(&TsImplementsClause::cast(node.clone()).unwrap(), f)
                }
                TS_IMPORT_EQUALS_DECL => {
                    std::fmt::Debug::fmt(&TsImportEqualsDecl::cast(node.clone()).unwrap(), f)
                }
                TS_IMPORT_TYPE => {
                    std::fmt::Debug::fmt(&TsImportType::cast(node.clone()).unwrap(), f)
                }
                TS_IMPORT_TYPE_QUALIFIER => {
                    std::fmt::Debug::fmt(&TsImportTypeQualifier::cast(node.clone()).unwrap(), f)
                }
                TS_INDEX_SIGNATURE_PARAMETER => {
                    std::fmt::Debug::fmt(&TsIndexSignatureParameter::cast(node.clone()).unwrap(), f)
                }
                TS_INDEX_SIGNATURE_TYPE_MEMBER => std::fmt::Debug::fmt(
                    &TsIndexSignatureTypeMember::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_INDEXED_ACCESS_TYPE => {
                    std::fmt::Debug::fmt(&TsIndexedAccessType::cast(node.clone()).unwrap(), f)
                }
                TS_INFER_TYPE => std::fmt::Debug::fmt(&TsInferType::cast(node.clone()).unwrap(), f),
                TS_INTERFACE_DECLARATION => {
                    std::fmt::Debug::fmt(&TsInterfaceDeclaration::cast(node.clone()).unwrap(), f)
                }
                TS_INTERSECTION_TYPE => {
                    std::fmt::Debug::fmt(&TsIntersectionType::cast(node.clone()).unwrap(), f)
                }
                TS_INTERSECTION_TYPE_ELEMENT_LIST => std::fmt::Debug::fmt(
                    &TsIntersectionTypeElementList::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_MAPPED_TYPE => {
                    std::fmt::Debug::fmt(&TsMappedType::cast(node.clone()).unwrap(), f)
                }
                TS_MAPPED_TYPE_AS_CLAUSE => {
                    std::fmt::Debug::fmt(&TsMappedTypeAsClause::cast(node.clone()).unwrap(), f)
                }
                TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE => std::fmt::Debug::fmt(
                    &TsMappedTypeOptionalModifierClause::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE => std::fmt::Debug::fmt(
                    &TsMappedTypeReadonlyModifierClause::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_METHOD_SIGNATURE_TYPE_MEMBER => std::fmt::Debug::fmt(
                    &TsMethodSignatureTypeMember::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_MODULE_BLOCK => {
                    std::fmt::Debug::fmt(&TsModuleBlock::cast(node.clone()).unwrap(), f)
                }
                TS_MODULE_DECLARATION => {
                    std::fmt::Debug::fmt(&TsModuleDeclaration::cast(node.clone()).unwrap(), f)
                }
                TS_NAME_WITH_TYPE_ARGUMENTS => {
                    std::fmt::Debug::fmt(&TsNameWithTypeArguments::cast(node.clone()).unwrap(), f)
                }
                TS_NAMED_TUPLE_TYPE_ELEMENT => {
                    std::fmt::Debug::fmt(&TsNamedTupleTypeElement::cast(node.clone()).unwrap(), f)
                }
                TS_NEVER_TYPE => std::fmt::Debug::fmt(&TsNeverType::cast(node.clone()).unwrap(), f),
                TS_NON_NULL_ASSERTION_ASSIGNMENT => std::fmt::Debug::fmt(
                    &TsNonNullAssertionAssignment::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_NON_NULL_ASSERTION_EXPRESSION => std::fmt::Debug::fmt(
                    &TsNonNullAssertionExpression::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_NON_PRIMITIVE_TYPE => {
                    std::fmt::Debug::fmt(&TsNonPrimitiveType::cast(node.clone()).unwrap(), f)
                }
                TS_NULL_LITERAL_TYPE => {
                    std::fmt::Debug::fmt(&TsNullLiteralType::cast(node.clone()).unwrap(), f)
                }
                TS_NUMBER_LITERAL_TYPE => {
                    std::fmt::Debug::fmt(&TsNumberLiteralType::cast(node.clone()).unwrap(), f)
                }
                TS_NUMBER_TYPE => {
                    std::fmt::Debug::fmt(&TsNumberType::cast(node.clone()).unwrap(), f)
                }
                TS_OBJECT_TYPE => {
                    std::fmt::Debug::fmt(&TsObjectType::cast(node.clone()).unwrap(), f)
                }
                TS_OPTIONAL_PROPERTY_ANNOTATION => std::fmt::Debug::fmt(
                    &TsOptionalPropertyAnnotation::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_OPTIONAL_TUPLE_TYPE_ELEMENT => std::fmt::Debug::fmt(
                    &TsOptionalTupleTypeElement::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_PARENTHESIZED_TYPE => {
                    std::fmt::Debug::fmt(&TsParenthesizedType::cast(node.clone()).unwrap(), f)
                }
                TS_PROPERTY_PARAMETER => {
                    std::fmt::Debug::fmt(&TsPropertyParameter::cast(node.clone()).unwrap(), f)
                }
                TS_PROPERTY_SIGNATURE_TYPE_MEMBER => std::fmt::Debug::fmt(
                    &TsPropertySignatureTypeMember::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_QUALIFIED_MODULE_NAME => {
                    std::fmt::Debug::fmt(&TsQualifiedModuleName::cast(node.clone()).unwrap(), f)
                }
                TS_QUALIFIED_NAME => {
                    std::fmt::Debug::fmt(&TsQualifiedName::cast(node.clone()).unwrap(), f)
                }
                TS_READONLY_PROPERTY_PARAMETER => std::fmt::Debug::fmt(
                    &TsReadonlyPropertyParameter::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_REFERENCE_TYPE => {
                    std::fmt::Debug::fmt(&TsReferenceType::cast(node.clone()).unwrap(), f)
                }
                TS_REST_TUPLE_TYPE_ELEMENT => {
                    std::fmt::Debug::fmt(&TsRestTupleTypeElement::cast(node.clone()).unwrap(), f)
                }
                TS_RETURN_TYPE_ANNOTATION => {
                    std::fmt::Debug::fmt(&TsReturnTypeAnnotation::cast(node.clone()).unwrap(), f)
                }
                TS_SETTER_SIGNATURE_TYPE_MEMBER => std::fmt::Debug::fmt(
                    &TsSetterSignatureTypeMember::cast(node.clone()).unwrap(),
                    f,
                ),
                TS_STRING_LITERAL_TYPE => {
                    std::fmt::Debug::fmt(&TsStringLiteralType::cast(node.clone()).unwrap(), f)
                }
                TS_STRING_TYPE => {
                    std::fmt::Debug::fmt(&TsStringType::cast(node.clone()).unwrap(), f)
                }
                TS_SYMBOL_TYPE => {
                    std::fmt::Debug::fmt(&TsSymbolType::cast(node.clone()).unwrap(), f)
                }
                TS_TEMPLATE_CHUNK_ELEMENT => {
                    std::fmt::Debug::fmt(&TsTemplateChunkElement::cast(node.clone()).unwrap(), f)
                }
                TS_TEMPLATE_ELEMENT => {
                    std::fmt::Debug::fmt(&TsTemplateElement::cast(node.clone()).unwrap(), f)
                }
                TS_TEMPLATE_ELEMENT_LIST => {
                    std::fmt::Debug::fmt(&TsTemplateElementList::cast(node.clone()).unwrap(), f)
                }
                TS_TEMPLATE_LITERAL_TYPE => {
                    std::fmt::Debug::fmt(&TsTemplateLiteralType::cast(node.clone()).unwrap(), f)
                }
                TS_THIS_PARAMETER => {
                    std::fmt::Debug::fmt(&TsThisParameter::cast(node.clone()).unwrap(), f)
                }
                TS_THIS_TYPE => std::fmt::Debug::fmt(&TsThisType::cast(node.clone()).unwrap(), f),
                TS_TUPLE_TYPE => std::fmt::Debug::fmt(&TsTupleType::cast(node.clone()).unwrap(), f),
                TS_TUPLE_TYPE_ELEMENT_LIST => {
                    std::fmt::Debug::fmt(&TsTupleTypeElementList::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_ALIAS_DECLARATION => {
                    std::fmt::Debug::fmt(&TsTypeAliasDeclaration::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_ANNOTATION => {
                    std::fmt::Debug::fmt(&TsTypeAnnotation::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_ARGUMENT_LIST => {
                    std::fmt::Debug::fmt(&TsTypeArgumentList::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_ARGUMENTS => {
                    std::fmt::Debug::fmt(&TsTypeArguments::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_ASSERTION_EXPRESSION => {
                    std::fmt::Debug::fmt(&TsTypeAssertionExpression::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_CONSTRAINT_CLAUSE => {
                    std::fmt::Debug::fmt(&TsTypeConstraintClause::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_LIST => std::fmt::Debug::fmt(&TsTypeList::cast(node.clone()).unwrap(), f),
                TS_TYPE_MEMBER_LIST => {
                    std::fmt::Debug::fmt(&TsTypeMemberList::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_OPERATOR_TYPE => {
                    std::fmt::Debug::fmt(&TsTypeOperatorType::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_PARAMETER => {
                    std::fmt::Debug::fmt(&TsTypeParameter::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_PARAMETER_LIST => {
                    std::fmt::Debug::fmt(&TsTypeParameterList::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_PARAMETER_NAME => {
                    std::fmt::Debug::fmt(&TsTypeParameterName::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_PARAMETERS => {
                    std::fmt::Debug::fmt(&TsTypeParameters::cast(node.clone()).unwrap(), f)
                }
                TS_TYPE_PREDICATE => {
                    std::fmt::Debug::fmt(&TsTypePredicate::cast(node.clone()).unwrap(), f)
                }
                TS_TYPEOF_TYPE => {
                    std::fmt::Debug::fmt(&TsTypeofType::cast(node.clone()).unwrap(), f)
                }
                TS_UNDEFINED_TYPE => {
                    std::fmt::Debug::fmt(&TsUndefinedType::cast(node.clone()).unwrap(), f)
                }
                TS_UNION_TYPE => std::fmt::Debug::fmt(&TsUnionType::cast(node.clone()).unwrap(), f),
                TS_UNION_TYPE_VARIANT_LIST => {
                    std::fmt::Debug::fmt(&TsUnionTypeVariantList::cast(node.clone()).unwrap(), f)
                }
                TS_UNKNOWN_TYPE => {
                    std::fmt::Debug::fmt(&TsUnknownType::cast(node.clone()).unwrap(), f)
                }
                TS_VOID_TYPE => std::fmt::Debug::fmt(&TsVoidType::cast(node.clone()).unwrap(), f),
                _ => std::fmt::Debug::fmt(node, f),
            },
            NodeOrToken::Token(token) => Debug::fmt(token, f),
        }
    }
}
