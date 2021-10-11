use crate::{token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::{
	ArrayExpr, ArrowExpr, AssignPattern, BlockStmt, Declarator, ExprStmt, FnDecl, Literal, Name,
	NameRef, ParameterList, ReturnStmt, Script, SequenceExpr, SinglePattern, VarDecl,
};
use rslint_parser::{AstNode, AstToken, SyntaxKind, SyntaxNode, SyntaxToken};

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

impl ToFormatElement for SyntaxToken {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		match self.kind() {
			SyntaxKind::STRING => rslint_parser::ast::String::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			_ => token(self.text().as_str()),
		}
	}
}
