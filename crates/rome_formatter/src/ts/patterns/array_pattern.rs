use crate::{
	format_elements, group_elements, join_elements, token, FormatElement, FormatError, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::ArrayPattern;

impl ToFormatElement for ArrayPattern {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let l_bracket = formatter.format_token(&self.l_brack_token()?)?;
		let elements = formatter.format_nodes(self.elements())?;
		let r_bracket = formatter.format_token(&self.r_brack_token()?)?;

		Ok(format_elements![group_elements(format_elements![
			l_bracket,
			join_elements(token(", "), elements),
			r_bracket
		])])
	}
}
