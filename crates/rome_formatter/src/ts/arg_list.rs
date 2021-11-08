use crate::{
	format_elements, group_elements, join_elements, token, FormatElement, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::ArgList;

impl ToFormatElement for ArgList {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let l_bracket = formatter.format_token(&self.l_paren_token()?)?;
		let args = formatter.format_nodes(self.args())?;
		let r_bracket = formatter.format_token(&self.r_paren_token()?)?;

		Some(group_elements(format_elements![
			l_bracket,
			join_elements(token(", "), args),
			r_bracket
		]))
	}
}
