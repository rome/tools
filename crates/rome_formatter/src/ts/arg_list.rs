use crate::{
	concat_elements, format_elements, group_elements, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::JsCallArguments;

impl ToFormatElement for JsCallArguments {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let l_bracket = formatter.format_token(&self.l_paren_token()?)?;
		let args = concat_elements(formatter.format_separated(self.args())?);
		let r_bracket = formatter.format_token(&self.r_paren_token()?)?;

		Ok(group_elements(format_elements![l_bracket, args, r_bracket]))
	}
}
