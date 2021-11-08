use crate::{
	format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::WithStmt;

impl ToFormatElement for WithStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let with_token = formatter.format_token(&self.with_token()?)?;
		let condition = formatter.format_node(self.condition()?)?;
		let cons = formatter.format_node(self.cons()?)?;

		Ok(format_elements![
			with_token,
			space_token(),
			condition,
			space_token(),
			cons
		])
	}
}
