use crate::{
	format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, space_token, token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
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
		let params = formatter.format_separated(self.parameters())?;

		Ok(group_elements(format_elements!(
			formatter.format_token(&self.l_paren_token()?)?,
			soft_indent(format_elements![
				join_elements(soft_line_break_or_space(), params),
				if_group_breaks(token(",")),
			]),
			formatter.format_token(&self.r_paren_token()?)?,
		)))
	}
}

impl ToFormatElement for JsAnyConstructorParameter {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyConstructorParameter::TsConstructorParam(_) => todo!(),
			JsAnyConstructorParameter::JsParameter(parameter) => {
				parameter.to_format_element(formatter)
			}
			JsAnyConstructorParameter::JsUnknownParameter(_) => todo!(),
		}
	}
}
