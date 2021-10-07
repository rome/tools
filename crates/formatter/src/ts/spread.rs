use crate::{format_tokens, ts::format_syntax_token, FormatToken, FormatValue};
use rslint_parser::ast::SpreadElement;

impl FormatValue for SpreadElement {
	fn format(&self) -> FormatToken {
		let dotdotdot_token = self.dotdotdot_token().unwrap();
		let child = self.element().unwrap();

		format_tokens!(format_syntax_token(dotdotdot_token), child.format())
	}
}
