use crate::{
	format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsEqualValueClause;

impl ToFormatElement for JsEqualValueClause {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.eq_token()?)?,
			space_token(),
			formatter.format_node(self.expression()?)?
		])
	}
}
