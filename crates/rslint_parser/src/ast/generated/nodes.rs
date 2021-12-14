//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(clippy::enum_variant_names)]
#![allow(clippy::match_like_matches_macro)]
use crate::{
	ast::*,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxResult, SyntaxToken, T,
};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
	pub(crate) syntax: SyntaxNode,
}
impl CallExpr {
	pub fn callee(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn type_args(&self) -> Option<TsTypeArgs> { support::node(&self.syntax) }
	pub fn arguments(&self) -> SyntaxResult<JsCallArguments> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportDecl {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn decl(&self) -> SyntaxResult<JsAnyExportDeclaration> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportDefaultDecl {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDefaultDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn decl(&self) -> SyntaxResult<DefaultDecl> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportDefaultExpr {
	pub(crate) syntax: SyntaxNode,
}
impl ExportDefaultExpr {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportNamed {
	pub(crate) syntax: SyntaxNode,
}
impl ExportNamed {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn specifiers(&self) -> ExportNamedSpecifierList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
	pub fn from_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![from]) }
	pub fn js_string_literal_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![js_string_literal])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExportWildcard {
	pub(crate) syntax: SyntaxNode,
}
impl ExportWildcard {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
	pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [*])
	}
	pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
	pub fn ident(&self) -> Option<Ident> { support::node(&self.syntax) }
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![from])
	}
	pub fn source_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForStmt {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmt {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![for])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn initializer(&self) -> Option<JsAnyForInitializer> { support::node(&self.syntax) }
	pub fn first_semi_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [;])
	}
	pub fn test(&self) -> Option<ForStmtTest> { support::node(&self.syntax) }
	pub fn update(&self) -> Option<ForStmtUpdate> { support::node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn cons(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForStmtTest {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmtTest {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ForStmtUpdate {
	pub(crate) syntax: SyntaxNode,
}
impl ForStmtUpdate {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ident {
	pub(crate) syntax: SyntaxNode,
}
impl Ident {
	pub fn ident_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImportMeta {
	pub(crate) syntax: SyntaxNode,
}
impl ImportMeta {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [.])
	}
	pub fn meta_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![meta])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentPattern {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> JsArrayAssignmentPatternElementList { support::list(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayAssignmentPatternRestElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayAssignmentPatternRestElement {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayBindingPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayBindingPattern {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> JsArrayBindingPatternElementList { support::list(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayBindingPatternRestElement {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayBindingPatternRestElement {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayExpression {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> JsArrayElementList { support::list(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrayHole {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrayHole {}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsArrowFunctionExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsArrowFunctionExpression {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn type_parameters(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameters(&self) -> Option<JsAnyArrowFunctionParameters> { support::node(&self.syntax) }
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=>])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAssignmentExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsAssignmentExpression {
	pub fn left(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
		support::required_node(&self.syntax)
	}
	pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(
			&self.syntax,
			&[
				T ! [=],
				T ! [+=],
				T ! [-=],
				T ! [*=],
				T ! [/=],
				T ! [%=],
				T ! [**=],
				T ! [>>=],
				T ! [<<=],
				T ! [>>>=],
				T ! [&=],
				T ! [|=],
				T ! [^=],
				T ! [&&=],
				T ! [||=],
				T ! [??=],
			],
		)
	}
	pub fn right(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAssignmentWithDefault {
	pub(crate) syntax: SyntaxNode,
}
impl JsAssignmentWithDefault {
	pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
		support::required_node(&self.syntax)
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn default(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsAwaitExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsAwaitExpression {
	pub fn await_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![await])
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBigIntLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsBigIntLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_big_int_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBinaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsBinaryExpression {
	pub fn left(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(
			&self.syntax,
			&[
				T ! [<],
				T ! [>],
				T ! [<=],
				T ! [>=],
				T ! [==],
				T ! [===],
				T ! [!=],
				T ! [!==],
				T ! [+],
				T ! [-],
				T ! [*],
				T ! [/],
				T ! [%],
				T ! [**],
				T ! [<<],
				T ! [>>],
				T ! [>>>],
				T ! [&],
				T ! [|],
				T ! [^],
				T![in],
				T![instanceof],
			],
		)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBindingPatternWithDefault {
	pub(crate) syntax: SyntaxNode,
}
impl JsBindingPatternWithDefault {
	pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn default(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBlockStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsBlockStatement {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn statements(&self) -> JsStatementList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBooleanLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsBooleanLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![true], T![false]])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBreakStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsBreakStatement {
	pub fn break_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![break])
	}
	pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ident]) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCallArguments {
	pub(crate) syntax: SyntaxNode,
}
impl JsCallArguments {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn args(&self) -> JsCallArgumentList { support::list(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCaseClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsCaseClause {
	pub fn case_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![case])
	}
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn consequent(&self) -> JsStatementList { support::list(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCatchClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsCatchClause {
	pub fn catch_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![catch])
	}
	pub fn declaration(&self) -> Option<JsCatchDeclaration> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsBlockStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsCatchDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsCatchDeclaration {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn binding(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsClassDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsClassDeclaration {
	pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![class])
	}
	pub fn id(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn implements_clause(&self) -> Option<TsImplementsClause> { support::node(&self.syntax) }
	pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> JsClassMemberList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsClassExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsClassExpression {
	pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![class])
	}
	pub fn id(&self) -> Option<JsAnyBinding> { support::node(&self.syntax) }
	pub fn extends_clause(&self) -> Option<JsExtendsClause> { support::node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> JsClassMemberList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberAssignment {
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberExpression {
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn optional_chain_token_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?.])
	}
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsComputedMemberName {
	pub(crate) syntax: SyntaxNode,
}
impl JsComputedMemberName {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConditionalExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsConditionalExpression {
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConstructorClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsConstructorClassMember {
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn name(&self) -> SyntaxResult<JsLiteralMemberName> { support::required_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsConstructorParameters> {
		support::required_node(&self.syntax)
	}
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsConstructorParameters {
	pub(crate) syntax: SyntaxNode,
}
impl JsConstructorParameters {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn parameters(&self) -> JsConstructorParameterList { support::list(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsContinueStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsContinueStatement {
	pub fn continue_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![continue])
	}
	pub fn label_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ident]) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDebuggerStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsDebuggerStatement {
	pub fn debugger_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![debugger])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDefaultClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsDefaultClause {
	pub fn default_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![default])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn consequent(&self) -> JsStatementList { support::list(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDefaultImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsDefaultImportSpecifier {
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn trailing_comma_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [,])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDirective {
	pub(crate) syntax: SyntaxNode,
}
impl JsDirective {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsDoWhileStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsDoWhileStatement {
	pub fn do_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![do])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
	pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![while])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsElseClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsElseClause {
	pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![else])
	}
	pub fn alternate(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsEmptyClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsEmptyClassMember {
	pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [;])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsEmptyStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsEmptyStatement {
	pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [;])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExpressionStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsExpressionStatement {
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsExtendsClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsExtendsClause {
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![extends])
	}
	pub fn super_class(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFinallyClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsFinallyClause {
	pub fn finally_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![finally])
	}
	pub fn body(&self) -> SyntaxResult<JsBlockStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsForInStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsForInStatement {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![for])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn initializer(&self) -> SyntaxResult<JsAnyForInOrOfInitializer> {
		support::required_node(&self.syntax)
	}
	pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![in])
	}
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsForOfStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsForOfStatement {
	pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![for])
	}
	pub fn await_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![await]) }
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn initializer(&self) -> SyntaxResult<JsAnyForInOrOfInitializer> {
		support::required_node(&self.syntax)
	}
	pub fn of_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![of])
	}
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsForVariableDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsForVariableDeclaration {
	pub fn kind_token(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![var], T![let], T![const]])
	}
	pub fn declaration(&self) -> SyntaxResult<JsVariableDeclaration> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionBody {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionBody {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn directives(&self) -> JsDirectiveList { support::list(&self.syntax) }
	pub fn statements(&self) -> JsStatementList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionDeclaration {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn id(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn type_parameters(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsFunctionExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsFunctionExpression {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![function])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn id(&self) -> Option<JsAnyBinding> { support::node(&self.syntax) }
	pub fn type_parameters(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsGetterClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsGetterClassMember {
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![abstract])
	}
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![get])
	}
	pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsGetterObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsGetterObjectMember {
	pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![get])
	}
	pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierAssignment {
	pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierBinding {
	pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierBinding {
	pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIdentifierExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsIdentifierExpression {
	pub fn name(&self) -> SyntaxResult<JsReferenceIdentifier> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsIfStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsIfStatement {
	pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![if])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn consequent(&self) -> SyntaxResult<JsAnyStatement> {
		support::required_node(&self.syntax)
	}
	pub fn else_clause(&self) -> Option<JsElseClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImport {
	pub(crate) syntax: SyntaxNode,
}
impl JsImport {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn import_clause(&self) -> SyntaxResult<AnyJsImportClause> {
		support::required_node(&self.syntax)
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportAssertion {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportAssertion {
	pub fn assert_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![assert])
	}
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn assertions(&self) -> JsImportAssertionEntryList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportAssertionEntry {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportAssertionEntry {
	pub fn key(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![ident], T![js_string_literal]])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportBareClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportBareClause {
	pub fn source(&self) -> SyntaxResult<JsModuleSource> { support::required_node(&self.syntax) }
	pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportCallExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportCallExpression {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportDefaultClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportDefaultClause {
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![from])
	}
	pub fn source(&self) -> SyntaxResult<JsModuleSource> { support::required_node(&self.syntax) }
	pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportNamedClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportNamedClause {
	pub fn default_specifier(&self) -> Option<JsDefaultImportSpecifier> {
		support::node(&self.syntax)
	}
	pub fn named_import(&self) -> SyntaxResult<JsAnyNamedImport> {
		support::required_node(&self.syntax)
	}
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![from])
	}
	pub fn source(&self) -> SyntaxResult<JsModuleSource> { support::required_node(&self.syntax) }
	pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsImportNamespaceClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsImportNamespaceClause {
	pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [*])
	}
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![as])
	}
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![from])
	}
	pub fn source(&self) -> SyntaxResult<JsModuleSource> { support::required_node(&self.syntax) }
	pub fn assertion(&self) -> Option<JsImportAssertion> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsInitializerClause {
	pub(crate) syntax: SyntaxNode,
}
impl JsInitializerClause {
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLabeledStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsLabeledStatement {
	pub fn label_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLiteralExportName {
	pub(crate) syntax: SyntaxNode,
}
impl JsLiteralExportName {
	pub fn value(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![ident], T![js_string_literal]])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLiteralMemberName {
	pub(crate) syntax: SyntaxNode,
}
impl JsLiteralMemberName {
	pub fn value(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(
			&self.syntax,
			&[T![ident], T![js_string_literal], T![js_number_literal]],
		)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsLogicalExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsLogicalExpression {
	pub fn left(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T ! [??], T ! [||], T ! [&&]])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsMethodClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsMethodClassMember {
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![abstract])
	}
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn type_parameters(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsMethodObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsMethodObjectMember {
	pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn type_params(&self) -> Option<TsTypeParams> { support::node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn return_type(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsModifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsModifier {
	pub fn declare_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![declare]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsModule {
	pub(crate) syntax: SyntaxNode,
}
impl JsModule {
	pub fn interpreter_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![js_shebang])
	}
	pub fn directives(&self) -> JsDirectiveList { support::list(&self.syntax) }
	pub fn items(&self) -> JsModuleItemList { support::list(&self.syntax) }
	pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![EOF])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsModuleSource {
	pub(crate) syntax: SyntaxNode,
}
impl JsModuleSource {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsName {
	pub(crate) syntax: SyntaxNode,
}
impl JsName {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamedImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsNamedImportSpecifier {
	pub fn name(&self) -> SyntaxResult<JsLiteralExportName> { support::required_node(&self.syntax) }
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![as])
	}
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamedImportSpecifiers {
	pub(crate) syntax: SyntaxNode,
}
impl JsNamedImportSpecifiers {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn specifiers(&self) -> JsNamedImportSpecifierList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNamespaceImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsNamespaceImportSpecifier {
	pub fn star_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [*])
	}
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![as])
	}
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNullLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsNullLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![null])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsNumberLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsNumberLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_number_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPattern {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn properties(&self) -> JsObjectAssignmentPatternPropertyList {
		support::list(&self.syntax)
	}
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternProperty {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternProperty {
	pub fn member(&self) -> SyntaxResult<JsName> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn pattern(&self) -> SyntaxResult<JsAnyAssignmentPattern> {
		support::required_node(&self.syntax)
	}
	pub fn init(&self) -> Option<JsInitializerClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternRest {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternRest {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn target(&self) -> SyntaxResult<JsAnyAssignment> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectAssignmentPatternShorthandProperty {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectAssignmentPatternShorthandProperty {
	pub fn identifier(&self) -> SyntaxResult<JsAnyAssignment> {
		support::required_node(&self.syntax)
	}
	pub fn init(&self) -> Option<JsInitializerClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPattern {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPattern {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn properties(&self) -> JsObjectBindingPatternPropertyList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternProperty {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternProperty {
	pub fn member(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn pattern(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn init(&self) -> Option<JsInitializerClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternRest {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternRest {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn binding(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectBindingPatternShorthandProperty {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectBindingPatternShorthandProperty {
	pub fn identifier(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn init(&self) -> Option<JsInitializerClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsObjectExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsObjectExpression {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> JsObjectMemberList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParameters {
	pub(crate) syntax: SyntaxNode,
}
impl JsParameters {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn items(&self) -> JsParameterList { support::list(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParenthesizedAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsParenthesizedAssignment {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn assignment(&self) -> SyntaxResult<JsAnyAssignment> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsParenthesizedExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsParenthesizedExpression {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn expression(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPostUpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsPostUpdateExpression {
	pub fn operand(&self) -> SyntaxResult<JsAnyAssignment> { support::required_node(&self.syntax) }
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T ! [++], T ! [--]])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPreUpdateExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsPreUpdateExpression {
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T ! [++], T ! [--]])
	}
	pub fn operand(&self) -> SyntaxResult<JsAnyAssignment> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPrivateClassMemberName {
	pub(crate) syntax: SyntaxNode,
}
impl JsPrivateClassMemberName {
	pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [#])
	}
	pub fn id_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPrivateName {
	pub(crate) syntax: SyntaxNode,
}
impl JsPrivateName {
	pub fn hash_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [#])
	}
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsPropertyClassMember {
	pub fn modifiers(&self) -> Option<JsAnyModifier> { support::node(&self.syntax) }
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![abstract])
	}
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?])
	}
	pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
	pub fn ty(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn value(&self) -> Option<JsInitializerClause> { support::node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsPropertyObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsPropertyObjectMember {
	pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsReferenceIdentifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsReferenceIdentifier {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![ident])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsRegexLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsRegexLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_regex_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsRestParameter {
	pub(crate) syntax: SyntaxNode,
}
impl JsRestParameter {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn binding(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsReturnStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsReturnStatement {
	pub fn return_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![return])
	}
	pub fn argument(&self) -> Option<JsAnyExpression> { support::node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsScript {
	pub(crate) syntax: SyntaxNode,
}
impl JsScript {
	pub fn interpreter_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![js_shebang])
	}
	pub fn directives(&self) -> JsDirectiveList { support::list(&self.syntax) }
	pub fn statements(&self) -> JsStatementList { support::list(&self.syntax) }
	pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![EOF])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSequenceExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsSequenceExpression {
	pub fn left(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [,])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSetterClassMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsSetterClassMember {
	pub fn access_modifier(&self) -> Option<TsAccessibility> { support::node(&self.syntax) }
	pub fn abstract_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![abstract])
	}
	pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
	pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![set])
	}
	pub fn name(&self) -> SyntaxResult<JsAnyClassMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn parameter(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSetterObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsSetterObjectMember {
	pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![set])
	}
	pub fn name(&self) -> SyntaxResult<JsAnyObjectMemberName> {
		support::required_node(&self.syntax)
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn parameter(&self) -> SyntaxResult<JsAnyBindingPattern> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsFunctionBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsShorthandNamedImportSpecifier {
	pub(crate) syntax: SyntaxNode,
}
impl JsShorthandNamedImportSpecifier {
	pub fn local_name(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsShorthandPropertyObjectMember {
	pub(crate) syntax: SyntaxNode,
}
impl JsShorthandPropertyObjectMember {
	pub fn name(&self) -> SyntaxResult<JsReferenceIdentifier> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSpread {
	pub(crate) syntax: SyntaxNode,
}
impl JsSpread {
	pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [...])
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStaticMemberAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl JsStaticMemberAssignment {
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [.])
	}
	pub fn member(&self) -> SyntaxResult<JsAnyName> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStaticMemberExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsStaticMemberExpression {
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T ! [.], T ! [?.]])
	}
	pub fn member(&self) -> SyntaxResult<JsAnyName> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStringLiteralExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsStringLiteralExpression {
	pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSuperExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsSuperExpression {
	pub fn super_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![super])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsSwitchStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsSwitchStatement {
	pub fn switch_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![switch])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn discriminant(&self) -> SyntaxResult<JsAnyExpression> {
		support::required_node(&self.syntax)
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn cases(&self) -> JsSwitchCaseList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsThisExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsThisExpression {
	pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![this])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsThrowStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsThrowStatement {
	pub fn throw_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![throw])
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTryFinallyStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsTryFinallyStatement {
	pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![try])
	}
	pub fn body(&self) -> SyntaxResult<JsBlockStatement> { support::required_node(&self.syntax) }
	pub fn catch_clause(&self) -> Option<JsCatchClause> { support::node(&self.syntax) }
	pub fn finally_clause(&self) -> SyntaxResult<JsFinallyClause> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsTryStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsTryStatement {
	pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![try])
	}
	pub fn body(&self) -> SyntaxResult<JsBlockStatement> { support::required_node(&self.syntax) }
	pub fn catch_clause(&self) -> SyntaxResult<JsCatchClause> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnaryExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsUnaryExpression {
	pub fn operator(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(
			&self.syntax,
			&[
				T![delete],
				T![void],
				T![typeof],
				T ! [+],
				T ! [-],
				T ! [~],
				T![!],
			],
		)
	}
	pub fn argument(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclaration {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclaration {
	pub fn id(&self) -> SyntaxResult<JsAnyBindingPattern> { support::required_node(&self.syntax) }
	pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
	pub fn type_annotation(&self) -> Option<TsTypeAnnotation> { support::node(&self.syntax) }
	pub fn initializer(&self) -> Option<JsInitializerClause> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableDeclarations {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableDeclarations {
	pub fn kind_token(&self) -> SyntaxResult<SyntaxToken> {
		support::find_required_token(&self.syntax, &[T![var], T![const], T![let]])
	}
	pub fn items(&self) -> JsVariableDeclarationList { support::list(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsVariableStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsVariableStatement {
	pub fn declarations(&self) -> SyntaxResult<JsVariableDeclarations> {
		support::required_node(&self.syntax)
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsWhileStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsWhileStatement {
	pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![while])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn test(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsWithStatement {
	pub(crate) syntax: SyntaxNode,
}
impl JsWithStatement {
	pub fn with_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![with])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
	pub fn body(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsYieldExpression {
	pub(crate) syntax: SyntaxNode,
}
impl JsYieldExpression {
	pub fn yield_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![yield])
	}
	pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [*]) }
	pub fn argument(&self) -> Option<JsAnyExpression> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NewExpr {
	pub(crate) syntax: SyntaxNode,
}
impl NewExpr {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![new])
	}
	pub fn object(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn type_args(&self) -> Option<TsTypeArgs> { support::node(&self.syntax) }
	pub fn arguments(&self) -> SyntaxResult<JsCallArguments> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NewTarget {
	pub(crate) syntax: SyntaxNode,
}
impl NewTarget {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![new])
	}
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [.])
	}
	pub fn target_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![target])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Specifier {
	pub(crate) syntax: SyntaxNode,
}
impl Specifier {
	pub fn name(&self) -> SyntaxResult<JsName> { support::required_node(&self.syntax) }
	pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Template {
	pub(crate) syntax: SyntaxNode,
}
impl Template {
	pub fn backtick_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['`'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsAccessibility {
	pub(crate) syntax: SyntaxNode,
}
impl TsAccessibility {
	pub fn private_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![private])
	}
	pub fn readonly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![readonly])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsAny {
	pub(crate) syntax: SyntaxNode,
}
impl TsAny {
	pub fn any_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![any])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsArray {
	pub(crate) syntax: SyntaxNode,
}
impl TsArray {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsAssertion {
	pub(crate) syntax: SyntaxNode,
}
impl TsAssertion {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [<])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [>])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsBigint {
	pub(crate) syntax: SyntaxNode,
}
impl TsBigint {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsBoolean {
	pub(crate) syntax: SyntaxNode,
}
impl TsBoolean {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsCallSignatureDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsCallSignatureDecl {
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConditionalType {
	pub(crate) syntax: SyntaxNode,
}
impl TsConditionalType {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn extends(&self) -> SyntaxResult<TsExtends> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstAssertion {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstAssertion {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [<])
	}
	pub fn const_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![const])
	}
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [>])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstraint {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstraint {
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![extends])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstructSignatureDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructSignatureDecl {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![new])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [:]) }
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstructorParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructorParam {
	pub fn readonly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![readonly])
	}
	pub fn pat(&self) -> SyntaxResult<JsAnyBindingPattern> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsConstructorType {
	pub(crate) syntax: SyntaxNode,
}
impl TsConstructorType {
	pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![new])
	}
	pub fn params(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> Option<TsType> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsDefault {
	pub(crate) syntax: SyntaxNode,
}
impl TsDefault {
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsEnum {
	pub(crate) syntax: SyntaxNode,
}
impl TsEnum {
	pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
	pub fn enum_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![enum])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> TsEnumMemberList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsEnumMember {
	pub(crate) syntax: SyntaxNode,
}
impl TsEnumMember {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn value(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExportAssignment {
	pub(crate) syntax: SyntaxNode,
}
impl TsExportAssignment {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExprWithTypeArgs {
	pub(crate) syntax: SyntaxNode,
}
impl TsExprWithTypeArgs {
	pub fn item(&self) -> SyntaxResult<TsEntityName> { support::required_node(&self.syntax) }
	pub fn type_params(&self) -> SyntaxResult<TsTypeArgs> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExtends {
	pub(crate) syntax: SyntaxNode,
}
impl TsExtends {
	pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![extends])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsExternalModuleRef {
	pub(crate) syntax: SyntaxNode,
}
impl TsExternalModuleRef {
	pub fn require_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![require])
	}
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn module_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![js_string_literal])
	}
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsFnType {
	pub(crate) syntax: SyntaxNode,
}
impl TsFnType {
	pub fn params(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn fat_arrow_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=>])
	}
	pub fn return_type(&self) -> Option<TsType> { support::node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImplementsClause {
	pub(crate) syntax: SyntaxNode,
}
impl TsImplementsClause {
	pub fn implements_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![implements])
	}
	pub fn interfaces(&self) -> TsTypeList { support::list(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImport {
	pub(crate) syntax: SyntaxNode,
}
impl TsImport {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn type_args(&self) -> SyntaxResult<TsTypeArgs> { support::required_node(&self.syntax) }
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn qualifier(&self) -> SyntaxResult<TsEntityName> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsImportEqualsDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsImportEqualsDecl {
	pub fn import_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![import])
	}
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn module(&self) -> SyntaxResult<TsModuleRef> { support::required_node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIndexSignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsIndexSignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![readonly])
	}
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn pat(&self) -> SyntaxResult<JsAnyBinding> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIndexedArray {
	pub(crate) syntax: SyntaxNode,
}
impl TsIndexedArray {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsInfer {
	pub(crate) syntax: SyntaxNode,
}
impl TsInfer {
	pub fn infer_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![infer])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsInterfaceDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsInterfaceDecl {
	pub fn declare_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![declare]) }
	pub fn interface_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![interface])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn extends_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![extends]) }
	pub fn extends(&self) -> Option<TsExprWithTypeArgs> { support::node(&self.syntax) }
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> SyntaxResult<TsTypeElement> { support::required_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsIntersection {
	pub(crate) syntax: SyntaxNode,
}
impl TsIntersection {
	pub fn types(&self) -> TsTypeList { support::list(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsLiteral {
	pub(crate) syntax: SyntaxNode,
}
impl TsLiteral {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedType {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedType {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn readonly_modifier(&self) -> Option<TsMappedTypeReadonly> { support::node(&self.syntax) }
	pub fn minus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [-]) }
	pub fn plus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [+]) }
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?])
	}
	pub fn param(&self) -> SyntaxResult<TsMappedTypeParam> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeParam {
	pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
	pub fn name(&self) -> Option<TsTypeName> { support::node(&self.syntax) }
	pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
	pub fn ident(&self) -> Option<Ident> { support::node(&self.syntax) }
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMappedTypeReadonly {
	pub(crate) syntax: SyntaxNode,
}
impl TsMappedTypeReadonly {
	pub fn minus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [-]) }
	pub fn plus_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [+]) }
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![readonly])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsMethodSignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsMethodSignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![readonly])
	}
	pub fn key(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn parameters(&self) -> SyntaxResult<JsParameters> { support::required_node(&self.syntax) }
	pub fn question_mark_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn return_type(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsModuleBlock {
	pub(crate) syntax: SyntaxNode,
}
impl TsModuleBlock {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn items(&self) -> SyntaxResult<JsAnyStatement> { support::required_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsModuleDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsModuleDecl {
	pub fn declare_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![declare])
	}
	pub fn global_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![global]) }
	pub fn module_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![module])
	}
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn body(&self) -> SyntaxResult<TsNamespaceBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNamespaceDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsNamespaceDecl {
	pub fn declare_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![declare])
	}
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [.]) }
	pub fn body(&self) -> SyntaxResult<TsNamespaceBody> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNamespaceExportDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsNamespaceExportDecl {
	pub fn export_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![export])
	}
	pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![as])
	}
	pub fn namespace_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![namespace])
	}
	pub fn ident(&self) -> Option<Ident> { support::node(&self.syntax) }
	pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [;]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNever {
	pub(crate) syntax: SyntaxNode,
}
impl TsNever {
	pub fn never_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![never])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNonNull {
	pub(crate) syntax: SyntaxNode,
}
impl TsNonNull {
	pub fn expr(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn excl_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![!])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNull {
	pub(crate) syntax: SyntaxNode,
}
impl TsNull {
	pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![null])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsNumber {
	pub(crate) syntax: SyntaxNode,
}
impl TsNumber {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsObject {
	pub(crate) syntax: SyntaxNode,
}
impl TsObject {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsObjectType {
	pub(crate) syntax: SyntaxNode,
}
impl TsObjectType {
	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['{'])
	}
	pub fn members(&self) -> TsObjectMemberList { support::list(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsParen {
	pub(crate) syntax: SyntaxNode,
}
impl TsParen {
	pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['('])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![')'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsPredicate {
	pub(crate) syntax: SyntaxNode,
}
impl TsPredicate {
	pub fn lhs(&self) -> SyntaxResult<TsThisOrMore> { support::required_node(&self.syntax) }
	pub fn rhs(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsPropertySignature {
	pub(crate) syntax: SyntaxNode,
}
impl TsPropertySignature {
	pub fn readonly_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, T![readonly])
	}
	pub fn prop(&self) -> SyntaxResult<JsAnyExpression> { support::required_node(&self.syntax) }
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [?])
	}
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsQualifiedPath {
	pub(crate) syntax: SyntaxNode,
}
impl TsQualifiedPath {
	pub fn lhs(&self) -> SyntaxResult<TsEntityName> { support::required_node(&self.syntax) }
	pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [.])
	}
	pub fn rhs(&self) -> SyntaxResult<TsTypeName> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsString {
	pub(crate) syntax: SyntaxNode,
}
impl TsString {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsSymbol {
	pub(crate) syntax: SyntaxNode,
}
impl TsSymbol {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTemplate {
	pub(crate) syntax: SyntaxNode,
}
impl TsTemplate {
	pub fn elements(&self) -> SyntaxResult<TsTemplateElement> {
		support::required_node(&self.syntax)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTemplateElement {
	pub(crate) syntax: SyntaxNode,
}
impl TsTemplateElement {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['}'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsThis {
	pub(crate) syntax: SyntaxNode,
}
impl TsThis {
	pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![this])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTuple {
	pub(crate) syntax: SyntaxNode,
}
impl TsTuple {
	pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T!['['])
	}
	pub fn elements(&self) -> SyntaxResult<TsTupleElement> { support::required_node(&self.syntax) }
	pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![']'])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTupleElement {
	pub(crate) syntax: SyntaxNode,
}
impl TsTupleElement {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [?])
	}
	pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [...]) }
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeAliasDecl {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeAliasDecl {
	pub fn type_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![type])
	}
	pub fn type_params(&self) -> SyntaxResult<TsTypeParams> { support::required_node(&self.syntax) }
	pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [=])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeAnnotation {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeAnnotation {
	pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [:])
	}
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeArgs {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeArgs {
	pub fn l_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [<])
	}
	pub fn ts_type_arg_list(&self) -> TsTypeArgList { support::list(&self.syntax) }
	pub fn r_angle_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T ! [>])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeName {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeName {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeOperator {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeOperator {
	pub fn ty(&self) -> SyntaxResult<TsType> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParam {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeParam {
	pub fn ident(&self) -> SyntaxResult<Ident> { support::required_node(&self.syntax) }
	pub fn constraint(&self) -> SyntaxResult<TsConstraint> { support::required_node(&self.syntax) }
	pub fn default(&self) -> SyntaxResult<TsDefault> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeParams {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeParams {
	pub fn l_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [<]) }
	pub fn params(&self) -> SyntaxResult<TsTypeParam> { support::required_node(&self.syntax) }
	pub fn r_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T ! [>]) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsTypeRef {
	pub(crate) syntax: SyntaxNode,
}
impl TsTypeRef {
	pub fn name(&self) -> SyntaxResult<TsEntityName> { support::required_node(&self.syntax) }
	pub fn type_args(&self) -> SyntaxResult<TsTypeArgs> { support::required_node(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUndefined {
	pub(crate) syntax: SyntaxNode,
}
impl TsUndefined {
	pub fn undefined_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![undefined])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUnion {
	pub(crate) syntax: SyntaxNode,
}
impl TsUnion {
	pub fn types(&self) -> TsTypeList { support::list(&self.syntax) }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsUnknown {
	pub(crate) syntax: SyntaxNode,
}
impl TsUnknown {
	pub fn unknown_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![unknown])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TsVoid {
	pub(crate) syntax: SyntaxNode,
}
impl TsVoid {
	pub fn void_token(&self) -> SyntaxResult<SyntaxToken> {
		support::required_token(&self.syntax, T![void])
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AnyJsImportClause {
	JsImportBareClause(JsImportBareClause),
	JsImportDefaultClause(JsImportDefaultClause),
	JsImportNamedClause(JsImportNamedClause),
	JsImportNamespaceClause(JsImportNamespaceClause),
	JsName(JsName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum DefaultDecl {
	JsClassDeclaration(JsClassDeclaration),
	JsFunctionDeclaration(JsFunctionDeclaration),
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
pub enum JsAnyArrowFunctionBody {
	JsAnyExpression(JsAnyExpression),
	JsFunctionBody(JsFunctionBody),
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
pub enum JsAnyClassMember {
	JsConstructorClassMember(JsConstructorClassMember),
	JsEmptyClassMember(JsEmptyClassMember),
	JsGetterClassMember(JsGetterClassMember),
	JsMethodClassMember(JsMethodClassMember),
	JsPropertyClassMember(JsPropertyClassMember),
	JsSetterClassMember(JsSetterClassMember),
	JsUnknownMember(JsUnknownMember),
	TsIndexSignature(TsIndexSignature),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyClassMemberName {
	JsComputedMemberName(JsComputedMemberName),
	JsLiteralMemberName(JsLiteralMemberName),
	JsPrivateClassMemberName(JsPrivateClassMemberName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyConstructorParameter {
	JsAnyBindingPattern(JsAnyBindingPattern),
	JsBindingPatternWithDefault(JsBindingPatternWithDefault),
	TsConstructorParam(TsConstructorParam),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyExportDeclaration {
	ExportNamed(ExportNamed),
	JsClassDeclaration(JsClassDeclaration),
	JsFunctionDeclaration(JsFunctionDeclaration),
	JsVariableStatement(JsVariableStatement),
	TsEnum(TsEnum),
	TsInterfaceDecl(TsInterfaceDecl),
	TsModuleDecl(TsModuleDecl),
	TsNamespaceDecl(TsNamespaceDecl),
	TsTypeAliasDecl(TsTypeAliasDecl),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyExpression {
	CallExpr(CallExpr),
	ImportMeta(ImportMeta),
	JsAnyLiteralExpression(JsAnyLiteralExpression),
	JsArrayExpression(JsArrayExpression),
	JsArrowFunctionExpression(JsArrowFunctionExpression),
	JsAssignmentExpression(JsAssignmentExpression),
	JsAwaitExpression(JsAwaitExpression),
	JsBinaryExpression(JsBinaryExpression),
	JsClassExpression(JsClassExpression),
	JsComputedMemberExpression(JsComputedMemberExpression),
	JsConditionalExpression(JsConditionalExpression),
	JsFunctionExpression(JsFunctionExpression),
	JsIdentifierExpression(JsIdentifierExpression),
	JsImportCallExpression(JsImportCallExpression),
	JsLogicalExpression(JsLogicalExpression),
	JsObjectExpression(JsObjectExpression),
	JsParenthesizedExpression(JsParenthesizedExpression),
	JsPostUpdateExpression(JsPostUpdateExpression),
	JsPreUpdateExpression(JsPreUpdateExpression),
	JsSequenceExpression(JsSequenceExpression),
	JsStaticMemberExpression(JsStaticMemberExpression),
	JsSuperExpression(JsSuperExpression),
	JsThisExpression(JsThisExpression),
	JsUnaryExpression(JsUnaryExpression),
	JsUnknownExpression(JsUnknownExpression),
	JsYieldExpression(JsYieldExpression),
	NewExpr(NewExpr),
	NewTarget(NewTarget),
	Template(Template),
	TsAssertion(TsAssertion),
	TsConstAssertion(TsConstAssertion),
	TsNonNull(TsNonNull),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyForInOrOfInitializer {
	JsAnyAssignmentPattern(JsAnyAssignmentPattern),
	JsForVariableDeclaration(JsForVariableDeclaration),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyForInitializer {
	JsAnyExpression(JsAnyExpression),
	JsVariableDeclarations(JsVariableDeclarations),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyImportAssertionEntry {
	JsImportAssertionEntry(JsImportAssertionEntry),
	JsUnknownImportAssertionEntry(JsUnknownImportAssertionEntry),
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
pub enum JsAnyModifier {
	JsModifier(JsModifier),
	JsUnknownModifier(JsUnknownModifier),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyModuleItem {
	ExportDecl(ExportDecl),
	ExportDefaultDecl(ExportDefaultDecl),
	ExportDefaultExpr(ExportDefaultExpr),
	ExportWildcard(ExportWildcard),
	JsAnyStatement(JsAnyStatement),
	JsImport(JsImport),
	TsExportAssignment(TsExportAssignment),
	TsImportEqualsDecl(TsImportEqualsDecl),
	TsNamespaceExportDecl(TsNamespaceExportDecl),
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
	JsAnyBindingPattern(JsAnyBindingPattern),
	JsBindingPatternWithDefault(JsBindingPatternWithDefault),
	JsRestParameter(JsRestParameter),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyRoot {
	JsModule(JsModule),
	JsScript(JsScript),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnyStatement {
	ForStmt(ForStmt),
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
	TsEnum(TsEnum),
	TsInterfaceDecl(TsInterfaceDecl),
	TsModuleDecl(TsModuleDecl),
	TsNamespaceDecl(TsNamespaceDecl),
	TsTypeAliasDecl(TsTypeAliasDecl),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum JsAnySwitchClause {
	JsCaseClause(JsCaseClause),
	JsDefaultClause(JsDefaultClause),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsEntityName {
	TsQualifiedPath(TsQualifiedPath),
	TsTypeName(TsTypeName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsModuleRef {
	TsEntityName(TsEntityName),
	TsExternalModuleRef(TsExternalModuleRef),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsNamespaceBody {
	TsModuleBlock(TsModuleBlock),
	TsNamespaceDecl(TsNamespaceDecl),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsThisOrMore {
	TsThis(TsThis),
	TsTypeName(TsTypeName),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsType {
	TsAny(TsAny),
	TsArray(TsArray),
	TsBigint(TsBigint),
	TsBoolean(TsBoolean),
	TsConditionalType(TsConditionalType),
	TsConstructorType(TsConstructorType),
	TsFnType(TsFnType),
	TsImport(TsImport),
	TsIndexedArray(TsIndexedArray),
	TsInfer(TsInfer),
	TsIntersection(TsIntersection),
	TsLiteral(TsLiteral),
	TsMappedType(TsMappedType),
	TsNever(TsNever),
	TsNull(TsNull),
	TsNumber(TsNumber),
	TsObject(TsObject),
	TsObjectType(TsObjectType),
	TsParen(TsParen),
	TsPredicate(TsPredicate),
	TsString(TsString),
	TsSymbol(TsSymbol),
	TsTemplate(TsTemplate),
	TsThis(TsThis),
	TsTuple(TsTuple),
	TsTypeOperator(TsTypeOperator),
	TsTypeRef(TsTypeRef),
	TsUndefined(TsUndefined),
	TsUnion(TsUnion),
	TsUnknown(TsUnknown),
	TsVoid(TsVoid),
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TsTypeElement {
	TsCallSignatureDecl(TsCallSignatureDecl),
	TsConstructSignatureDecl(TsConstructSignatureDecl),
	TsIndexSignature(TsIndexSignature),
	TsMethodSignature(TsMethodSignature),
	TsPropertySignature(TsPropertySignature),
}
impl AstNode for CallExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == CALL_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for CallExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CallExpr")
			.field("callee", &support::DebugSyntaxResult(self.callee()))
			.field(
				"type_args",
				&support::DebugOptionalElement(self.type_args()),
			)
			.field("arguments", &support::DebugSyntaxResult(self.arguments()))
			.finish()
	}
}
impl AstNode for ExportDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportDecl")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field("decl", &support::DebugSyntaxResult(self.decl()))
			.finish()
	}
}
impl AstNode for ExportDefaultDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DEFAULT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportDefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportDefaultDecl")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"default_token",
				&support::DebugOptionalElement(self.default_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field("decl", &support::DebugSyntaxResult(self.decl()))
			.finish()
	}
}
impl AstNode for ExportDefaultExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_DEFAULT_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportDefaultExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportDefaultExpr")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field(
				"default_token",
				&support::DebugOptionalElement(self.default_token()),
			)
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.finish()
	}
}
impl AstNode for ExportNamed {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_NAMED }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportNamed {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportNamed")
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
				"from_token",
				&support::DebugOptionalElement(self.from_token()),
			)
			.field(
				"js_string_literal_token",
				&support::DebugOptionalElement(self.js_string_literal_token()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for ExportWildcard {
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_WILDCARD }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ExportWildcard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ExportWildcard")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field(
				"type_token",
				&support::DebugOptionalElement(self.type_token()),
			)
			.field("star_token", &support::DebugSyntaxResult(self.star_token()))
			.field("as_token", &support::DebugOptionalElement(self.as_token()))
			.field("ident", &support::DebugOptionalElement(self.ident()))
			.field("from_token", &support::DebugSyntaxResult(self.from_token()))
			.field(
				"source_token",
				&support::DebugSyntaxResult(self.source_token()),
			)
			.finish()
	}
}
impl AstNode for ForStmt {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForStmt")
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
			.field("cons", &support::DebugSyntaxResult(self.cons()))
			.finish()
	}
}
impl AstNode for ForStmtTest {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT_TEST }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForStmtTest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForStmtTest")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.finish()
	}
}
impl AstNode for ForStmtUpdate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_STMT_UPDATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for ForStmtUpdate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForStmtUpdate")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.finish()
	}
}
impl AstNode for Ident {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IDENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for Ident {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Ident")
			.field(
				"ident_token",
				&support::DebugSyntaxResult(self.ident_token()),
			)
			.finish()
	}
}
impl AstNode for ImportMeta {
	fn can_cast(kind: SyntaxKind) -> bool { kind == IMPORT_META }
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
impl AstNode for JsArrayAssignmentPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_PATTERN }
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
impl AstNode for JsArrayAssignmentPatternRestElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT }
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
impl AstNode for JsArrayBindingPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_BINDING_PATTERN }
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
impl AstNode for JsArrayBindingPatternRestElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_BINDING_PATTERN_REST_ELEMENT }
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
impl AstNode for JsArrayExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_EXPRESSION }
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
impl AstNode for JsArrayHole {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_HOLE }
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
impl AstNode for JsArrowFunctionExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARROW_FUNCTION_EXPRESSION }
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
			.field(
				"parameters",
				&support::DebugOptionalElement(self.parameters()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field(
				"fat_arrow_token",
				&support::DebugSyntaxResult(self.fat_arrow_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsAssignmentExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ASSIGNMENT_EXPRESSION }
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
impl AstNode for JsAssignmentWithDefault {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ASSIGNMENT_WITH_DEFAULT }
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
impl AstNode for JsAwaitExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_AWAIT_EXPRESSION }
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
impl AstNode for JsBigIntLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BIG_INT_LITERAL_EXPRESSION }
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
impl AstNode for JsBinaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BINARY_EXPRESSION }
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
impl AstNode for JsBindingPatternWithDefault {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BINDING_PATTERN_WITH_DEFAULT }
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
impl AstNode for JsBlockStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BLOCK_STATEMENT }
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
impl AstNode for JsBooleanLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BOOLEAN_LITERAL_EXPRESSION }
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
impl AstNode for JsBreakStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_BREAK_STATEMENT }
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
impl AstNode for JsCallArguments {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CALL_ARGUMENTS }
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
impl AstNode for JsCaseClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CASE_CLAUSE }
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
impl AstNode for JsCatchClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CATCH_CLAUSE }
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
impl AstNode for JsCatchDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CATCH_DECLARATION }
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
impl AstNode for JsClassDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CLASS_DECLARATION }
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
				"implements_clause",
				&support::DebugOptionalElement(self.implements_clause()),
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
impl AstNode for JsClassExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CLASS_EXPRESSION }
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
impl AstNode for JsComputedMemberAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_ASSIGNMENT }
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
impl AstNode for JsComputedMemberExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_EXPRESSION }
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
				"optional_chain_token_token",
				&support::DebugOptionalElement(self.optional_chain_token_token()),
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
impl AstNode for JsComputedMemberName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_COMPUTED_MEMBER_NAME }
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
impl AstNode for JsConditionalExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONDITIONAL_EXPRESSION }
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
impl AstNode for JsConstructorClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONSTRUCTOR_CLASS_MEMBER }
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
impl AstNode for JsConstructorParameters {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONSTRUCTOR_PARAMETERS }
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
impl AstNode for JsContinueStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONTINUE_STATEMENT }
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
impl AstNode for JsDebuggerStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEBUGGER_STATEMENT }
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
impl AstNode for JsDefaultClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEFAULT_CLAUSE }
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
impl AstNode for JsDefaultImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DEFAULT_IMPORT_SPECIFIER }
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
impl AstNode for JsDirective {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DIRECTIVE }
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
impl AstNode for JsDoWhileStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DO_WHILE_STATEMENT }
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
impl AstNode for JsElseClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ELSE_CLAUSE }
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
impl AstNode for JsEmptyClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EMPTY_CLASS_MEMBER }
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
impl AstNode for JsEmptyStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EMPTY_STATEMENT }
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
impl AstNode for JsExpressionStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXPRESSION_STATEMENT }
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
impl AstNode for JsExtendsClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_EXTENDS_CLAUSE }
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
			.finish()
	}
}
impl AstNode for JsFinallyClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FINALLY_CLAUSE }
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
impl AstNode for JsForInStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FOR_IN_STATEMENT }
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
impl AstNode for JsForOfStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FOR_OF_STATEMENT }
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
impl AstNode for JsForVariableDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FOR_VARIABLE_DECLARATION }
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
			.field(
				"declaration",
				&support::DebugSyntaxResult(self.declaration()),
			)
			.finish()
	}
}
impl AstNode for JsFunctionBody {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_BODY }
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
impl AstNode for JsFunctionDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_DECLARATION }
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
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsFunctionExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_FUNCTION_EXPRESSION }
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
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsGetterClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_GETTER_CLASS_MEMBER }
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
				"abstract_token",
				&support::DebugOptionalElement(self.abstract_token()),
			)
			.field(
				"static_token",
				&support::DebugOptionalElement(self.static_token()),
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
impl AstNode for JsGetterObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_GETTER_OBJECT_MEMBER }
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
impl AstNode for JsIdentifierAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IDENTIFIER_ASSIGNMENT }
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
impl AstNode for JsIdentifierBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IDENTIFIER_BINDING }
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
impl AstNode for JsIdentifierExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IDENTIFIER_EXPRESSION }
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
impl AstNode for JsIfStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IF_STATEMENT }
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
impl AstNode for JsImport {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT }
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
impl AstNode for JsImportAssertion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_ASSERTION }
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
impl AstNode for JsImportAssertionEntry {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_ASSERTION_ENTRY }
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
impl AstNode for JsImportBareClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_BARE_CLAUSE }
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
impl AstNode for JsImportCallExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_CALL_EXPRESSION }
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
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("argument", &support::DebugSyntaxResult(self.argument()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for JsImportDefaultClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_DEFAULT_CLAUSE }
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
impl AstNode for JsImportNamedClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_NAMED_CLAUSE }
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
impl AstNode for JsImportNamespaceClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_NAMESPACE_CLAUSE }
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
impl AstNode for JsInitializerClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_INITIALIZER_CLAUSE }
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
impl AstNode for JsLabeledStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LABELED_STATEMENT }
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
impl AstNode for JsLiteralExportName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LITERAL_EXPORT_NAME }
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
impl AstNode for JsLiteralMemberName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LITERAL_MEMBER_NAME }
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
impl AstNode for JsLogicalExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_LOGICAL_EXPRESSION }
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
impl AstNode for JsMethodClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_METHOD_CLASS_MEMBER }
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
				"type_parameters",
				&support::DebugOptionalElement(self.type_parameters()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsMethodObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_METHOD_OBJECT_MEMBER }
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
				"type_params",
				&support::DebugOptionalElement(self.type_params()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for JsModifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MODIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsModifier")
			.field(
				"declare_token",
				&support::DebugOptionalElement(self.declare_token()),
			)
			.finish()
	}
}
impl AstNode for JsModule {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MODULE }
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
impl AstNode for JsModuleSource {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MODULE_SOURCE }
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
impl AstNode for JsName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAME }
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
impl AstNode for JsNamedImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMED_IMPORT_SPECIFIER }
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
impl AstNode for JsNamedImportSpecifiers {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMED_IMPORT_SPECIFIERS }
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
impl AstNode for JsNamespaceImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMESPACE_IMPORT_SPECIFIER }
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
impl AstNode for JsNullLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NULL_LITERAL_EXPRESSION }
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
impl AstNode for JsNumberLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NUMBER_LITERAL_EXPRESSION }
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
impl AstNode for JsObjectAssignmentPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN }
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
impl AstNode for JsObjectAssignmentPatternProperty {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY }
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
impl AstNode for JsObjectAssignmentPatternRest {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN_REST }
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
impl AstNode for JsObjectAssignmentPatternShorthandProperty {
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl AstNode for JsObjectBindingPattern {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN }
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
impl AstNode for JsObjectBindingPatternProperty {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_PROPERTY }
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
impl AstNode for JsObjectBindingPatternRest {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_REST }
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
impl AstNode for JsObjectBindingPatternShorthandProperty {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY }
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
impl AstNode for JsObjectExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_EXPRESSION }
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
impl AstNode for JsParameters {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARAMETERS }
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
impl AstNode for JsParenthesizedAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARENTHESIZED_ASSIGNMENT }
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
impl AstNode for JsParenthesizedExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARENTHESIZED_EXPRESSION }
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
impl AstNode for JsPostUpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_POST_UPDATE_EXPRESSION }
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
impl AstNode for JsPreUpdateExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRE_UPDATE_EXPRESSION }
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
impl AstNode for JsPrivateClassMemberName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRIVATE_CLASS_MEMBER_NAME }
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
impl AstNode for JsPrivateName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PRIVATE_NAME }
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
impl AstNode for JsPropertyClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PROPERTY_CLASS_MEMBER }
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
				"modifiers",
				&support::DebugOptionalElement(self.modifiers()),
			)
			.field(
				"access_modifier",
				&support::DebugOptionalElement(self.access_modifier()),
			)
			.field(
				"abstract_token",
				&support::DebugOptionalElement(self.abstract_token()),
			)
			.field(
				"static_token",
				&support::DebugOptionalElement(self.static_token()),
			)
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field(
				"question_mark_token",
				&support::DebugOptionalElement(self.question_mark_token()),
			)
			.field(
				"excl_token",
				&support::DebugOptionalElement(self.excl_token()),
			)
			.field("ty", &support::DebugOptionalElement(self.ty()))
			.field("value", &support::DebugOptionalElement(self.value()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsPropertyObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PROPERTY_OBJECT_MEMBER }
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
impl AstNode for JsReferenceIdentifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_REFERENCE_IDENTIFIER }
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
impl AstNode for JsRegexLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_REGEX_LITERAL_EXPRESSION }
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
impl AstNode for JsRestParameter {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_REST_PARAMETER }
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
			.finish()
	}
}
impl AstNode for JsReturnStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_RETURN_STATEMENT }
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
impl AstNode for JsScript {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SCRIPT }
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
impl AstNode for JsSequenceExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SEQUENCE_EXPRESSION }
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
impl AstNode for JsSetterClassMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SETTER_CLASS_MEMBER }
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
				"abstract_token",
				&support::DebugOptionalElement(self.abstract_token()),
			)
			.field(
				"static_token",
				&support::DebugOptionalElement(self.static_token()),
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
impl AstNode for JsSetterObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SETTER_OBJECT_MEMBER }
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
impl AstNode for JsShorthandNamedImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SHORTHAND_NAMED_IMPORT_SPECIFIER }
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
impl AstNode for JsShorthandPropertyObjectMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SHORTHAND_PROPERTY_OBJECT_MEMBER }
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
impl AstNode for JsSpread {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SPREAD }
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
impl AstNode for JsStaticMemberAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STATIC_MEMBER_ASSIGNMENT }
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
impl AstNode for JsStaticMemberExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STATIC_MEMBER_EXPRESSION }
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
impl AstNode for JsStringLiteralExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STRING_LITERAL_EXPRESSION }
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
impl AstNode for JsSuperExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SUPER_EXPRESSION }
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
impl AstNode for JsSwitchStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SWITCH_STATEMENT }
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
impl AstNode for JsThisExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_THIS_EXPRESSION }
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
impl AstNode for JsThrowStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_THROW_STATEMENT }
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
impl AstNode for JsTryFinallyStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_TRY_FINALLY_STATEMENT }
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
impl AstNode for JsTryStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_TRY_STATEMENT }
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
impl AstNode for JsUnaryExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNARY_EXPRESSION }
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
impl AstNode for JsVariableDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATION }
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
			.field("id", &support::DebugSyntaxResult(self.id()))
			.field(
				"excl_token",
				&support::DebugOptionalElement(self.excl_token()),
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
impl AstNode for JsVariableDeclarations {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATIONS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsVariableDeclarations {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsVariableDeclarations")
			.field("kind_token", &support::DebugSyntaxResult(self.kind_token()))
			.field("items", &self.items())
			.finish()
	}
}
impl AstNode for JsVariableStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_STATEMENT }
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
				"declarations",
				&support::DebugSyntaxResult(self.declarations()),
			)
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for JsWhileStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_WHILE_STATEMENT }
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
impl AstNode for JsWithStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_WITH_STATEMENT }
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
impl AstNode for JsYieldExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_YIELD_EXPRESSION }
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
			.field(
				"star_token",
				&support::DebugOptionalElement(self.star_token()),
			)
			.field("argument", &support::DebugOptionalElement(self.argument()))
			.finish()
	}
}
impl AstNode for NewExpr {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NEW_EXPR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for NewExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("NewExpr")
			.field("new_token", &support::DebugSyntaxResult(self.new_token()))
			.field("object", &support::DebugSyntaxResult(self.object()))
			.field(
				"type_args",
				&support::DebugOptionalElement(self.type_args()),
			)
			.field("arguments", &support::DebugSyntaxResult(self.arguments()))
			.finish()
	}
}
impl AstNode for NewTarget {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NEW_TARGET }
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
impl AstNode for Specifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == SPECIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for Specifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Specifier")
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field("as_token", &support::DebugOptionalElement(self.as_token()))
			.field("alias", &support::DebugOptionalElement(self.alias()))
			.finish()
	}
}
impl AstNode for Template {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TEMPLATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for Template {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Template")
			.field(
				"backtick_token",
				&support::DebugSyntaxResult(self.backtick_token()),
			)
			.finish()
	}
}
impl AstNode for TsAccessibility {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ACCESSIBILITY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsAccessibility {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsAccessibility")
			.field(
				"private_token",
				&support::DebugSyntaxResult(self.private_token()),
			)
			.field(
				"readonly_token",
				&support::DebugSyntaxResult(self.readonly_token()),
			)
			.finish()
	}
}
impl AstNode for TsAny {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ANY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsAny {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsAny")
			.field("any_token", &support::DebugSyntaxResult(self.any_token()))
			.finish()
	}
}
impl AstNode for TsArray {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ARRAY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsArray")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for TsAssertion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ASSERTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsAssertion")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"l_angle_token",
				&support::DebugSyntaxResult(self.l_angle_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_angle_token",
				&support::DebugSyntaxResult(self.r_angle_token()),
			)
			.finish()
	}
}
impl AstNode for TsBigint {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_BIGINT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsBigint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsBigint")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsBoolean {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_BOOLEAN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsBoolean {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsBoolean")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsCallSignatureDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CALL_SIGNATURE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsCallSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsCallSignatureDecl")
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"return_type",
				&support::DebugSyntaxResult(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsConditionalType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONDITIONAL_TYPE }
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
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"question_mark_token",
				&support::DebugSyntaxResult(self.question_mark_token()),
			)
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("extends", &support::DebugSyntaxResult(self.extends()))
			.finish()
	}
}
impl AstNode for TsConstAssertion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONST_ASSERTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstAssertion")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"l_angle_token",
				&support::DebugSyntaxResult(self.l_angle_token()),
			)
			.field(
				"const_token",
				&support::DebugSyntaxResult(self.const_token()),
			)
			.field(
				"r_angle_token",
				&support::DebugSyntaxResult(self.r_angle_token()),
			)
			.finish()
	}
}
impl AstNode for TsConstraint {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRAINT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstraint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstraint")
			.field(
				"extends_token",
				&support::DebugSyntaxResult(self.extends_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsConstructSignatureDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCT_SIGNATURE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstructSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstructSignatureDecl")
			.field("new_token", &support::DebugSyntaxResult(self.new_token()))
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"colon_token",
				&support::DebugOptionalElement(self.colon_token()),
			)
			.field(
				"return_type",
				&support::DebugSyntaxResult(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsConstructorParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCTOR_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsConstructorParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsConstructorParam")
			.field(
				"readonly_token",
				&support::DebugSyntaxResult(self.readonly_token()),
			)
			.field("pat", &support::DebugSyntaxResult(self.pat()))
			.finish()
	}
}
impl AstNode for TsConstructorType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_CONSTRUCTOR_TYPE }
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
			.field("new_token", &support::DebugSyntaxResult(self.new_token()))
			.field("params", &support::DebugSyntaxResult(self.params()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsDefault {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_DEFAULT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsDefault")
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsEnum {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ENUM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsEnum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsEnum")
			.field(
				"const_token",
				&support::DebugOptionalElement(self.const_token()),
			)
			.field("enum_token", &support::DebugSyntaxResult(self.enum_token()))
			.field("ident", &support::DebugSyntaxResult(self.ident()))
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
impl AstNode for TsEnumMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ENUM_MEMBER }
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
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("value", &support::DebugSyntaxResult(self.value()))
			.finish()
	}
}
impl AstNode for TsExportAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXPORT_ASSIGNMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExportAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsExportAssignment")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for TsExprWithTypeArgs {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXPR_WITH_TYPE_ARGS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExprWithTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsExprWithTypeArgs")
			.field("item", &support::DebugSyntaxResult(self.item()))
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.finish()
	}
}
impl AstNode for TsExtends {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXTENDS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsExtends {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsExtends")
			.field(
				"extends_token",
				&support::DebugSyntaxResult(self.extends_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsExternalModuleRef {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_EXTERNAL_MODULE_REF }
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
impl AstNode for TsFnType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_FN_TYPE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsFnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsFnType")
			.field("params", &support::DebugSyntaxResult(self.params()))
			.field(
				"fat_arrow_token",
				&support::DebugSyntaxResult(self.fat_arrow_token()),
			)
			.field(
				"return_type",
				&support::DebugOptionalElement(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsImplementsClause {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_IMPLEMENTS_CLAUSE }
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
			.field("interfaces", &self.interfaces())
			.finish()
	}
}
impl AstNode for TsImport {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_IMPORT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsImport")
			.field(
				"import_token",
				&support::DebugSyntaxResult(self.import_token()),
			)
			.field("type_args", &support::DebugSyntaxResult(self.type_args()))
			.field(
				"dot_token",
				&support::DebugOptionalElement(self.dot_token()),
			)
			.field(
				"l_paren_token",
				&support::DebugSyntaxResult(self.l_paren_token()),
			)
			.field("qualifier", &support::DebugSyntaxResult(self.qualifier()))
			.field(
				"r_paren_token",
				&support::DebugSyntaxResult(self.r_paren_token()),
			)
			.finish()
	}
}
impl AstNode for TsImportEqualsDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_IMPORT_EQUALS_DECL }
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
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("module", &support::DebugSyntaxResult(self.module()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for TsIndexSignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INDEX_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIndexSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsIndexSignature")
			.field(
				"readonly_token",
				&support::DebugOptionalElement(self.readonly_token()),
			)
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("pat", &support::DebugSyntaxResult(self.pat()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for TsIndexedArray {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INDEXED_ARRAY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIndexedArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsIndexedArray")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for TsInfer {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INFER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsInfer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsInfer")
			.field(
				"infer_token",
				&support::DebugSyntaxResult(self.infer_token()),
			)
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsInterfaceDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INTERFACE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsInterfaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsInterfaceDecl")
			.field(
				"declare_token",
				&support::DebugOptionalElement(self.declare_token()),
			)
			.field(
				"interface_token",
				&support::DebugSyntaxResult(self.interface_token()),
			)
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field(
				"extends_token",
				&support::DebugOptionalElement(self.extends_token()),
			)
			.field("extends", &support::DebugOptionalElement(self.extends()))
			.field(
				"l_curly_token",
				&support::DebugSyntaxResult(self.l_curly_token()),
			)
			.field("members", &support::DebugSyntaxResult(self.members()))
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for TsIntersection {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_INTERSECTION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsIntersection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsIntersection")
			.field("types", &self.types())
			.finish()
	}
}
impl AstNode for TsLiteral {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_LITERAL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsLiteral")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsMappedType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE }
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
				"minus_token",
				&support::DebugOptionalElement(self.minus_token()),
			)
			.field(
				"plus_token",
				&support::DebugOptionalElement(self.plus_token()),
			)
			.field(
				"question_mark_token",
				&support::DebugOptionalElement(self.question_mark_token()),
			)
			.field("param", &support::DebugSyntaxResult(self.param()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
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
impl AstNode for TsMappedTypeParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsMappedTypeParam")
			.field(
				"l_brack_token",
				&support::DebugOptionalElement(self.l_brack_token()),
			)
			.field("name", &support::DebugOptionalElement(self.name()))
			.field(
				"r_brack_token",
				&support::DebugOptionalElement(self.r_brack_token()),
			)
			.field("ident", &support::DebugOptionalElement(self.ident()))
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsMappedTypeReadonly {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MAPPED_TYPE_READONLY }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMappedTypeReadonly {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsMappedTypeReadonly")
			.field(
				"minus_token",
				&support::DebugOptionalElement(self.minus_token()),
			)
			.field(
				"plus_token",
				&support::DebugOptionalElement(self.plus_token()),
			)
			.field(
				"readonly_token",
				&support::DebugOptionalElement(self.readonly_token()),
			)
			.finish()
	}
}
impl AstNode for TsMethodSignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_METHOD_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsMethodSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsMethodSignature")
			.field(
				"readonly_token",
				&support::DebugOptionalElement(self.readonly_token()),
			)
			.field("key", &support::DebugSyntaxResult(self.key()))
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field("parameters", &support::DebugSyntaxResult(self.parameters()))
			.field(
				"question_mark_token",
				&support::DebugOptionalElement(self.question_mark_token()),
			)
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"return_type",
				&support::DebugSyntaxResult(self.return_type()),
			)
			.finish()
	}
}
impl AstNode for TsModuleBlock {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MODULE_BLOCK }
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
			.field("items", &support::DebugSyntaxResult(self.items()))
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for TsModuleDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_MODULE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsModuleDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsModuleDecl")
			.field(
				"declare_token",
				&support::DebugSyntaxResult(self.declare_token()),
			)
			.field(
				"global_token",
				&support::DebugOptionalElement(self.global_token()),
			)
			.field(
				"module_token",
				&support::DebugSyntaxResult(self.module_token()),
			)
			.field(
				"dot_token",
				&support::DebugOptionalElement(self.dot_token()),
			)
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for TsNamespaceDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NAMESPACE_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNamespaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNamespaceDecl")
			.field(
				"declare_token",
				&support::DebugSyntaxResult(self.declare_token()),
			)
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"dot_token",
				&support::DebugOptionalElement(self.dot_token()),
			)
			.field("body", &support::DebugSyntaxResult(self.body()))
			.finish()
	}
}
impl AstNode for TsNamespaceExportDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NAMESPACE_EXPORT_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNamespaceExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNamespaceExportDecl")
			.field(
				"export_token",
				&support::DebugSyntaxResult(self.export_token()),
			)
			.field("as_token", &support::DebugSyntaxResult(self.as_token()))
			.field(
				"namespace_token",
				&support::DebugSyntaxResult(self.namespace_token()),
			)
			.field("ident", &support::DebugOptionalElement(self.ident()))
			.field(
				"semicolon_token",
				&support::DebugOptionalElement(self.semicolon_token()),
			)
			.finish()
	}
}
impl AstNode for TsNever {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NEVER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNever {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNever")
			.field(
				"never_token",
				&support::DebugSyntaxResult(self.never_token()),
			)
			.finish()
	}
}
impl AstNode for TsNonNull {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NON_NULL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNonNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNonNull")
			.field("expr", &support::DebugSyntaxResult(self.expr()))
			.field("excl_token", &support::DebugSyntaxResult(self.excl_token()))
			.finish()
	}
}
impl AstNode for TsNull {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NULL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNull")
			.field("null_token", &support::DebugSyntaxResult(self.null_token()))
			.finish()
	}
}
impl AstNode for TsNumber {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_NUMBER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsNumber")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsObject {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_OBJECT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsObject")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsObjectType {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_OBJECT_TYPE }
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
impl AstNode for TsParen {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PAREN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsParen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsParen")
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
impl AstNode for TsPredicate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PREDICATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsPredicate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsPredicate")
			.field("lhs", &support::DebugSyntaxResult(self.lhs()))
			.field("rhs", &support::DebugSyntaxResult(self.rhs()))
			.finish()
	}
}
impl AstNode for TsPropertySignature {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_PROPERTY_SIGNATURE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsPropertySignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsPropertySignature")
			.field(
				"readonly_token",
				&support::DebugOptionalElement(self.readonly_token()),
			)
			.field("prop", &support::DebugSyntaxResult(self.prop()))
			.field(
				"question_mark_token",
				&support::DebugSyntaxResult(self.question_mark_token()),
			)
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsQualifiedPath {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_QUALIFIED_PATH }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsQualifiedPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsQualifiedPath")
			.field("lhs", &support::DebugSyntaxResult(self.lhs()))
			.field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
			.field("rhs", &support::DebugSyntaxResult(self.rhs()))
			.finish()
	}
}
impl AstNode for TsString {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_STRING }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsString")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsSymbol {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_SYMBOL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsSymbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsSymbol")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsTemplate {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TEMPLATE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTemplate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTemplate")
			.field("elements", &support::DebugSyntaxResult(self.elements()))
			.finish()
	}
}
impl AstNode for TsTemplateElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TEMPLATE_ELEMENT }
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
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.field(
				"r_curly_token",
				&support::DebugSyntaxResult(self.r_curly_token()),
			)
			.finish()
	}
}
impl AstNode for TsThis {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_THIS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsThis {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsThis")
			.field("this_token", &support::DebugSyntaxResult(self.this_token()))
			.finish()
	}
}
impl AstNode for TsTuple {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TUPLE }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTuple")
			.field(
				"l_brack_token",
				&support::DebugSyntaxResult(self.l_brack_token()),
			)
			.field("elements", &support::DebugSyntaxResult(self.elements()))
			.field(
				"r_brack_token",
				&support::DebugSyntaxResult(self.r_brack_token()),
			)
			.finish()
	}
}
impl AstNode for TsTupleElement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TUPLE_ELEMENT }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTupleElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTupleElement")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field(
				"colon_token",
				&support::DebugSyntaxResult(self.colon_token()),
			)
			.field(
				"question_mark_token",
				&support::DebugSyntaxResult(self.question_mark_token()),
			)
			.field(
				"dotdotdot_token",
				&support::DebugOptionalElement(self.dotdotdot_token()),
			)
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsTypeAliasDecl {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ALIAS_DECL }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeAliasDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeAliasDecl")
			.field("type_token", &support::DebugSyntaxResult(self.type_token()))
			.field(
				"type_params",
				&support::DebugSyntaxResult(self.type_params()),
			)
			.field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsTypeAnnotation {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ANNOTATION }
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
impl AstNode for TsTypeArgs {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ARGS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeArgs")
			.field(
				"l_angle_token",
				&support::DebugSyntaxResult(self.l_angle_token()),
			)
			.field("ts_type_arg_list", &self.ts_type_arg_list())
			.field(
				"r_angle_token",
				&support::DebugSyntaxResult(self.r_angle_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeName {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_NAME }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeName")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.finish()
	}
}
impl AstNode for TsTypeOperator {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_OPERATOR }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeOperator")
			.field("ty", &support::DebugSyntaxResult(self.ty()))
			.finish()
	}
}
impl AstNode for TsTypeParam {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_PARAM }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeParam")
			.field("ident", &support::DebugSyntaxResult(self.ident()))
			.field("constraint", &support::DebugSyntaxResult(self.constraint()))
			.field("default", &support::DebugSyntaxResult(self.default()))
			.finish()
	}
}
impl AstNode for TsTypeParams {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_PARAMS }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeParams")
			.field(
				"l_angle_token",
				&support::DebugOptionalElement(self.l_angle_token()),
			)
			.field("params", &support::DebugSyntaxResult(self.params()))
			.field(
				"r_angle_token",
				&support::DebugOptionalElement(self.r_angle_token()),
			)
			.finish()
	}
}
impl AstNode for TsTypeRef {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_REF }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsTypeRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsTypeRef")
			.field("name", &support::DebugSyntaxResult(self.name()))
			.field("type_args", &support::DebugSyntaxResult(self.type_args()))
			.finish()
	}
}
impl AstNode for TsUndefined {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNDEFINED }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUndefined {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsUndefined")
			.field(
				"undefined_token",
				&support::DebugSyntaxResult(self.undefined_token()),
			)
			.finish()
	}
}
impl AstNode for TsUnion {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNION }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsUnion")
			.field("types", &self.types())
			.finish()
	}
}
impl AstNode for TsUnknown {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_UNKNOWN }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsUnknown {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsUnknown")
			.field(
				"unknown_token",
				&support::DebugSyntaxResult(self.unknown_token()),
			)
			.finish()
	}
}
impl AstNode for TsVoid {
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_VOID }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for TsVoid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("TsVoid")
			.field("void_token", &support::DebugSyntaxResult(self.void_token()))
			.finish()
	}
}
impl From<JsImportBareClause> for AnyJsImportClause {
	fn from(node: JsImportBareClause) -> AnyJsImportClause {
		AnyJsImportClause::JsImportBareClause(node)
	}
}
impl From<JsImportDefaultClause> for AnyJsImportClause {
	fn from(node: JsImportDefaultClause) -> AnyJsImportClause {
		AnyJsImportClause::JsImportDefaultClause(node)
	}
}
impl From<JsImportNamedClause> for AnyJsImportClause {
	fn from(node: JsImportNamedClause) -> AnyJsImportClause {
		AnyJsImportClause::JsImportNamedClause(node)
	}
}
impl From<JsImportNamespaceClause> for AnyJsImportClause {
	fn from(node: JsImportNamespaceClause) -> AnyJsImportClause {
		AnyJsImportClause::JsImportNamespaceClause(node)
	}
}
impl From<JsName> for AnyJsImportClause {
	fn from(node: JsName) -> AnyJsImportClause { AnyJsImportClause::JsName(node) }
}
impl AstNode for AnyJsImportClause {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_IMPORT_BARE_CLAUSE
				| JS_IMPORT_DEFAULT_CLAUSE
				| JS_IMPORT_NAMED_CLAUSE
				| JS_IMPORT_NAMESPACE_CLAUSE
				| JS_NAME
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_IMPORT_BARE_CLAUSE => {
				AnyJsImportClause::JsImportBareClause(JsImportBareClause { syntax })
			}
			JS_IMPORT_DEFAULT_CLAUSE => {
				AnyJsImportClause::JsImportDefaultClause(JsImportDefaultClause { syntax })
			}
			JS_IMPORT_NAMED_CLAUSE => {
				AnyJsImportClause::JsImportNamedClause(JsImportNamedClause { syntax })
			}
			JS_IMPORT_NAMESPACE_CLAUSE => {
				AnyJsImportClause::JsImportNamespaceClause(JsImportNamespaceClause { syntax })
			}
			JS_NAME => AnyJsImportClause::JsName(JsName { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			AnyJsImportClause::JsImportBareClause(it) => &it.syntax,
			AnyJsImportClause::JsImportDefaultClause(it) => &it.syntax,
			AnyJsImportClause::JsImportNamedClause(it) => &it.syntax,
			AnyJsImportClause::JsImportNamespaceClause(it) => &it.syntax,
			AnyJsImportClause::JsName(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for AnyJsImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AnyJsImportClause::JsImportBareClause(it) => std::fmt::Debug::fmt(it, f),
			AnyJsImportClause::JsImportDefaultClause(it) => std::fmt::Debug::fmt(it, f),
			AnyJsImportClause::JsImportNamedClause(it) => std::fmt::Debug::fmt(it, f),
			AnyJsImportClause::JsImportNamespaceClause(it) => std::fmt::Debug::fmt(it, f),
			AnyJsImportClause::JsName(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsClassDeclaration> for DefaultDecl {
	fn from(node: JsClassDeclaration) -> DefaultDecl { DefaultDecl::JsClassDeclaration(node) }
}
impl From<JsFunctionDeclaration> for DefaultDecl {
	fn from(node: JsFunctionDeclaration) -> DefaultDecl { DefaultDecl::JsFunctionDeclaration(node) }
}
impl AstNode for DefaultDecl {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(kind, JS_CLASS_DECLARATION | JS_FUNCTION_DECLARATION)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_CLASS_DECLARATION => DefaultDecl::JsClassDeclaration(JsClassDeclaration { syntax }),
			JS_FUNCTION_DECLARATION => {
				DefaultDecl::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			DefaultDecl::JsClassDeclaration(it) => &it.syntax,
			DefaultDecl::JsFunctionDeclaration(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for DefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			DefaultDecl::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
			DefaultDecl::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
		}
	}
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl From<JsArrayHole> for JsAnyArrayElement {
	fn from(node: JsArrayHole) -> JsAnyArrayElement { JsAnyArrayElement::JsArrayHole(node) }
}
impl From<JsSpread> for JsAnyArrayElement {
	fn from(node: JsSpread) -> JsAnyArrayElement { JsAnyArrayElement::JsSpread(node) }
}
impl AstNode for JsAnyArrayElement {
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl From<JsFunctionBody> for JsAnyArrowFunctionBody {
	fn from(node: JsFunctionBody) -> JsAnyArrowFunctionBody {
		JsAnyArrowFunctionBody::JsFunctionBody(node)
	}
}
impl AstNode for JsAnyArrowFunctionBody {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_FUNCTION_BODY => true,
			k if JsAnyExpression::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_FUNCTION_BODY => JsAnyArrowFunctionBody::JsFunctionBody(JsFunctionBody { syntax }),
			_ => {
				if let Some(js_any_expression) = JsAnyExpression::cast(syntax) {
					return Some(JsAnyArrowFunctionBody::JsAnyExpression(js_any_expression));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyArrowFunctionBody::JsFunctionBody(it) => &it.syntax,
			JsAnyArrowFunctionBody::JsAnyExpression(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyArrowFunctionBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyArrowFunctionBody::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyArrowFunctionBody::JsFunctionBody(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsParameters> for JsAnyArrowFunctionParameters {
	fn from(node: JsParameters) -> JsAnyArrowFunctionParameters {
		JsAnyArrowFunctionParameters::JsParameters(node)
	}
}
impl AstNode for JsAnyArrowFunctionParameters {
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl From<JsIdentifierBinding> for JsAnyBinding {
	fn from(node: JsIdentifierBinding) -> JsAnyBinding { JsAnyBinding::JsIdentifierBinding(node) }
}
impl From<JsUnknownBinding> for JsAnyBinding {
	fn from(node: JsUnknownBinding) -> JsAnyBinding { JsAnyBinding::JsUnknownBinding(node) }
}
impl AstNode for JsAnyBinding {
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl From<JsUnknownMember> for JsAnyClassMember {
	fn from(node: JsUnknownMember) -> JsAnyClassMember { JsAnyClassMember::JsUnknownMember(node) }
}
impl From<TsIndexSignature> for JsAnyClassMember {
	fn from(node: TsIndexSignature) -> JsAnyClassMember { JsAnyClassMember::TsIndexSignature(node) }
}
impl AstNode for JsAnyClassMember {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_CONSTRUCTOR_CLASS_MEMBER
				| JS_EMPTY_CLASS_MEMBER
				| JS_GETTER_CLASS_MEMBER
				| JS_METHOD_CLASS_MEMBER
				| JS_PROPERTY_CLASS_MEMBER
				| JS_SETTER_CLASS_MEMBER
				| JS_UNKNOWN_MEMBER
				| TS_INDEX_SIGNATURE
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
			JS_UNKNOWN_MEMBER => JsAnyClassMember::JsUnknownMember(JsUnknownMember { syntax }),
			TS_INDEX_SIGNATURE => JsAnyClassMember::TsIndexSignature(TsIndexSignature { syntax }),
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
			JsAnyClassMember::JsUnknownMember(it) => &it.syntax,
			JsAnyClassMember::TsIndexSignature(it) => &it.syntax,
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
			JsAnyClassMember::JsUnknownMember(it) => std::fmt::Debug::fmt(it, f),
			JsAnyClassMember::TsIndexSignature(it) => std::fmt::Debug::fmt(it, f),
		}
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl From<JsBindingPatternWithDefault> for JsAnyConstructorParameter {
	fn from(node: JsBindingPatternWithDefault) -> JsAnyConstructorParameter {
		JsAnyConstructorParameter::JsBindingPatternWithDefault(node)
	}
}
impl From<TsConstructorParam> for JsAnyConstructorParameter {
	fn from(node: TsConstructorParam) -> JsAnyConstructorParameter {
		JsAnyConstructorParameter::TsConstructorParam(node)
	}
}
impl AstNode for JsAnyConstructorParameter {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_BINDING_PATTERN_WITH_DEFAULT | TS_CONSTRUCTOR_PARAM => true,
			k if JsAnyBindingPattern::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				JsAnyConstructorParameter::JsBindingPatternWithDefault(
					JsBindingPatternWithDefault { syntax },
				)
			}
			TS_CONSTRUCTOR_PARAM => {
				JsAnyConstructorParameter::TsConstructorParam(TsConstructorParam { syntax })
			}
			_ => {
				if let Some(js_any_binding_pattern) = JsAnyBindingPattern::cast(syntax) {
					return Some(JsAnyConstructorParameter::JsAnyBindingPattern(
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
			JsAnyConstructorParameter::JsBindingPatternWithDefault(it) => &it.syntax,
			JsAnyConstructorParameter::TsConstructorParam(it) => &it.syntax,
			JsAnyConstructorParameter::JsAnyBindingPattern(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyConstructorParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyConstructorParameter::JsAnyBindingPattern(it) => std::fmt::Debug::fmt(it, f),
			JsAnyConstructorParameter::JsBindingPatternWithDefault(it) => {
				std::fmt::Debug::fmt(it, f)
			}
			JsAnyConstructorParameter::TsConstructorParam(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<ExportNamed> for JsAnyExportDeclaration {
	fn from(node: ExportNamed) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::ExportNamed(node)
	}
}
impl From<JsClassDeclaration> for JsAnyExportDeclaration {
	fn from(node: JsClassDeclaration) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::JsClassDeclaration(node)
	}
}
impl From<JsFunctionDeclaration> for JsAnyExportDeclaration {
	fn from(node: JsFunctionDeclaration) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::JsFunctionDeclaration(node)
	}
}
impl From<JsVariableStatement> for JsAnyExportDeclaration {
	fn from(node: JsVariableStatement) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::JsVariableStatement(node)
	}
}
impl From<TsEnum> for JsAnyExportDeclaration {
	fn from(node: TsEnum) -> JsAnyExportDeclaration { JsAnyExportDeclaration::TsEnum(node) }
}
impl From<TsInterfaceDecl> for JsAnyExportDeclaration {
	fn from(node: TsInterfaceDecl) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::TsInterfaceDecl(node)
	}
}
impl From<TsModuleDecl> for JsAnyExportDeclaration {
	fn from(node: TsModuleDecl) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::TsModuleDecl(node)
	}
}
impl From<TsNamespaceDecl> for JsAnyExportDeclaration {
	fn from(node: TsNamespaceDecl) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::TsNamespaceDecl(node)
	}
}
impl From<TsTypeAliasDecl> for JsAnyExportDeclaration {
	fn from(node: TsTypeAliasDecl) -> JsAnyExportDeclaration {
		JsAnyExportDeclaration::TsTypeAliasDecl(node)
	}
}
impl AstNode for JsAnyExportDeclaration {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			EXPORT_NAMED
				| JS_CLASS_DECLARATION
				| JS_FUNCTION_DECLARATION
				| JS_VARIABLE_STATEMENT
				| TS_ENUM | TS_INTERFACE_DECL
				| TS_MODULE_DECL | TS_NAMESPACE_DECL
				| TS_TYPE_ALIAS_DECL
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			EXPORT_NAMED => JsAnyExportDeclaration::ExportNamed(ExportNamed { syntax }),
			JS_CLASS_DECLARATION => {
				JsAnyExportDeclaration::JsClassDeclaration(JsClassDeclaration { syntax })
			}
			JS_FUNCTION_DECLARATION => {
				JsAnyExportDeclaration::JsFunctionDeclaration(JsFunctionDeclaration { syntax })
			}
			JS_VARIABLE_STATEMENT => {
				JsAnyExportDeclaration::JsVariableStatement(JsVariableStatement { syntax })
			}
			TS_ENUM => JsAnyExportDeclaration::TsEnum(TsEnum { syntax }),
			TS_INTERFACE_DECL => {
				JsAnyExportDeclaration::TsInterfaceDecl(TsInterfaceDecl { syntax })
			}
			TS_MODULE_DECL => JsAnyExportDeclaration::TsModuleDecl(TsModuleDecl { syntax }),
			TS_NAMESPACE_DECL => {
				JsAnyExportDeclaration::TsNamespaceDecl(TsNamespaceDecl { syntax })
			}
			TS_TYPE_ALIAS_DECL => {
				JsAnyExportDeclaration::TsTypeAliasDecl(TsTypeAliasDecl { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyExportDeclaration::ExportNamed(it) => &it.syntax,
			JsAnyExportDeclaration::JsClassDeclaration(it) => &it.syntax,
			JsAnyExportDeclaration::JsFunctionDeclaration(it) => &it.syntax,
			JsAnyExportDeclaration::JsVariableStatement(it) => &it.syntax,
			JsAnyExportDeclaration::TsEnum(it) => &it.syntax,
			JsAnyExportDeclaration::TsInterfaceDecl(it) => &it.syntax,
			JsAnyExportDeclaration::TsModuleDecl(it) => &it.syntax,
			JsAnyExportDeclaration::TsNamespaceDecl(it) => &it.syntax,
			JsAnyExportDeclaration::TsTypeAliasDecl(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyExportDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyExportDeclaration::ExportNamed(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::JsClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::JsFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::JsVariableStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsEnum(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsInterfaceDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsModuleDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsNamespaceDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExportDeclaration::TsTypeAliasDecl(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<CallExpr> for JsAnyExpression {
	fn from(node: CallExpr) -> JsAnyExpression { JsAnyExpression::CallExpr(node) }
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
impl From<JsLogicalExpression> for JsAnyExpression {
	fn from(node: JsLogicalExpression) -> JsAnyExpression {
		JsAnyExpression::JsLogicalExpression(node)
	}
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
impl From<NewExpr> for JsAnyExpression {
	fn from(node: NewExpr) -> JsAnyExpression { JsAnyExpression::NewExpr(node) }
}
impl From<NewTarget> for JsAnyExpression {
	fn from(node: NewTarget) -> JsAnyExpression { JsAnyExpression::NewTarget(node) }
}
impl From<Template> for JsAnyExpression {
	fn from(node: Template) -> JsAnyExpression { JsAnyExpression::Template(node) }
}
impl From<TsAssertion> for JsAnyExpression {
	fn from(node: TsAssertion) -> JsAnyExpression { JsAnyExpression::TsAssertion(node) }
}
impl From<TsConstAssertion> for JsAnyExpression {
	fn from(node: TsConstAssertion) -> JsAnyExpression { JsAnyExpression::TsConstAssertion(node) }
}
impl From<TsNonNull> for JsAnyExpression {
	fn from(node: TsNonNull) -> JsAnyExpression { JsAnyExpression::TsNonNull(node) }
}
impl AstNode for JsAnyExpression {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			CALL_EXPR
			| IMPORT_META
			| JS_ARRAY_EXPRESSION
			| JS_ARROW_FUNCTION_EXPRESSION
			| JS_ASSIGNMENT_EXPRESSION
			| JS_AWAIT_EXPRESSION
			| JS_BINARY_EXPRESSION
			| JS_CLASS_EXPRESSION
			| JS_COMPUTED_MEMBER_EXPRESSION
			| JS_CONDITIONAL_EXPRESSION
			| JS_FUNCTION_EXPRESSION
			| JS_IDENTIFIER_EXPRESSION
			| JS_IMPORT_CALL_EXPRESSION
			| JS_LOGICAL_EXPRESSION
			| JS_OBJECT_EXPRESSION
			| JS_PARENTHESIZED_EXPRESSION
			| JS_POST_UPDATE_EXPRESSION
			| JS_PRE_UPDATE_EXPRESSION
			| JS_SEQUENCE_EXPRESSION
			| JS_STATIC_MEMBER_EXPRESSION
			| JS_SUPER_EXPRESSION
			| JS_THIS_EXPRESSION
			| JS_UNARY_EXPRESSION
			| JS_UNKNOWN_EXPRESSION
			| JS_YIELD_EXPRESSION
			| NEW_EXPR
			| NEW_TARGET
			| TEMPLATE
			| TS_ASSERTION
			| TS_CONST_ASSERTION
			| TS_NON_NULL => true,
			k if JsAnyLiteralExpression::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			CALL_EXPR => JsAnyExpression::CallExpr(CallExpr { syntax }),
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
			JS_LOGICAL_EXPRESSION => {
				JsAnyExpression::JsLogicalExpression(JsLogicalExpression { syntax })
			}
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
			JS_THIS_EXPRESSION => JsAnyExpression::JsThisExpression(JsThisExpression { syntax }),
			JS_UNARY_EXPRESSION => JsAnyExpression::JsUnaryExpression(JsUnaryExpression { syntax }),
			JS_UNKNOWN_EXPRESSION => {
				JsAnyExpression::JsUnknownExpression(JsUnknownExpression { syntax })
			}
			JS_YIELD_EXPRESSION => JsAnyExpression::JsYieldExpression(JsYieldExpression { syntax }),
			NEW_EXPR => JsAnyExpression::NewExpr(NewExpr { syntax }),
			NEW_TARGET => JsAnyExpression::NewTarget(NewTarget { syntax }),
			TEMPLATE => JsAnyExpression::Template(Template { syntax }),
			TS_ASSERTION => JsAnyExpression::TsAssertion(TsAssertion { syntax }),
			TS_CONST_ASSERTION => JsAnyExpression::TsConstAssertion(TsConstAssertion { syntax }),
			TS_NON_NULL => JsAnyExpression::TsNonNull(TsNonNull { syntax }),
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
			JsAnyExpression::CallExpr(it) => &it.syntax,
			JsAnyExpression::ImportMeta(it) => &it.syntax,
			JsAnyExpression::JsArrayExpression(it) => &it.syntax,
			JsAnyExpression::JsArrowFunctionExpression(it) => &it.syntax,
			JsAnyExpression::JsAssignmentExpression(it) => &it.syntax,
			JsAnyExpression::JsAwaitExpression(it) => &it.syntax,
			JsAnyExpression::JsBinaryExpression(it) => &it.syntax,
			JsAnyExpression::JsClassExpression(it) => &it.syntax,
			JsAnyExpression::JsComputedMemberExpression(it) => &it.syntax,
			JsAnyExpression::JsConditionalExpression(it) => &it.syntax,
			JsAnyExpression::JsFunctionExpression(it) => &it.syntax,
			JsAnyExpression::JsIdentifierExpression(it) => &it.syntax,
			JsAnyExpression::JsImportCallExpression(it) => &it.syntax,
			JsAnyExpression::JsLogicalExpression(it) => &it.syntax,
			JsAnyExpression::JsObjectExpression(it) => &it.syntax,
			JsAnyExpression::JsParenthesizedExpression(it) => &it.syntax,
			JsAnyExpression::JsPostUpdateExpression(it) => &it.syntax,
			JsAnyExpression::JsPreUpdateExpression(it) => &it.syntax,
			JsAnyExpression::JsSequenceExpression(it) => &it.syntax,
			JsAnyExpression::JsStaticMemberExpression(it) => &it.syntax,
			JsAnyExpression::JsSuperExpression(it) => &it.syntax,
			JsAnyExpression::JsThisExpression(it) => &it.syntax,
			JsAnyExpression::JsUnaryExpression(it) => &it.syntax,
			JsAnyExpression::JsUnknownExpression(it) => &it.syntax,
			JsAnyExpression::JsYieldExpression(it) => &it.syntax,
			JsAnyExpression::NewExpr(it) => &it.syntax,
			JsAnyExpression::NewTarget(it) => &it.syntax,
			JsAnyExpression::Template(it) => &it.syntax,
			JsAnyExpression::TsAssertion(it) => &it.syntax,
			JsAnyExpression::TsConstAssertion(it) => &it.syntax,
			JsAnyExpression::TsNonNull(it) => &it.syntax,
			JsAnyExpression::JsAnyLiteralExpression(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyExpression::CallExpr(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::ImportMeta(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsAnyLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsArrayExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsArrowFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsAssignmentExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsAwaitExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsClassExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsComputedMemberExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsConditionalExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsIdentifierExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsImportCallExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsLogicalExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsObjectExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsPostUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsPreUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsSequenceExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsStaticMemberExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsSuperExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsThisExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsUnaryExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsUnknownExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::JsYieldExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::NewExpr(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::NewTarget(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::Template(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::TsAssertion(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::TsConstAssertion(it) => std::fmt::Debug::fmt(it, f),
			JsAnyExpression::TsNonNull(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsForVariableDeclaration> for JsAnyForInOrOfInitializer {
	fn from(node: JsForVariableDeclaration) -> JsAnyForInOrOfInitializer {
		JsAnyForInOrOfInitializer::JsForVariableDeclaration(node)
	}
}
impl AstNode for JsAnyForInOrOfInitializer {
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl From<JsVariableDeclarations> for JsAnyForInitializer {
	fn from(node: JsVariableDeclarations) -> JsAnyForInitializer {
		JsAnyForInitializer::JsVariableDeclarations(node)
	}
}
impl AstNode for JsAnyForInitializer {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_VARIABLE_DECLARATIONS => true,
			k if JsAnyExpression::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_VARIABLE_DECLARATIONS => {
				JsAnyForInitializer::JsVariableDeclarations(JsVariableDeclarations { syntax })
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
			JsAnyForInitializer::JsVariableDeclarations(it) => &it.syntax,
			JsAnyForInitializer::JsAnyExpression(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyForInitializer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyForInitializer::JsAnyExpression(it) => std::fmt::Debug::fmt(it, f),
			JsAnyForInitializer::JsVariableDeclarations(it) => std::fmt::Debug::fmt(it, f),
		}
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl From<JsModifier> for JsAnyModifier {
	fn from(node: JsModifier) -> JsAnyModifier { JsAnyModifier::JsModifier(node) }
}
impl From<JsUnknownModifier> for JsAnyModifier {
	fn from(node: JsUnknownModifier) -> JsAnyModifier { JsAnyModifier::JsUnknownModifier(node) }
}
impl AstNode for JsAnyModifier {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_MODIFIER | JS_UNKNOWN_MODIFIER) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_MODIFIER => JsAnyModifier::JsModifier(JsModifier { syntax }),
			JS_UNKNOWN_MODIFIER => JsAnyModifier::JsUnknownModifier(JsUnknownModifier { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyModifier::JsModifier(it) => &it.syntax,
			JsAnyModifier::JsUnknownModifier(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyModifier::JsModifier(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModifier::JsUnknownModifier(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<ExportDecl> for JsAnyModuleItem {
	fn from(node: ExportDecl) -> JsAnyModuleItem { JsAnyModuleItem::ExportDecl(node) }
}
impl From<ExportDefaultDecl> for JsAnyModuleItem {
	fn from(node: ExportDefaultDecl) -> JsAnyModuleItem { JsAnyModuleItem::ExportDefaultDecl(node) }
}
impl From<ExportDefaultExpr> for JsAnyModuleItem {
	fn from(node: ExportDefaultExpr) -> JsAnyModuleItem { JsAnyModuleItem::ExportDefaultExpr(node) }
}
impl From<ExportWildcard> for JsAnyModuleItem {
	fn from(node: ExportWildcard) -> JsAnyModuleItem { JsAnyModuleItem::ExportWildcard(node) }
}
impl From<JsImport> for JsAnyModuleItem {
	fn from(node: JsImport) -> JsAnyModuleItem { JsAnyModuleItem::JsImport(node) }
}
impl From<TsExportAssignment> for JsAnyModuleItem {
	fn from(node: TsExportAssignment) -> JsAnyModuleItem {
		JsAnyModuleItem::TsExportAssignment(node)
	}
}
impl From<TsImportEqualsDecl> for JsAnyModuleItem {
	fn from(node: TsImportEqualsDecl) -> JsAnyModuleItem {
		JsAnyModuleItem::TsImportEqualsDecl(node)
	}
}
impl From<TsNamespaceExportDecl> for JsAnyModuleItem {
	fn from(node: TsNamespaceExportDecl) -> JsAnyModuleItem {
		JsAnyModuleItem::TsNamespaceExportDecl(node)
	}
}
impl AstNode for JsAnyModuleItem {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			EXPORT_DECL
			| EXPORT_DEFAULT_DECL
			| EXPORT_DEFAULT_EXPR
			| EXPORT_WILDCARD
			| JS_IMPORT
			| TS_EXPORT_ASSIGNMENT
			| TS_IMPORT_EQUALS_DECL
			| TS_NAMESPACE_EXPORT_DECL => true,
			k if JsAnyStatement::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			EXPORT_DECL => JsAnyModuleItem::ExportDecl(ExportDecl { syntax }),
			EXPORT_DEFAULT_DECL => JsAnyModuleItem::ExportDefaultDecl(ExportDefaultDecl { syntax }),
			EXPORT_DEFAULT_EXPR => JsAnyModuleItem::ExportDefaultExpr(ExportDefaultExpr { syntax }),
			EXPORT_WILDCARD => JsAnyModuleItem::ExportWildcard(ExportWildcard { syntax }),
			JS_IMPORT => JsAnyModuleItem::JsImport(JsImport { syntax }),
			TS_EXPORT_ASSIGNMENT => {
				JsAnyModuleItem::TsExportAssignment(TsExportAssignment { syntax })
			}
			TS_IMPORT_EQUALS_DECL => {
				JsAnyModuleItem::TsImportEqualsDecl(TsImportEqualsDecl { syntax })
			}
			TS_NAMESPACE_EXPORT_DECL => {
				JsAnyModuleItem::TsNamespaceExportDecl(TsNamespaceExportDecl { syntax })
			}
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
			JsAnyModuleItem::ExportDecl(it) => &it.syntax,
			JsAnyModuleItem::ExportDefaultDecl(it) => &it.syntax,
			JsAnyModuleItem::ExportDefaultExpr(it) => &it.syntax,
			JsAnyModuleItem::ExportWildcard(it) => &it.syntax,
			JsAnyModuleItem::JsImport(it) => &it.syntax,
			JsAnyModuleItem::TsExportAssignment(it) => &it.syntax,
			JsAnyModuleItem::TsImportEqualsDecl(it) => &it.syntax,
			JsAnyModuleItem::TsNamespaceExportDecl(it) => &it.syntax,
			JsAnyModuleItem::JsAnyStatement(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyModuleItem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyModuleItem::ExportDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::ExportDefaultDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::ExportDefaultExpr(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::ExportWildcard(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::JsAnyStatement(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::JsImport(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::TsExportAssignment(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::TsImportEqualsDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyModuleItem::TsNamespaceExportDecl(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsName> for JsAnyName {
	fn from(node: JsName) -> JsAnyName { JsAnyName::JsName(node) }
}
impl From<JsPrivateName> for JsAnyName {
	fn from(node: JsPrivateName) -> JsAnyName { JsAnyName::JsPrivateName(node) }
}
impl AstNode for JsAnyName {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_NAME | JS_PRIVATE_NAME) }
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			JS_GETTER_OBJECT_MEMBER
				| JS_METHOD_OBJECT_MEMBER
				| JS_PROPERTY_OBJECT_MEMBER
				| JS_SETTER_OBJECT_MEMBER
				| JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
				| JS_SPREAD | JS_UNKNOWN_MEMBER
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
	fn can_cast(kind: SyntaxKind) -> bool {
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
impl From<JsBindingPatternWithDefault> for JsAnyParameter {
	fn from(node: JsBindingPatternWithDefault) -> JsAnyParameter {
		JsAnyParameter::JsBindingPatternWithDefault(node)
	}
}
impl From<JsRestParameter> for JsAnyParameter {
	fn from(node: JsRestParameter) -> JsAnyParameter { JsAnyParameter::JsRestParameter(node) }
}
impl AstNode for JsAnyParameter {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			JS_BINDING_PATTERN_WITH_DEFAULT | JS_REST_PARAMETER => true,
			k if JsAnyBindingPattern::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_BINDING_PATTERN_WITH_DEFAULT => {
				JsAnyParameter::JsBindingPatternWithDefault(JsBindingPatternWithDefault { syntax })
			}
			JS_REST_PARAMETER => JsAnyParameter::JsRestParameter(JsRestParameter { syntax }),
			_ => {
				if let Some(js_any_binding_pattern) = JsAnyBindingPattern::cast(syntax) {
					return Some(JsAnyParameter::JsAnyBindingPattern(js_any_binding_pattern));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyParameter::JsBindingPatternWithDefault(it) => &it.syntax,
			JsAnyParameter::JsRestParameter(it) => &it.syntax,
			JsAnyParameter::JsAnyBindingPattern(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for JsAnyParameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyParameter::JsAnyBindingPattern(it) => std::fmt::Debug::fmt(it, f),
			JsAnyParameter::JsBindingPatternWithDefault(it) => std::fmt::Debug::fmt(it, f),
			JsAnyParameter::JsRestParameter(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsModule> for JsAnyRoot {
	fn from(node: JsModule) -> JsAnyRoot { JsAnyRoot::JsModule(node) }
}
impl From<JsScript> for JsAnyRoot {
	fn from(node: JsScript) -> JsAnyRoot { JsAnyRoot::JsScript(node) }
}
impl AstNode for JsAnyRoot {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_MODULE | JS_SCRIPT) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			JS_MODULE => JsAnyRoot::JsModule(JsModule { syntax }),
			JS_SCRIPT => JsAnyRoot::JsScript(JsScript { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyRoot::JsModule(it) => &it.syntax,
			JsAnyRoot::JsScript(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyRoot {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyRoot::JsModule(it) => std::fmt::Debug::fmt(it, f),
			JsAnyRoot::JsScript(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<ForStmt> for JsAnyStatement {
	fn from(node: ForStmt) -> JsAnyStatement { JsAnyStatement::ForStmt(node) }
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
impl From<TsEnum> for JsAnyStatement {
	fn from(node: TsEnum) -> JsAnyStatement { JsAnyStatement::TsEnum(node) }
}
impl From<TsInterfaceDecl> for JsAnyStatement {
	fn from(node: TsInterfaceDecl) -> JsAnyStatement { JsAnyStatement::TsInterfaceDecl(node) }
}
impl From<TsModuleDecl> for JsAnyStatement {
	fn from(node: TsModuleDecl) -> JsAnyStatement { JsAnyStatement::TsModuleDecl(node) }
}
impl From<TsNamespaceDecl> for JsAnyStatement {
	fn from(node: TsNamespaceDecl) -> JsAnyStatement { JsAnyStatement::TsNamespaceDecl(node) }
}
impl From<TsTypeAliasDecl> for JsAnyStatement {
	fn from(node: TsTypeAliasDecl) -> JsAnyStatement { JsAnyStatement::TsTypeAliasDecl(node) }
}
impl AstNode for JsAnyStatement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			FOR_STMT
				| JS_BLOCK_STATEMENT
				| JS_BREAK_STATEMENT
				| JS_CLASS_DECLARATION
				| JS_CONTINUE_STATEMENT
				| JS_DEBUGGER_STATEMENT
				| JS_DO_WHILE_STATEMENT
				| JS_EMPTY_STATEMENT
				| JS_EXPRESSION_STATEMENT
				| JS_FOR_IN_STATEMENT
				| JS_FOR_OF_STATEMENT
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
				| TS_ENUM | TS_INTERFACE_DECL
				| TS_MODULE_DECL | TS_NAMESPACE_DECL
				| TS_TYPE_ALIAS_DECL
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			FOR_STMT => JsAnyStatement::ForStmt(ForStmt { syntax }),
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
			TS_ENUM => JsAnyStatement::TsEnum(TsEnum { syntax }),
			TS_INTERFACE_DECL => JsAnyStatement::TsInterfaceDecl(TsInterfaceDecl { syntax }),
			TS_MODULE_DECL => JsAnyStatement::TsModuleDecl(TsModuleDecl { syntax }),
			TS_NAMESPACE_DECL => JsAnyStatement::TsNamespaceDecl(TsNamespaceDecl { syntax }),
			TS_TYPE_ALIAS_DECL => JsAnyStatement::TsTypeAliasDecl(TsTypeAliasDecl { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			JsAnyStatement::ForStmt(it) => &it.syntax,
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
			JsAnyStatement::TsEnum(it) => &it.syntax,
			JsAnyStatement::TsInterfaceDecl(it) => &it.syntax,
			JsAnyStatement::TsModuleDecl(it) => &it.syntax,
			JsAnyStatement::TsNamespaceDecl(it) => &it.syntax,
			JsAnyStatement::TsTypeAliasDecl(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for JsAnyStatement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			JsAnyStatement::ForStmt(it) => std::fmt::Debug::fmt(it, f),
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
			JsAnyStatement::TsEnum(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsInterfaceDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsModuleDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsNamespaceDecl(it) => std::fmt::Debug::fmt(it, f),
			JsAnyStatement::TsTypeAliasDecl(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<JsCaseClause> for JsAnySwitchClause {
	fn from(node: JsCaseClause) -> JsAnySwitchClause { JsAnySwitchClause::JsCaseClause(node) }
}
impl From<JsDefaultClause> for JsAnySwitchClause {
	fn from(node: JsDefaultClause) -> JsAnySwitchClause { JsAnySwitchClause::JsDefaultClause(node) }
}
impl AstNode for JsAnySwitchClause {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, JS_CASE_CLAUSE | JS_DEFAULT_CLAUSE) }
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
impl From<TsQualifiedPath> for TsEntityName {
	fn from(node: TsQualifiedPath) -> TsEntityName { TsEntityName::TsQualifiedPath(node) }
}
impl From<TsTypeName> for TsEntityName {
	fn from(node: TsTypeName) -> TsEntityName { TsEntityName::TsTypeName(node) }
}
impl AstNode for TsEntityName {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, TS_QUALIFIED_PATH | TS_TYPE_NAME) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_QUALIFIED_PATH => TsEntityName::TsQualifiedPath(TsQualifiedPath { syntax }),
			TS_TYPE_NAME => TsEntityName::TsTypeName(TsTypeName { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsEntityName::TsQualifiedPath(it) => &it.syntax,
			TsEntityName::TsTypeName(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsEntityName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsEntityName::TsQualifiedPath(it) => std::fmt::Debug::fmt(it, f),
			TsEntityName::TsTypeName(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsExternalModuleRef> for TsModuleRef {
	fn from(node: TsExternalModuleRef) -> TsModuleRef { TsModuleRef::TsExternalModuleRef(node) }
}
impl AstNode for TsModuleRef {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			TS_EXTERNAL_MODULE_REF => true,
			k if TsEntityName::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_EXTERNAL_MODULE_REF => {
				TsModuleRef::TsExternalModuleRef(TsExternalModuleRef { syntax })
			}
			_ => {
				if let Some(ts_entity_name) = TsEntityName::cast(syntax) {
					return Some(TsModuleRef::TsEntityName(ts_entity_name));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsModuleRef::TsExternalModuleRef(it) => &it.syntax,
			TsModuleRef::TsEntityName(it) => it.syntax(),
		}
	}
}
impl std::fmt::Debug for TsModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsModuleRef::TsEntityName(it) => std::fmt::Debug::fmt(it, f),
			TsModuleRef::TsExternalModuleRef(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsModuleBlock> for TsNamespaceBody {
	fn from(node: TsModuleBlock) -> TsNamespaceBody { TsNamespaceBody::TsModuleBlock(node) }
}
impl From<TsNamespaceDecl> for TsNamespaceBody {
	fn from(node: TsNamespaceDecl) -> TsNamespaceBody { TsNamespaceBody::TsNamespaceDecl(node) }
}
impl AstNode for TsNamespaceBody {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, TS_MODULE_BLOCK | TS_NAMESPACE_DECL) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_MODULE_BLOCK => TsNamespaceBody::TsModuleBlock(TsModuleBlock { syntax }),
			TS_NAMESPACE_DECL => TsNamespaceBody::TsNamespaceDecl(TsNamespaceDecl { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsNamespaceBody::TsModuleBlock(it) => &it.syntax,
			TsNamespaceBody::TsNamespaceDecl(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsNamespaceBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsNamespaceBody::TsModuleBlock(it) => std::fmt::Debug::fmt(it, f),
			TsNamespaceBody::TsNamespaceDecl(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsThis> for TsThisOrMore {
	fn from(node: TsThis) -> TsThisOrMore { TsThisOrMore::TsThis(node) }
}
impl From<TsTypeName> for TsThisOrMore {
	fn from(node: TsTypeName) -> TsThisOrMore { TsThisOrMore::TsTypeName(node) }
}
impl AstNode for TsThisOrMore {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, TS_THIS | TS_TYPE_NAME) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_THIS => TsThisOrMore::TsThis(TsThis { syntax }),
			TS_TYPE_NAME => TsThisOrMore::TsTypeName(TsTypeName { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsThisOrMore::TsThis(it) => &it.syntax,
			TsThisOrMore::TsTypeName(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsThisOrMore {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsThisOrMore::TsThis(it) => std::fmt::Debug::fmt(it, f),
			TsThisOrMore::TsTypeName(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsAny> for TsType {
	fn from(node: TsAny) -> TsType { TsType::TsAny(node) }
}
impl From<TsArray> for TsType {
	fn from(node: TsArray) -> TsType { TsType::TsArray(node) }
}
impl From<TsBigint> for TsType {
	fn from(node: TsBigint) -> TsType { TsType::TsBigint(node) }
}
impl From<TsBoolean> for TsType {
	fn from(node: TsBoolean) -> TsType { TsType::TsBoolean(node) }
}
impl From<TsConditionalType> for TsType {
	fn from(node: TsConditionalType) -> TsType { TsType::TsConditionalType(node) }
}
impl From<TsConstructorType> for TsType {
	fn from(node: TsConstructorType) -> TsType { TsType::TsConstructorType(node) }
}
impl From<TsFnType> for TsType {
	fn from(node: TsFnType) -> TsType { TsType::TsFnType(node) }
}
impl From<TsImport> for TsType {
	fn from(node: TsImport) -> TsType { TsType::TsImport(node) }
}
impl From<TsIndexedArray> for TsType {
	fn from(node: TsIndexedArray) -> TsType { TsType::TsIndexedArray(node) }
}
impl From<TsInfer> for TsType {
	fn from(node: TsInfer) -> TsType { TsType::TsInfer(node) }
}
impl From<TsIntersection> for TsType {
	fn from(node: TsIntersection) -> TsType { TsType::TsIntersection(node) }
}
impl From<TsLiteral> for TsType {
	fn from(node: TsLiteral) -> TsType { TsType::TsLiteral(node) }
}
impl From<TsMappedType> for TsType {
	fn from(node: TsMappedType) -> TsType { TsType::TsMappedType(node) }
}
impl From<TsNever> for TsType {
	fn from(node: TsNever) -> TsType { TsType::TsNever(node) }
}
impl From<TsNull> for TsType {
	fn from(node: TsNull) -> TsType { TsType::TsNull(node) }
}
impl From<TsNumber> for TsType {
	fn from(node: TsNumber) -> TsType { TsType::TsNumber(node) }
}
impl From<TsObject> for TsType {
	fn from(node: TsObject) -> TsType { TsType::TsObject(node) }
}
impl From<TsObjectType> for TsType {
	fn from(node: TsObjectType) -> TsType { TsType::TsObjectType(node) }
}
impl From<TsParen> for TsType {
	fn from(node: TsParen) -> TsType { TsType::TsParen(node) }
}
impl From<TsPredicate> for TsType {
	fn from(node: TsPredicate) -> TsType { TsType::TsPredicate(node) }
}
impl From<TsString> for TsType {
	fn from(node: TsString) -> TsType { TsType::TsString(node) }
}
impl From<TsSymbol> for TsType {
	fn from(node: TsSymbol) -> TsType { TsType::TsSymbol(node) }
}
impl From<TsTemplate> for TsType {
	fn from(node: TsTemplate) -> TsType { TsType::TsTemplate(node) }
}
impl From<TsThis> for TsType {
	fn from(node: TsThis) -> TsType { TsType::TsThis(node) }
}
impl From<TsTuple> for TsType {
	fn from(node: TsTuple) -> TsType { TsType::TsTuple(node) }
}
impl From<TsTypeOperator> for TsType {
	fn from(node: TsTypeOperator) -> TsType { TsType::TsTypeOperator(node) }
}
impl From<TsTypeRef> for TsType {
	fn from(node: TsTypeRef) -> TsType { TsType::TsTypeRef(node) }
}
impl From<TsUndefined> for TsType {
	fn from(node: TsUndefined) -> TsType { TsType::TsUndefined(node) }
}
impl From<TsUnion> for TsType {
	fn from(node: TsUnion) -> TsType { TsType::TsUnion(node) }
}
impl From<TsUnknown> for TsType {
	fn from(node: TsUnknown) -> TsType { TsType::TsUnknown(node) }
}
impl From<TsVoid> for TsType {
	fn from(node: TsVoid) -> TsType { TsType::TsVoid(node) }
}
impl AstNode for TsType {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			TS_ANY
				| TS_ARRAY | TS_BIGINT
				| TS_BOOLEAN | TS_CONDITIONAL_TYPE
				| TS_CONSTRUCTOR_TYPE
				| TS_FN_TYPE | TS_IMPORT
				| TS_INDEXED_ARRAY
				| TS_INFER | TS_INTERSECTION
				| TS_LITERAL | TS_MAPPED_TYPE
				| TS_NEVER | TS_NULL
				| TS_NUMBER | TS_OBJECT
				| TS_OBJECT_TYPE | TS_PAREN
				| TS_PREDICATE | TS_STRING
				| TS_SYMBOL | TS_TEMPLATE
				| TS_THIS | TS_TUPLE
				| TS_TYPE_OPERATOR
				| TS_TYPE_REF | TS_UNDEFINED
				| TS_UNION | TS_UNKNOWN
				| TS_VOID
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_ANY => TsType::TsAny(TsAny { syntax }),
			TS_ARRAY => TsType::TsArray(TsArray { syntax }),
			TS_BIGINT => TsType::TsBigint(TsBigint { syntax }),
			TS_BOOLEAN => TsType::TsBoolean(TsBoolean { syntax }),
			TS_CONDITIONAL_TYPE => TsType::TsConditionalType(TsConditionalType { syntax }),
			TS_CONSTRUCTOR_TYPE => TsType::TsConstructorType(TsConstructorType { syntax }),
			TS_FN_TYPE => TsType::TsFnType(TsFnType { syntax }),
			TS_IMPORT => TsType::TsImport(TsImport { syntax }),
			TS_INDEXED_ARRAY => TsType::TsIndexedArray(TsIndexedArray { syntax }),
			TS_INFER => TsType::TsInfer(TsInfer { syntax }),
			TS_INTERSECTION => TsType::TsIntersection(TsIntersection { syntax }),
			TS_LITERAL => TsType::TsLiteral(TsLiteral { syntax }),
			TS_MAPPED_TYPE => TsType::TsMappedType(TsMappedType { syntax }),
			TS_NEVER => TsType::TsNever(TsNever { syntax }),
			TS_NULL => TsType::TsNull(TsNull { syntax }),
			TS_NUMBER => TsType::TsNumber(TsNumber { syntax }),
			TS_OBJECT => TsType::TsObject(TsObject { syntax }),
			TS_OBJECT_TYPE => TsType::TsObjectType(TsObjectType { syntax }),
			TS_PAREN => TsType::TsParen(TsParen { syntax }),
			TS_PREDICATE => TsType::TsPredicate(TsPredicate { syntax }),
			TS_STRING => TsType::TsString(TsString { syntax }),
			TS_SYMBOL => TsType::TsSymbol(TsSymbol { syntax }),
			TS_TEMPLATE => TsType::TsTemplate(TsTemplate { syntax }),
			TS_THIS => TsType::TsThis(TsThis { syntax }),
			TS_TUPLE => TsType::TsTuple(TsTuple { syntax }),
			TS_TYPE_OPERATOR => TsType::TsTypeOperator(TsTypeOperator { syntax }),
			TS_TYPE_REF => TsType::TsTypeRef(TsTypeRef { syntax }),
			TS_UNDEFINED => TsType::TsUndefined(TsUndefined { syntax }),
			TS_UNION => TsType::TsUnion(TsUnion { syntax }),
			TS_UNKNOWN => TsType::TsUnknown(TsUnknown { syntax }),
			TS_VOID => TsType::TsVoid(TsVoid { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsType::TsAny(it) => &it.syntax,
			TsType::TsArray(it) => &it.syntax,
			TsType::TsBigint(it) => &it.syntax,
			TsType::TsBoolean(it) => &it.syntax,
			TsType::TsConditionalType(it) => &it.syntax,
			TsType::TsConstructorType(it) => &it.syntax,
			TsType::TsFnType(it) => &it.syntax,
			TsType::TsImport(it) => &it.syntax,
			TsType::TsIndexedArray(it) => &it.syntax,
			TsType::TsInfer(it) => &it.syntax,
			TsType::TsIntersection(it) => &it.syntax,
			TsType::TsLiteral(it) => &it.syntax,
			TsType::TsMappedType(it) => &it.syntax,
			TsType::TsNever(it) => &it.syntax,
			TsType::TsNull(it) => &it.syntax,
			TsType::TsNumber(it) => &it.syntax,
			TsType::TsObject(it) => &it.syntax,
			TsType::TsObjectType(it) => &it.syntax,
			TsType::TsParen(it) => &it.syntax,
			TsType::TsPredicate(it) => &it.syntax,
			TsType::TsString(it) => &it.syntax,
			TsType::TsSymbol(it) => &it.syntax,
			TsType::TsTemplate(it) => &it.syntax,
			TsType::TsThis(it) => &it.syntax,
			TsType::TsTuple(it) => &it.syntax,
			TsType::TsTypeOperator(it) => &it.syntax,
			TsType::TsTypeRef(it) => &it.syntax,
			TsType::TsUndefined(it) => &it.syntax,
			TsType::TsUnion(it) => &it.syntax,
			TsType::TsUnknown(it) => &it.syntax,
			TsType::TsVoid(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsType::TsAny(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsArray(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsBigint(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsBoolean(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsConditionalType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsConstructorType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsFnType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsImport(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsIndexedArray(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsInfer(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsIntersection(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsLiteral(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsMappedType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsNever(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsNull(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsNumber(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsObject(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsObjectType(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsParen(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsPredicate(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsString(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsSymbol(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsTemplate(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsThis(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsTuple(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsTypeOperator(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsTypeRef(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsUndefined(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsUnion(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsUnknown(it) => std::fmt::Debug::fmt(it, f),
			TsType::TsVoid(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl From<TsCallSignatureDecl> for TsTypeElement {
	fn from(node: TsCallSignatureDecl) -> TsTypeElement { TsTypeElement::TsCallSignatureDecl(node) }
}
impl From<TsConstructSignatureDecl> for TsTypeElement {
	fn from(node: TsConstructSignatureDecl) -> TsTypeElement {
		TsTypeElement::TsConstructSignatureDecl(node)
	}
}
impl From<TsIndexSignature> for TsTypeElement {
	fn from(node: TsIndexSignature) -> TsTypeElement { TsTypeElement::TsIndexSignature(node) }
}
impl From<TsMethodSignature> for TsTypeElement {
	fn from(node: TsMethodSignature) -> TsTypeElement { TsTypeElement::TsMethodSignature(node) }
}
impl From<TsPropertySignature> for TsTypeElement {
	fn from(node: TsPropertySignature) -> TsTypeElement { TsTypeElement::TsPropertySignature(node) }
}
impl AstNode for TsTypeElement {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			TS_CALL_SIGNATURE_DECL
				| TS_CONSTRUCT_SIGNATURE_DECL
				| TS_INDEX_SIGNATURE
				| TS_METHOD_SIGNATURE
				| TS_PROPERTY_SIGNATURE
		)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			TS_CALL_SIGNATURE_DECL => {
				TsTypeElement::TsCallSignatureDecl(TsCallSignatureDecl { syntax })
			}
			TS_CONSTRUCT_SIGNATURE_DECL => {
				TsTypeElement::TsConstructSignatureDecl(TsConstructSignatureDecl { syntax })
			}
			TS_INDEX_SIGNATURE => TsTypeElement::TsIndexSignature(TsIndexSignature { syntax }),
			TS_METHOD_SIGNATURE => TsTypeElement::TsMethodSignature(TsMethodSignature { syntax }),
			TS_PROPERTY_SIGNATURE => {
				TsTypeElement::TsPropertySignature(TsPropertySignature { syntax })
			}
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsTypeElement::TsCallSignatureDecl(it) => &it.syntax,
			TsTypeElement::TsConstructSignatureDecl(it) => &it.syntax,
			TsTypeElement::TsIndexSignature(it) => &it.syntax,
			TsTypeElement::TsMethodSignature(it) => &it.syntax,
			TsTypeElement::TsPropertySignature(it) => &it.syntax,
		}
	}
}
impl std::fmt::Debug for TsTypeElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TsTypeElement::TsCallSignatureDecl(it) => std::fmt::Debug::fmt(it, f),
			TsTypeElement::TsConstructSignatureDecl(it) => std::fmt::Debug::fmt(it, f),
			TsTypeElement::TsIndexSignature(it) => std::fmt::Debug::fmt(it, f),
			TsTypeElement::TsMethodSignature(it) => std::fmt::Debug::fmt(it, f),
			TsTypeElement::TsPropertySignature(it) => std::fmt::Debug::fmt(it, f),
		}
	}
}
impl std::fmt::Display for AnyJsImportClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for DefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
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
impl std::fmt::Display for JsAnyArrowFunctionBody {
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
impl std::fmt::Display for JsAnyExportDeclaration {
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
impl std::fmt::Display for JsAnyImportAssertionEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyLiteralExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for JsAnyModifier {
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
impl std::fmt::Display for TsEntityName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceBody {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsThisOrMore {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for CallExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDefaultDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportDefaultExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportNamed {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ExportWildcard {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmtTest {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for ForStmtUpdate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Ident {
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
impl std::fmt::Display for JsForVariableDeclaration {
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
impl std::fmt::Display for JsInitializerClause {
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
impl std::fmt::Display for JsModifier {
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
impl std::fmt::Display for JsVariableDeclarations {
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
impl std::fmt::Display for JsYieldExpression {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewExpr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for NewTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Specifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Template {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAccessibility {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAny {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsBigint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsBoolean {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsCallSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConditionalType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstAssertion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstraint {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructSignatureDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructorParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsConstructorType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsDefault {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsEnum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsEnumMember {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExportAssignment {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExprWithTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExtends {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsExternalModuleRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsFnType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsImplementsClause {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsImport {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsImportEqualsDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIndexSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIndexedArray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsInfer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsInterfaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsIntersection {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMappedTypeReadonly {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsMethodSignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleBlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsModuleDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNamespaceExportDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNever {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNonNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNull {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsObjectType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsParen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsPredicate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsPropertySignature {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsQualifiedPath {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsString {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsSymbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTemplate {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTemplateElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsThis {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTuple {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTupleElement {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeAliasDecl {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeAnnotation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeArgs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeName {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeOperator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeParam {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeParams {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsTypeRef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUndefined {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUnion {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsUnknown {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for TsVoid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownAssignment {
	syntax: SyntaxNode,
}
impl JsUnknownAssignment {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownAssignment {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_ASSIGNMENT }
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownBinding {
	syntax: SyntaxNode,
}
impl JsUnknownBinding {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownBinding {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_BINDING }
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownExpression {
	syntax: SyntaxNode,
}
impl JsUnknownExpression {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownExpression {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_EXPRESSION }
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownImportAssertionEntry {
	syntax: SyntaxNode,
}
impl JsUnknownImportAssertionEntry {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownImportAssertionEntry {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_IMPORT_ASSERTION_ENTRY }
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownMember {
	syntax: SyntaxNode,
}
impl JsUnknownMember {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownMember {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_MEMBER }
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownModifier {
	syntax: SyntaxNode,
}
impl JsUnknownModifier {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownModifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_MODIFIER }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl std::fmt::Debug for JsUnknownModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JsUnknownModifier")
			.field("items", &support::DebugSyntaxElementChildren(self.items()))
			.finish()
	}
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownNamedImportSpecifier {
	syntax: SyntaxNode,
}
impl JsUnknownNamedImportSpecifier {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownNamedImportSpecifier {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_NAMED_IMPORT_SPECIFIER }
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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsUnknownStatement {
	syntax: SyntaxNode,
}
impl JsUnknownStatement {
	pub fn items(&self) -> SyntaxElementChildren { support::elements(&self.syntax) }
}
impl AstNode for JsUnknownStatement {
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_UNKNOWN_STATEMENT }
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
#[derive(Default, Clone)]
pub struct ExportNamedSpecifierList {
	syntax_list: SyntaxList,
}
impl AstList for ExportNamedSpecifierList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == EXPORT_NAMED_SPECIFIER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<ExportNamedSpecifierList> {
		if Self::can_cast(syntax.kind()) {
			Some(ExportNamedSpecifierList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<Specifier> for ExportNamedSpecifierList {}
impl Debug for ExportNamedSpecifierList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.elements()).finish()
	}
}
impl IntoIterator for ExportNamedSpecifierList {
	type Item = SyntaxResult<Specifier>;
	type IntoIter = AstSeparatedListNodesIterator<Specifier>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &ExportNamedSpecifierList {
	type Item = SyntaxResult<Specifier>;
	type IntoIter = AstSeparatedListNodesIterator<Specifier>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Default, Clone)]
pub struct JsArrayAssignmentPatternElementList {
	syntax_list: SyntaxList,
}
impl AstList for JsArrayAssignmentPatternElementList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsArrayAssignmentPatternElementList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsArrayAssignmentPatternElementList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyArrayAssignmentPatternElement> for JsArrayAssignmentPatternElementList {}
impl Debug for JsArrayAssignmentPatternElementList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsArrayBindingPatternElementList {
	syntax_list: SyntaxList,
}
impl AstList for JsArrayBindingPatternElementList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsArrayBindingPatternElementList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsArrayBindingPatternElementList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyArrayBindingPatternElement> for JsArrayBindingPatternElementList {}
impl Debug for JsArrayBindingPatternElementList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsArrayElementList {
	syntax_list: SyntaxList,
}
impl AstList for JsArrayElementList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_ARRAY_ELEMENT_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsArrayElementList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsArrayElementList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyArrayElement> for JsArrayElementList {}
impl Debug for JsArrayElementList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsCallArgumentList {
	syntax_list: SyntaxList,
}
impl AstList for JsCallArgumentList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CALL_ARGUMENT_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsCallArgumentList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsCallArgumentList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyExpression> for JsCallArgumentList {}
impl Debug for JsCallArgumentList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsClassMemberList {
	syntax_list: SyntaxList,
}
impl AstList for JsClassMemberList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CLASS_MEMBER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsClassMemberList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsClassMemberList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstNodeList<JsAnyClassMember> for JsClassMemberList {}
impl Debug for JsClassMemberList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsConstructorParameterList {
	syntax_list: SyntaxList,
}
impl AstList for JsConstructorParameterList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_CONSTRUCTOR_PARAMETER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsConstructorParameterList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsConstructorParameterList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyConstructorParameter> for JsConstructorParameterList {}
impl Debug for JsConstructorParameterList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsDirectiveList {
	syntax_list: SyntaxList,
}
impl AstList for JsDirectiveList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_DIRECTIVE_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsDirectiveList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsDirectiveList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstNodeList<JsDirective> for JsDirectiveList {}
impl Debug for JsDirectiveList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsImportAssertionEntryList {
	syntax_list: SyntaxList,
}
impl AstList for JsImportAssertionEntryList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_IMPORT_ASSERTION_ENTRY_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsImportAssertionEntryList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsImportAssertionEntryList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyImportAssertionEntry> for JsImportAssertionEntryList {}
impl Debug for JsImportAssertionEntryList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsModuleItemList {
	syntax_list: SyntaxList,
}
impl AstList for JsModuleItemList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_MODULE_ITEM_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsModuleItemList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsModuleItemList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstNodeList<JsAnyModuleItem> for JsModuleItemList {}
impl Debug for JsModuleItemList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsNamedImportSpecifierList {
	syntax_list: SyntaxList,
}
impl AstList for JsNamedImportSpecifierList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_NAMED_IMPORT_SPECIFIER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsNamedImportSpecifierList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsNamedImportSpecifierList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyNamedImportSpecifier> for JsNamedImportSpecifierList {}
impl Debug for JsNamedImportSpecifierList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsObjectAssignmentPatternPropertyList {
	syntax_list: SyntaxList,
}
impl AstList for JsObjectAssignmentPatternPropertyList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsObjectAssignmentPatternPropertyList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsObjectAssignmentPatternPropertyList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyObjectAssignmentPatternMember>
	for JsObjectAssignmentPatternPropertyList
{
}
impl Debug for JsObjectAssignmentPatternPropertyList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsObjectBindingPatternPropertyList {
	syntax_list: SyntaxList,
}
impl AstList for JsObjectBindingPatternPropertyList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsObjectBindingPatternPropertyList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsObjectBindingPatternPropertyList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyObjectBindingPatternMember> for JsObjectBindingPatternPropertyList {}
impl Debug for JsObjectBindingPatternPropertyList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsObjectMemberList {
	syntax_list: SyntaxList,
}
impl AstList for JsObjectMemberList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_OBJECT_MEMBER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsObjectMemberList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsObjectMemberList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyObjectMember> for JsObjectMemberList {}
impl Debug for JsObjectMemberList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsParameterList {
	syntax_list: SyntaxList,
}
impl AstList for JsParameterList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_PARAMETER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsParameterList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsParameterList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsAnyParameter> for JsParameterList {}
impl Debug for JsParameterList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsStatementList {
	syntax_list: SyntaxList,
}
impl AstList for JsStatementList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_STATEMENT_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsStatementList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsStatementList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstNodeList<JsAnyStatement> for JsStatementList {}
impl Debug for JsStatementList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsSwitchCaseList {
	syntax_list: SyntaxList,
}
impl AstList for JsSwitchCaseList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_SWITCH_CASE_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsSwitchCaseList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsSwitchCaseList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstNodeList<JsAnySwitchClause> for JsSwitchCaseList {}
impl Debug for JsSwitchCaseList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Default, Clone)]
pub struct JsVariableDeclarationList {
	syntax_list: SyntaxList,
}
impl AstList for JsVariableDeclarationList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == JS_VARIABLE_DECLARATION_LIST }
	fn cast(syntax: SyntaxNode) -> Option<JsVariableDeclarationList> {
		if Self::can_cast(syntax.kind()) {
			Some(JsVariableDeclarationList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<JsVariableDeclaration> for JsVariableDeclarationList {}
impl Debug for JsVariableDeclarationList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.elements()).finish()
	}
}
impl IntoIterator for JsVariableDeclarationList {
	type Item = SyntaxResult<JsVariableDeclaration>;
	type IntoIter = AstSeparatedListNodesIterator<JsVariableDeclaration>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &JsVariableDeclarationList {
	type Item = SyntaxResult<JsVariableDeclaration>;
	type IntoIter = AstSeparatedListNodesIterator<JsVariableDeclaration>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Default, Clone)]
pub struct TsEnumMemberList {
	syntax_list: SyntaxList,
}
impl AstList for TsEnumMemberList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_ENUM_MEMBER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<TsEnumMemberList> {
		if Self::can_cast(syntax.kind()) {
			Some(TsEnumMemberList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstNodeList<TsEnumMember> for TsEnumMemberList {}
impl Debug for TsEnumMemberList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}
impl IntoIterator for &TsEnumMemberList {
	type Item = TsEnumMember;
	type IntoIter = AstNodeListIterator<TsEnumMember>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for TsEnumMemberList {
	type Item = TsEnumMember;
	type IntoIter = AstNodeListIterator<TsEnumMember>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Default, Clone)]
pub struct TsObjectMemberList {
	syntax_list: SyntaxList,
}
impl AstList for TsObjectMemberList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_OBJECT_MEMBER_LIST }
	fn cast(syntax: SyntaxNode) -> Option<TsObjectMemberList> {
		if Self::can_cast(syntax.kind()) {
			Some(TsObjectMemberList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstNodeList<TsTypeElement> for TsObjectMemberList {}
impl Debug for TsObjectMemberList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}
impl IntoIterator for &TsObjectMemberList {
	type Item = TsTypeElement;
	type IntoIter = AstNodeListIterator<TsTypeElement>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for TsObjectMemberList {
	type Item = TsTypeElement;
	type IntoIter = AstNodeListIterator<TsTypeElement>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Default, Clone)]
pub struct TsTypeArgList {
	syntax_list: SyntaxList,
}
impl AstList for TsTypeArgList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_ARG_LIST }
	fn cast(syntax: SyntaxNode) -> Option<TsTypeArgList> {
		if Self::can_cast(syntax.kind()) {
			Some(TsTypeArgList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<TsType> for TsTypeArgList {}
impl Debug for TsTypeArgList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.elements()).finish()
	}
}
impl IntoIterator for TsTypeArgList {
	type Item = SyntaxResult<TsType>;
	type IntoIter = AstSeparatedListNodesIterator<TsType>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsTypeArgList {
	type Item = SyntaxResult<TsType>;
	type IntoIter = AstSeparatedListNodesIterator<TsType>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Default, Clone)]
pub struct TsTypeList {
	syntax_list: SyntaxList,
}
impl AstList for TsTypeList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_LIST }
	fn cast(syntax: SyntaxNode) -> Option<TsTypeList> {
		if Self::can_cast(syntax.kind()) {
			Some(TsTypeList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<TsExprWithTypeArgs> for TsTypeList {}
impl Debug for TsTypeList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.elements()).finish()
	}
}
impl IntoIterator for TsTypeList {
	type Item = SyntaxResult<TsExprWithTypeArgs>;
	type IntoIter = AstSeparatedListNodesIterator<TsExprWithTypeArgs>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsTypeList {
	type Item = SyntaxResult<TsExprWithTypeArgs>;
	type IntoIter = AstSeparatedListNodesIterator<TsExprWithTypeArgs>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
#[derive(Default, Clone)]
pub struct TsTypeParamList {
	syntax_list: SyntaxList,
}
impl AstList for TsTypeParamList {
	fn syntax_list(&self) -> &SyntaxList { &self.syntax_list }
	fn can_cast(kind: SyntaxKind) -> bool { kind == TS_TYPE_PARAM_LIST }
	fn cast(syntax: SyntaxNode) -> Option<TsTypeParamList> {
		if Self::can_cast(syntax.kind()) {
			Some(TsTypeParamList {
				syntax_list: syntax.into_list(),
			})
		} else {
			None
		}
	}
}
impl AstSeparatedList<TsTypeParam> for TsTypeParamList {}
impl Debug for TsTypeParamList {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.elements()).finish()
	}
}
impl IntoIterator for TsTypeParamList {
	type Item = SyntaxResult<TsTypeParam>;
	type IntoIter = AstSeparatedListNodesIterator<TsTypeParam>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl IntoIterator for &TsTypeParamList {
	type Item = SyntaxResult<TsTypeParam>;
	type IntoIter = AstSeparatedListNodesIterator<TsTypeParam>;
	fn into_iter(self) -> Self::IntoIter { self.iter() }
}
pub struct DebugSyntaxElement(pub(crate) SyntaxElement);
impl Debug for DebugSyntaxElement {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match &self.0 {
			NodeOrToken::Node(node) => match node.kind() {
				CALL_EXPR => std::fmt::Debug::fmt(&CallExpr::cast(node.clone()).unwrap(), f),
				EXPORT_DECL => std::fmt::Debug::fmt(&ExportDecl::cast(node.clone()).unwrap(), f),
				EXPORT_DEFAULT_DECL => {
					std::fmt::Debug::fmt(&ExportDefaultDecl::cast(node.clone()).unwrap(), f)
				}
				EXPORT_DEFAULT_EXPR => {
					std::fmt::Debug::fmt(&ExportDefaultExpr::cast(node.clone()).unwrap(), f)
				}
				EXPORT_NAMED => std::fmt::Debug::fmt(&ExportNamed::cast(node.clone()).unwrap(), f),
				EXPORT_NAMED_SPECIFIER_LIST => {
					std::fmt::Debug::fmt(&ExportNamedSpecifierList::cast(node.clone()).unwrap(), f)
				}
				EXPORT_WILDCARD => {
					std::fmt::Debug::fmt(&ExportWildcard::cast(node.clone()).unwrap(), f)
				}
				FOR_STMT => std::fmt::Debug::fmt(&ForStmt::cast(node.clone()).unwrap(), f),
				FOR_STMT_TEST => std::fmt::Debug::fmt(&ForStmtTest::cast(node.clone()).unwrap(), f),
				FOR_STMT_UPDATE => {
					std::fmt::Debug::fmt(&ForStmtUpdate::cast(node.clone()).unwrap(), f)
				}
				IDENT => std::fmt::Debug::fmt(&Ident::cast(node.clone()).unwrap(), f),
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
				JS_FOR_VARIABLE_DECLARATION => {
					std::fmt::Debug::fmt(&JsForVariableDeclaration::cast(node.clone()).unwrap(), f)
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
				JS_INITIALIZER_CLAUSE => {
					std::fmt::Debug::fmt(&JsInitializerClause::cast(node.clone()).unwrap(), f)
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
				JS_MODIFIER => std::fmt::Debug::fmt(&JsModifier::cast(node.clone()).unwrap(), f),
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
				JS_UNKNOWN_MODIFIER => {
					std::fmt::Debug::fmt(&JsUnknownModifier::cast(node.clone()).unwrap(), f)
				}
				JS_UNKNOWN_NAMED_IMPORT_SPECIFIER => std::fmt::Debug::fmt(
					&JsUnknownNamedImportSpecifier::cast(node.clone()).unwrap(),
					f,
				),
				JS_UNKNOWN_STATEMENT => {
					std::fmt::Debug::fmt(&JsUnknownStatement::cast(node.clone()).unwrap(), f)
				}
				JS_VARIABLE_DECLARATION => {
					std::fmt::Debug::fmt(&JsVariableDeclaration::cast(node.clone()).unwrap(), f)
				}
				JS_VARIABLE_DECLARATION_LIST => {
					std::fmt::Debug::fmt(&JsVariableDeclarationList::cast(node.clone()).unwrap(), f)
				}
				JS_VARIABLE_DECLARATIONS => {
					std::fmt::Debug::fmt(&JsVariableDeclarations::cast(node.clone()).unwrap(), f)
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
				JS_YIELD_EXPRESSION => {
					std::fmt::Debug::fmt(&JsYieldExpression::cast(node.clone()).unwrap(), f)
				}
				NEW_EXPR => std::fmt::Debug::fmt(&NewExpr::cast(node.clone()).unwrap(), f),
				NEW_TARGET => std::fmt::Debug::fmt(&NewTarget::cast(node.clone()).unwrap(), f),
				SPECIFIER => std::fmt::Debug::fmt(&Specifier::cast(node.clone()).unwrap(), f),
				TEMPLATE => std::fmt::Debug::fmt(&Template::cast(node.clone()).unwrap(), f),
				TS_ACCESSIBILITY => {
					std::fmt::Debug::fmt(&TsAccessibility::cast(node.clone()).unwrap(), f)
				}
				TS_ANY => std::fmt::Debug::fmt(&TsAny::cast(node.clone()).unwrap(), f),
				TS_ARRAY => std::fmt::Debug::fmt(&TsArray::cast(node.clone()).unwrap(), f),
				TS_ASSERTION => std::fmt::Debug::fmt(&TsAssertion::cast(node.clone()).unwrap(), f),
				TS_BIGINT => std::fmt::Debug::fmt(&TsBigint::cast(node.clone()).unwrap(), f),
				TS_BOOLEAN => std::fmt::Debug::fmt(&TsBoolean::cast(node.clone()).unwrap(), f),
				TS_CALL_SIGNATURE_DECL => {
					std::fmt::Debug::fmt(&TsCallSignatureDecl::cast(node.clone()).unwrap(), f)
				}
				TS_CONDITIONAL_TYPE => {
					std::fmt::Debug::fmt(&TsConditionalType::cast(node.clone()).unwrap(), f)
				}
				TS_CONST_ASSERTION => {
					std::fmt::Debug::fmt(&TsConstAssertion::cast(node.clone()).unwrap(), f)
				}
				TS_CONSTRAINT => {
					std::fmt::Debug::fmt(&TsConstraint::cast(node.clone()).unwrap(), f)
				}
				TS_CONSTRUCT_SIGNATURE_DECL => {
					std::fmt::Debug::fmt(&TsConstructSignatureDecl::cast(node.clone()).unwrap(), f)
				}
				TS_CONSTRUCTOR_PARAM => {
					std::fmt::Debug::fmt(&TsConstructorParam::cast(node.clone()).unwrap(), f)
				}
				TS_CONSTRUCTOR_TYPE => {
					std::fmt::Debug::fmt(&TsConstructorType::cast(node.clone()).unwrap(), f)
				}
				TS_DEFAULT => std::fmt::Debug::fmt(&TsDefault::cast(node.clone()).unwrap(), f),
				TS_ENUM => std::fmt::Debug::fmt(&TsEnum::cast(node.clone()).unwrap(), f),
				TS_ENUM_MEMBER => {
					std::fmt::Debug::fmt(&TsEnumMember::cast(node.clone()).unwrap(), f)
				}
				TS_ENUM_MEMBER_LIST => {
					std::fmt::Debug::fmt(&TsEnumMemberList::cast(node.clone()).unwrap(), f)
				}
				TS_EXPORT_ASSIGNMENT => {
					std::fmt::Debug::fmt(&TsExportAssignment::cast(node.clone()).unwrap(), f)
				}
				TS_EXPR_WITH_TYPE_ARGS => {
					std::fmt::Debug::fmt(&TsExprWithTypeArgs::cast(node.clone()).unwrap(), f)
				}
				TS_EXTENDS => std::fmt::Debug::fmt(&TsExtends::cast(node.clone()).unwrap(), f),
				TS_EXTERNAL_MODULE_REF => {
					std::fmt::Debug::fmt(&TsExternalModuleRef::cast(node.clone()).unwrap(), f)
				}
				TS_FN_TYPE => std::fmt::Debug::fmt(&TsFnType::cast(node.clone()).unwrap(), f),
				TS_IMPLEMENTS_CLAUSE => {
					std::fmt::Debug::fmt(&TsImplementsClause::cast(node.clone()).unwrap(), f)
				}
				TS_IMPORT => std::fmt::Debug::fmt(&TsImport::cast(node.clone()).unwrap(), f),
				TS_IMPORT_EQUALS_DECL => {
					std::fmt::Debug::fmt(&TsImportEqualsDecl::cast(node.clone()).unwrap(), f)
				}
				TS_INDEX_SIGNATURE => {
					std::fmt::Debug::fmt(&TsIndexSignature::cast(node.clone()).unwrap(), f)
				}
				TS_INDEXED_ARRAY => {
					std::fmt::Debug::fmt(&TsIndexedArray::cast(node.clone()).unwrap(), f)
				}
				TS_INFER => std::fmt::Debug::fmt(&TsInfer::cast(node.clone()).unwrap(), f),
				TS_INTERFACE_DECL => {
					std::fmt::Debug::fmt(&TsInterfaceDecl::cast(node.clone()).unwrap(), f)
				}
				TS_INTERSECTION => {
					std::fmt::Debug::fmt(&TsIntersection::cast(node.clone()).unwrap(), f)
				}
				TS_LITERAL => std::fmt::Debug::fmt(&TsLiteral::cast(node.clone()).unwrap(), f),
				TS_MAPPED_TYPE => {
					std::fmt::Debug::fmt(&TsMappedType::cast(node.clone()).unwrap(), f)
				}
				TS_MAPPED_TYPE_PARAM => {
					std::fmt::Debug::fmt(&TsMappedTypeParam::cast(node.clone()).unwrap(), f)
				}
				TS_MAPPED_TYPE_READONLY => {
					std::fmt::Debug::fmt(&TsMappedTypeReadonly::cast(node.clone()).unwrap(), f)
				}
				TS_METHOD_SIGNATURE => {
					std::fmt::Debug::fmt(&TsMethodSignature::cast(node.clone()).unwrap(), f)
				}
				TS_MODULE_BLOCK => {
					std::fmt::Debug::fmt(&TsModuleBlock::cast(node.clone()).unwrap(), f)
				}
				TS_MODULE_DECL => {
					std::fmt::Debug::fmt(&TsModuleDecl::cast(node.clone()).unwrap(), f)
				}
				TS_NAMESPACE_DECL => {
					std::fmt::Debug::fmt(&TsNamespaceDecl::cast(node.clone()).unwrap(), f)
				}
				TS_NAMESPACE_EXPORT_DECL => {
					std::fmt::Debug::fmt(&TsNamespaceExportDecl::cast(node.clone()).unwrap(), f)
				}
				TS_NEVER => std::fmt::Debug::fmt(&TsNever::cast(node.clone()).unwrap(), f),
				TS_NON_NULL => std::fmt::Debug::fmt(&TsNonNull::cast(node.clone()).unwrap(), f),
				TS_NULL => std::fmt::Debug::fmt(&TsNull::cast(node.clone()).unwrap(), f),
				TS_NUMBER => std::fmt::Debug::fmt(&TsNumber::cast(node.clone()).unwrap(), f),
				TS_OBJECT => std::fmt::Debug::fmt(&TsObject::cast(node.clone()).unwrap(), f),
				TS_OBJECT_MEMBER_LIST => {
					std::fmt::Debug::fmt(&TsObjectMemberList::cast(node.clone()).unwrap(), f)
				}
				TS_OBJECT_TYPE => {
					std::fmt::Debug::fmt(&TsObjectType::cast(node.clone()).unwrap(), f)
				}
				TS_PAREN => std::fmt::Debug::fmt(&TsParen::cast(node.clone()).unwrap(), f),
				TS_PREDICATE => std::fmt::Debug::fmt(&TsPredicate::cast(node.clone()).unwrap(), f),
				TS_PROPERTY_SIGNATURE => {
					std::fmt::Debug::fmt(&TsPropertySignature::cast(node.clone()).unwrap(), f)
				}
				TS_QUALIFIED_PATH => {
					std::fmt::Debug::fmt(&TsQualifiedPath::cast(node.clone()).unwrap(), f)
				}
				TS_STRING => std::fmt::Debug::fmt(&TsString::cast(node.clone()).unwrap(), f),
				TS_SYMBOL => std::fmt::Debug::fmt(&TsSymbol::cast(node.clone()).unwrap(), f),
				TS_TEMPLATE => std::fmt::Debug::fmt(&TsTemplate::cast(node.clone()).unwrap(), f),
				TS_TEMPLATE_ELEMENT => {
					std::fmt::Debug::fmt(&TsTemplateElement::cast(node.clone()).unwrap(), f)
				}
				TS_THIS => std::fmt::Debug::fmt(&TsThis::cast(node.clone()).unwrap(), f),
				TS_TUPLE => std::fmt::Debug::fmt(&TsTuple::cast(node.clone()).unwrap(), f),
				TS_TUPLE_ELEMENT => {
					std::fmt::Debug::fmt(&TsTupleElement::cast(node.clone()).unwrap(), f)
				}
				TS_TYPE_ALIAS_DECL => {
					std::fmt::Debug::fmt(&TsTypeAliasDecl::cast(node.clone()).unwrap(), f)
				}
				TS_TYPE_ANNOTATION => {
					std::fmt::Debug::fmt(&TsTypeAnnotation::cast(node.clone()).unwrap(), f)
				}
				TS_TYPE_ARG_LIST => {
					std::fmt::Debug::fmt(&TsTypeArgList::cast(node.clone()).unwrap(), f)
				}
				TS_TYPE_ARGS => std::fmt::Debug::fmt(&TsTypeArgs::cast(node.clone()).unwrap(), f),
				TS_TYPE_LIST => std::fmt::Debug::fmt(&TsTypeList::cast(node.clone()).unwrap(), f),
				TS_TYPE_NAME => std::fmt::Debug::fmt(&TsTypeName::cast(node.clone()).unwrap(), f),
				TS_TYPE_OPERATOR => {
					std::fmt::Debug::fmt(&TsTypeOperator::cast(node.clone()).unwrap(), f)
				}
				TS_TYPE_PARAM => std::fmt::Debug::fmt(&TsTypeParam::cast(node.clone()).unwrap(), f),
				TS_TYPE_PARAM_LIST => {
					std::fmt::Debug::fmt(&TsTypeParamList::cast(node.clone()).unwrap(), f)
				}
				TS_TYPE_PARAMS => {
					std::fmt::Debug::fmt(&TsTypeParams::cast(node.clone()).unwrap(), f)
				}
				TS_TYPE_REF => std::fmt::Debug::fmt(&TsTypeRef::cast(node.clone()).unwrap(), f),
				TS_UNDEFINED => std::fmt::Debug::fmt(&TsUndefined::cast(node.clone()).unwrap(), f),
				TS_UNION => std::fmt::Debug::fmt(&TsUnion::cast(node.clone()).unwrap(), f),
				TS_UNKNOWN => std::fmt::Debug::fmt(&TsUnknown::cast(node.clone()).unwrap(), f),
				TS_VOID => std::fmt::Debug::fmt(&TsVoid::cast(node.clone()).unwrap(), f),
				_ => std::fmt::Debug::fmt(node, f),
			},
			NodeOrToken::Token(token) => Debug::fmt(token, f),
		}
	}
}
