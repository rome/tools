use crate::{format_tokens, FormatToken, FormatValue};
use syntax::{
	ast::{self, AstNode},
	SyntaxNode,
};

pub fn format_node(node: SyntaxNode) -> FormatToken {
	let return_statement = ast::ReturnStatement::cast(node).unwrap();
	let return_value = return_statement.return_value().unwrap();

	return format_tokens!(
		"return",
		FormatToken::Space,
		return_value.syntax().format(),
		";"
	);
}
