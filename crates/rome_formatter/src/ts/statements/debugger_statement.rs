use crate::{format_elements, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::DebuggerStmt;

impl ToFormatElement for DebuggerStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		Some(format_elements![
			formatter.format_token(&self.debugger_token()?)?,
			token(";")
		])
	}
}
