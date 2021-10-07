use crate::{format_tokens, FormatToken, FormatValue};
use rslint_parser::ast::Name;

impl FormatValue for Name {
	fn format(&self) -> FormatToken {
		format_tokens!(self.ident_token().unwrap().text().as_str())
	}
}
