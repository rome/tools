use crate::{format_tokens, FormatToken, ListToken};
use syntax::{
	ast::{self, AstNode, FormalParameters},
	SyntaxNode,
};

pub fn format(node: FormalParameters) -> FormatToken {
	// if let None = node.inner_children() {
	// 	return format_tokens!("()");
	// }
	// let formal_parameters = ast::FormalParameters::cast(node).unwrap();
	let params = node.inner_children();
	let mut tokens = vec![];
	for param in params {
		tokens.push(format_tokens!(param.value().unwrap().to_string().as_str()))
	}
	// let group = GroupToken::new(tokens);
	format_tokens!(ListToken::join(
		format_tokens!(",", FormatToken::Space),
		tokens
	))
}
