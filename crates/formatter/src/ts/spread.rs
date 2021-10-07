use crate::{format_elements, ts::format_syntax_token, FormatElement, FormatValue};
use rslint_parser::ast::SpreadElement;

impl FormatValue for SpreadElement {
	fn format(&self) -> FormatElement {
		let dotdotdot_token = self.dotdotdot_token().unwrap();
		let child = self.element().unwrap();

		format_elements!(format_syntax_token(dotdotdot_token), child.format())
	}
}
