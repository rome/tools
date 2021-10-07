use rslint_parser::ast::NameRef;

use crate::{format_tokens, FormatValue};

impl FormatValue for NameRef {
	fn format(&self) -> crate::FormatToken {
		if let Some(name_ref) = self.ident_token() {
			return format_tokens!(name_ref.text().as_str());
		}
		panic!("What the hell")
	}
}
