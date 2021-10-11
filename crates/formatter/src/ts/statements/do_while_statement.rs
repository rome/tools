use crate::{format_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::DoWhileStmt;

impl ToFormatElement for DoWhileStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let do_token = formatter.format_token(&self.do_token()?)?;

		let condition = formatter.format_node(self.condition()?)?;
		let cons = formatter.format_node(self.cons()?)?;
		let while_token = formatter.format_token(&self.while_token()?)?;

		Some(format_elements![
			do_token,
			space_token(),
			cons,
			space_token(),
			while_token,
			space_token(),
			condition,
			token(";")
		])
	}
}
