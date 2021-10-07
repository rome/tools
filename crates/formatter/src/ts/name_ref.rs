use rslint_parser::ast::NameRef;

use crate::{ts::format_syntax_token, FormatValue};

impl FormatValue for NameRef {
	fn format(&self) -> crate::FormatElement {
		if let Some(name_ref) = self.ident_token() {
			return format_syntax_token(name_ref);
		}
		panic!("What the hell")
	}
}
