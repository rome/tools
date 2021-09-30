use crate::{format_tokens, FormatToken};
use syntax::{
	ast::{self, AstNode},
	SyntaxNode,
};

pub fn format(node: SyntaxNode) -> FormatToken {
	let string = ast::StringLiteral::cast(node).unwrap();
	format_tokens!("\"", string.value().unwrap().text(), "\"")
}
