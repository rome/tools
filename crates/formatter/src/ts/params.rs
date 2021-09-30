use crate::{format_tokens, FormatToken, GroupToken, ListToken};
use syntax::{
	ast::{self, AstNode},
	SyntaxNode,
};

pub fn format(node: SyntaxNode) -> FormatToken {
	if let None = node.first_child() {
		return format_tokens!("()");
	}
	let formal_parameters = ast::FormalParameters::cast(node).unwrap();
	let params = formal_parameters.parameters();
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
