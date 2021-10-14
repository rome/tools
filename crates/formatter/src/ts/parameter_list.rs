use crate::{
	format_elements, join_elements, space_token, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::ParameterList;

impl ToFormatElement for ParameterList {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let param_tokens = formatter.format_children(self.parameters())?;

		Some(format_elements![
			formatter.format_token(&self.l_paren_token()?)?,
			join_elements(format_elements!(token(","), space_token()), param_tokens,),
			formatter.format_token(&self.r_paren_token()?)?
		])
	}
}
