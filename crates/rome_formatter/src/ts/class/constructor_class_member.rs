use crate::{
	format_elements, group_elements, join_elements, soft_line_break_or_space, space_token, token,
	FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
	JsAnyConstructorMemberName, JsAnyConstructorParameter, JsConstructorClassMember,
	JsConstructorParameterList,
};

impl ToFormatElement for JsConstructorClassMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.name()?)?,
			formatter.format_node(self.parameter_list()?)?,
			space_token(),
			formatter.format_node(self.body()?)?
		])
	}
}

impl ToFormatElement for JsAnyConstructorMemberName {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyConstructorMemberName::JsStringLiteral(literal) => {
				literal.to_format_element(formatter)
			}
			JsAnyConstructorMemberName::JsStaticMemberName(name) => {
				name.to_format_element(formatter)
			}
		}
	}
}

impl ToFormatElement for JsConstructorParameterList {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let l_bracket = formatter.format_token(&self.l_paren_token()?)?;
		let params = formatter.format_nodes(self.parameters())?;
		let r_bracket = formatter.format_token(&self.r_paren_token()?)?;

		Ok(format_elements![group_elements(format_elements![
			l_bracket,
			join_elements(
				format_elements![token(","), soft_line_break_or_space()],
				params
			),
			r_bracket
		])])
	}
}

impl ToFormatElement for JsAnyConstructorParameter {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyConstructorParameter::TsConstructorParam(_) => todo!(),
			JsAnyConstructorParameter::Pattern(pattern) => pattern.to_format_element(formatter),
		}
	}
}
