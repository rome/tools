use crate::{
	format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space, token,
	FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsAnyParameter, JsParameterList};

impl ToFormatElement for JsParameterList {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let param_tokens = formatter.format_nodes(self.parameters())?;

		Ok(group_elements(format_elements![
			formatter.format_token(&self.l_paren_token()?)?,
			soft_indent(join_elements(
				format_elements![token(","), soft_line_break_or_space()],
				param_tokens,
			),),
			formatter.format_token(&self.r_paren_token()?)?
		]))
	}
}

impl ToFormatElement for JsAnyParameter {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyParameter::Pattern(pattern) => pattern.to_format_element(formatter),
			JsAnyParameter::JsRestParameter(_) => todo!("rest parameter"),
		}
	}
}
