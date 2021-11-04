use crate::{format_elements, FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::SpreadElement;

impl ToFormatElement for SpreadElement {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		Ok(format_elements![
			formatter.format_token(&self.dotdotdot_token()?)?,
			formatter.format_node(self.element()?)?
		])
	}
}
