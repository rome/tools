use rslint_parser::ast::SpreadElement;

use crate::{format_tokens, FormatToken, FormatValue};

impl FormatValue for SpreadElement {
	fn format(&self) -> FormatToken {
		let dotdotdot_token = self.dotdotdot_token().unwrap();
		let child = self.element().unwrap();
		format_tokens!(dotdotdot_token.text().as_str(), child.format())
	}
}
