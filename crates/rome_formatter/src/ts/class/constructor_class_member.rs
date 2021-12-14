use crate::{
	format_elements, group_elements, join_elements, soft_line_break_or_space, space_token,
	FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
	JsAnyConstructorParameter, JsConstructorClassMember, JsConstructorParameters,
};

impl ToFormatElement for JsConstructorClassMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.name()?)?,
			formatter.format_node(self.parameters()?)?,
			space_token(),
			formatter.format_node(self.body()?)?
		])
	}
}

impl ToFormatElement for JsConstructorParameters {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let l_bracket = formatter.format_token(&self.l_paren_token()?)?;
		let params = formatter.format_separated(self.parameters())?;
		let r_bracket = formatter.format_token(&self.r_paren_token()?)?;

		Ok(format_elements![group_elements(format_elements![
			l_bracket,
			join_elements(soft_line_break_or_space(), params),
			r_bracket
		])])
	}
}

impl ToFormatElement for JsAnyConstructorParameter {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyConstructorParameter::TsConstructorParam(_) => todo!(),
			JsAnyConstructorParameter::JsBindingPatternWithDefault(binding) => {
				binding.to_format_element(formatter)
			}
			JsAnyConstructorParameter::JsAnyBindingPattern(binding) => {
				binding.to_format_element(formatter)
			}
		}
	}
}
