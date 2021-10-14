use crate::{format_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::LabelledStmt;

impl ToFormatElement for LabelledStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let label = formatter.format_node(self.label()?)?;
		let statement = formatter.format_node(self.stmt()?)?;

		Some(format_elements![
			label,
			token(":"),
			space_token(),
			statement
		])
	}
}
