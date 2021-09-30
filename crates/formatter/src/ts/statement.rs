use crate::{format_tokens, FormatToken, FormatValue, LineToken};
use syntax::{
	ast::{self, AstNode},
	SyntaxNode,
};

pub fn format(node: SyntaxNode) -> FormatToken {
	let statement_block = ast::StatementBlock::cast(node).unwrap();
	if let Some(return_statement) = statement_block.return_statement() {
		let return_value = return_statement.return_value().unwrap();

		return format_tokens!(
			"return",
			FormatToken::Space,
			return_value.syntax().format(),
			";",
			LineToken::hard()
		);
	}

	format_tokens!("")
}
