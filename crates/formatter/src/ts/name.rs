use crate::{ts::format_syntax_token, FormatElement, FormatValue};
use rslint_parser::ast::Name;

impl FormatValue for Name {
	fn format(&self) -> FormatElement {
		format_syntax_token(self.ident_token().unwrap())
	}
}
