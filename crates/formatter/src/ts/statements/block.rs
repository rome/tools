use rslint_parser::ast::BlockStmt;

use crate::{
	format_elements, hard_line_break, indent, join_elements, token, FormatElement, Formatter,
	ToFormatElement,
};

impl ToFormatElement for BlockStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let stmts = self.stmts().map(|stmt| formatter.format_node(stmt));

		format_elements![
			token("{"),
			indent(join_elements(hard_line_break(), stmts)),
			token("}")
		]
	}
}
