use rslint_parser::ast::BlockStmt;

use crate::{
	format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space, token,
	FormatElement, ToFormatElement,
};

impl ToFormatElement for BlockStmt {
	fn to_format_element(&self) -> FormatElement {
		let body: Vec<_> = self.stmts().map(|stmt| stmt.to_format_element()).collect();
		if !body.is_empty() {
			return group_elements(format_elements![
				token("{"),
				soft_indent(join_elements(soft_line_break_or_space(), body)),
				token("}")
			]);
		}
		format_elements![token("{"), token("}")]
	}
}
