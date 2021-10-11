use crate::ts::statements::format_statements;
use crate::{
	block_indent, format_elements, hard_line_break, token, FormatElement, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::{BlockStmt, IfStmt};
use rslint_parser::AstNode;

impl ToFormatElement for BlockStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let stmts = format_statements(self.stmts(), formatter);

		// Formatting of curly braces for an:
		// * empty block: same line `{}`,
		// * empty block that is the 'cons' or 'alt' of an if statement: two lines `{\n}`
		// * non empty block: put each stmt on its own line: `{\nstmt1;\nstmt2;\n}`
		let body = if stmts.is_empty() && self.syntax().parent().and_then(IfStmt::cast).is_some() {
			hard_line_break()
		} else {
			block_indent(stmts)
		};

		Some(format_elements![token("{"), body, token("}")])
	}
}
