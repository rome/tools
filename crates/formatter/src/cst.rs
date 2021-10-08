use crate::{token, FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::{
	ArrayExpr, ArrowExpr, AssignPattern, BlockStmt, Declarator, ExprStmt, FnDecl, Literal, Name,
	NameRef, ParameterList, ReturnStmt, Script, SequenceExpr, SinglePattern, VarDecl,
};
use rslint_parser::{AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

/// Creates a format token representing the exact same text as the syntax token
///
/// # Examples
///
/// ```
///
/// use rome_formatter::{FormatOptions, syntax_token, format_element};
/// use rslint_parser::{SyntaxNode, T};
/// use rslint_rowan::{GreenNode, GreenToken, SmolStr, SyntaxKind, NodeOrToken};
///
/// let node = SyntaxNode::new_root(
///   GreenNode::new(SyntaxKind(1), vec![
///     NodeOrToken::Token(GreenToken::new(SyntaxKind(T![=>].into()), SmolStr::new("=>")))
///   ])
/// );
///
/// let token = node.first_token().unwrap();
/// let element = syntax_token(&token);
///
/// assert_eq!("=>", format_element(&element, FormatOptions::default()).code())
/// ```
pub fn syntax_token(syntax_token: &SyntaxToken) -> FormatElement {
	token(syntax_token.text().as_str())
}

impl ToFormatElement for SyntaxNode {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		match self.kind() {
			SyntaxKind::ARRAY_EXPR => ArrayExpr::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::ARROW_EXPR => ArrowExpr::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::ASSIGN_PATTERN => AssignPattern::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::LITERAL => Literal::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::NAME => Name::cast(self.clone()).unwrap().to_format_element(context),
			SyntaxKind::NAME_REF => NameRef::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::PARAMETER_LIST => ParameterList::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::SCRIPT => Script::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::SINGLE_PATTERN => SinglePattern::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::SPREAD_ELEMENT => SinglePattern::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::VAR_DECL => VarDecl::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::DECLARATOR => Declarator::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::FN_DECL => FnDecl::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::SEQUENCE_EXPR => SequenceExpr::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::BLOCK_STMT => BlockStmt::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::EXPR_STMT => ExprStmt::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			SyntaxKind::RETURN_STMT => ReturnStmt::cast(self.clone())
				.unwrap()
				.to_format_element(context),
			_ => todo!(
				"Implement formatting for the {:?} syntax kind.",
				self.kind()
			),
		}
	}
}
