use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::CallExpr;

impl ToFormatElement for CallExpr {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let name = formatter.format_node(self.callee()?)?;
		let arguments = formatter.format_node(self.arguments()?)?;

		Ok(format_elements![name, arguments])
	}
}
