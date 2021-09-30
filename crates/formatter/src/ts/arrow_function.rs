use crate::{format_tokens, ts::params, FormatToken, FormatValue, ListToken};
use syntax::{
	ast::{self, AstNode},
	SyntaxNode,
};

pub fn format(node: SyntaxNode) -> FormatToken {
	let arrow_function = ast::ArrowFunction::cast(node).unwrap();
	let mut tokens: Vec<FormatToken> = vec![];
	tokens.push(format_tokens!("("));
	if let Some(formal_parameters) = arrow_function.parameters() {
		tokens.push(formal_parameters.syntax().format());
	}
	tokens.push(format_tokens!(")"));
	tokens.push(format_tokens!(" => "));
	let body = arrow_function.body();

	if let Some(body) = body {
		let s = body.syntax();
		tokens.push(s.format());
	} else {
		let last_child = arrow_function.syntax().last_token().unwrap();
		// the last child should be the value
		tokens.push(format_tokens!(last_child.text()));
	}

	FormatToken::concat(tokens)
}
