use crate::{
	format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space,
	FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsAnyParameter, JsParameters, JsRestParameter};

impl ToFormatElement for JsParameters {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let param_tokens = formatter.format_separated(self.items())?;

		Ok(group_elements(format_elements![
			formatter.format_token(&self.l_paren_token()?)?,
			soft_indent(join_elements(soft_line_break_or_space(), param_tokens,),),
			formatter.format_token(&self.r_paren_token()?)?
		]))
	}
}

impl ToFormatElement for JsAnyParameter {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyParameter::JsAnyBindingPattern(binding) => binding.to_format_element(formatter),
			JsAnyParameter::JsBindingPatternWithDefault(binding) => {
				binding.to_format_element(formatter)
			}
			JsAnyParameter::JsRestParameter(binding) => binding.to_format_element(formatter),
		}
	}
}

impl ToFormatElement for JsRestParameter {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.dotdotdot_token()?)?,
			formatter.format_node(self.binding()?)?
		])
	}
}
