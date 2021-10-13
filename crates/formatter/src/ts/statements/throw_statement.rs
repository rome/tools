use crate::{format_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::ThrowStmt;

impl ToFormatElement for ThrowStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let throw_token =
			formatter.format_token(&self.throw_token().expect("throw token is missing"));
		let exception = formatter.format_node(self.exception().expect("exception is missing"));
		format_elements![throw_token, space_token(), exception, token(";")]
	}
}
