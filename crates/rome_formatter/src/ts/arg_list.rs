use crate::{
	format_elements, group_elements, join_elements, space_token, FormatElement, FormatResult,
	Formatter, ToFormatElement,
};
use rslint_parser::ast::JsCallArguments;

impl ToFormatElement for JsCallArguments {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let args = formatter.format_separated(self.args())?;

		Ok(group_elements(format_elements![
			formatter.format_token(&self.l_paren_token()?)?,
			join_elements(space_token(), args),
			formatter.format_token(&self.r_paren_token()?)?
		]))
	}
}
