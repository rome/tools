use crate::{format_elements, ts::format_syntax_token, FormatElement, ToFormatElement};
use rslint_parser::ast::SpreadElement;

impl ToFormatElement for SpreadElement {
	fn to_format_element(&self) -> FormatElement {
		let dotdotdot_token = self.dotdotdot_token().unwrap();
		let child = self.element().unwrap();

		format_elements!(
			format_syntax_token(dotdotdot_token),
			child.to_format_element()
		)
	}
}
