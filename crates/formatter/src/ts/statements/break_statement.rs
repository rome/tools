use rslint_parser::ast::BreakStmt;

use crate::{format_elements, group_elements, space_token, token, FormatElement, ToFormatElement};

impl ToFormatElement for BreakStmt {
	fn to_format_element(&self, formatter: &crate::Formatter) -> crate::FormatElement {
		let break_element =
			formatter.format_token(&self.break_token().expect("Break token missing"));
		let ident = if let Some(ident_token) = self.ident_token() {
			group_elements(format_elements![
				space_token(),
				formatter.format_token(&ident_token)
			])
		} else {
			FormatElement::Empty
		};
		let semicolon = if let Some(semicolon) = self.semicolon_token() {
			formatter.format_token(&semicolon)
		} else {
			token(";")
		};

		format_elements![break_element, ident, semicolon]
	}
}
