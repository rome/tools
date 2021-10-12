use rslint_parser::ast::{BlockStmt, IfStmt};
use rslint_parser::AstNode;

use crate::{
	format_elements, hard_line_break, indent, token, FormatElement, Formatter, ToFormatElement,
};

impl ToFormatElement for BlockStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let stmts = formatter.format_statements(self.stmts());

		// Insert an empty line if the block is empty but it's the consequence or alternative of an if statement
		let body = if stmts.is_empty() && self.syntax().parent().and_then(IfStmt::cast).is_some() {
			hard_line_break()
		} else {
			indent(stmts)
		};

		format_elements![token("{"), body, token("}")]
	}
}
