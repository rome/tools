use crate::{
	format_elements, group_elements, join_elements, soft_line_break_or_space, space_token, token,
	FormatElement, FormatError, Formatter, ToFormatElement,
};
use rslint_parser::ast::{Constructor, ConstructorParamOrPat, ConstructorParameters};

impl ToFormatElement for Constructor {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let constructor_token = formatter.format_node(self.name()?)?;
		let params = formatter.format_node(self.parameters()?)?;
		let body = formatter.format_node(self.body()?)?;
		Ok(format_elements![
			constructor_token,
			params,
			space_token(),
			body
		])
	}
}

impl ToFormatElement for ConstructorParameters {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
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

impl ToFormatElement for ConstructorParamOrPat {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		match self {
			ConstructorParamOrPat::TsConstructorParam(_) => todo!(),
			ConstructorParamOrPat::Pattern(pattern) => pattern.to_format_element(formatter),
		}
	}
}
