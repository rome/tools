use crate::{format_elements, space_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::WithStmt;

impl ToFormatElement for WithStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let with_token = formatter.format_token(&self.with_token()?)?;
		let condition = formatter.format_node(self.condition()?)?;
		let cons = formatter.format_node(self.cons()?)?;

		Some(format_elements![
			with_token,
			space_token(),
			condition,
			space_token(),
			cons
		])
	}
}
