use crate::{format_elements, space_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::WhileStmt;

impl ToFormatElement for WhileStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let while_token = formatter.format_token(&self.while_token()?)?;
		let condition = formatter.format_node(self.condition()?)?;
		let cons = formatter.format_node(self.cons()?)?;

		Some(format_elements![
			while_token,
			space_token(),
			condition,
			space_token(),
			cons
		])
	}
}
