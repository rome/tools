use crate::{ts::format_syntax_token, FormatToken, FormatValue};
use rslint_parser::ast::Name;

impl FormatValue for Name {
	fn format(&self) -> FormatToken {
		format_syntax_token(self.ident_token().unwrap())
	}
}
