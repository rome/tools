use crate::{
	empty_element, format_elements, group_elements, space_token, token, FormatElement,
	FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::BreakStmt;

impl ToFormatElement for BreakStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let break_element = formatter.format_token(&self.break_token()?)?;
		let ident = if let Some(ident_token) = self.ident_token() {
			group_elements(format_elements![
				space_token(),
				formatter.format_token(&ident_token)?
			])
		} else {
			empty_element()
		};

		Ok(format_elements![break_element, ident, token(";")])
	}
}
