use crate::{
	format_tokens, ts::format_syntax_token, FormatToken, FormatValue, GroupToken, ListToken,
};
use parser::SyntaxKind;
use syntax::{
	ast::{self, AstNode},
	NodeOrToken, SyntaxNode,
};

pub fn format_node(node: SyntaxNode) -> FormatToken {
	let array = ast::Array::cast(node).unwrap();
	let elements = array.elements();
	let mut tokens = vec![];

	for element in elements {
		let token = match element {
			NodeOrToken::Node(node) => Some(node.format()),
			NodeOrToken::Token(token) => {
				// this is an hack until we figure out a better API to retrieve
				// token elements that are really part of an array
				match token.kind() {
					SyntaxKind::True | SyntaxKind::False | SyntaxKind::Number => {
						Some(format_syntax_token(token))
					}

					_ => None,
				}
			}
		};
		if let Some(token) = token {
			tokens.push(token);
		}
	}
	let separator = format_tokens!(",", FormatToken::Space);
	format_tokens!(
		"[",
		FormatToken::indent(GroupToken::new(ListToken::join(separator, tokens))),
		",",
		"]",
	)
}
