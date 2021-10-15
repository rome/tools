use crate::{format_elements, FormatElement, Formatter, ToFormatElement};
use rslint_parser::{ast::CallExpr, AstNode};

impl ToFormatElement for CallExpr {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let name = formatter.format_node(self.callee()?)?;
		let arguments = formatter.format_node(self.arguments()?)?;
		Some(format_elements![name, arguments])
	}
}
