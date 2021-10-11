use crate::{token, FormatElement, Formatter, ToFormatElement};
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
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		match self.kind() {
			SyntaxKind::ARRAY_EXPR => ArrayExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::ARROW_EXPR => ArrowExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::ASSIGN_PATTERN => AssignPattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::LITERAL => Literal::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::NAME => Name::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::NAME_REF => NameRef::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::PARAMETER_LIST => ParameterList::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SCRIPT => Script::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SINGLE_PATTERN => SinglePattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SPREAD_ELEMENT => SinglePattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::VAR_DECL => VarDecl::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::DECLARATOR => Declarator::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FN_DECL => FnDecl::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SEQUENCE_EXPR => SequenceExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::BLOCK_STMT => BlockStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::EXPR_STMT => ExprStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::RETURN_STMT => ReturnStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			_ => todo!(
				"Implement formatting for the {:?} syntax kind.",
				self.kind()
			),
		}
	}
}
