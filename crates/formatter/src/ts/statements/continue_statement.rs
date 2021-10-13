use crate::{
	empty_element, format_elements, space_token, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::ContinueStmt;

impl ToFormatElement for ContinueStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		// NOTE: rslint parser (upstream) is currently broken https://github.com/rslint/rslint/issues/126
		let ident = self.ident_token().map_or(empty_element(), |ident_token| {
			format_elements![space_token(), formatter.format_node(ident_token)]
		});
		format_elements![token("continue"), ident, token(";")]
	}
}
