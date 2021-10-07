use rslint_parser::ast::NameRef;

use crate::{ts::format_syntax_token, ToFormatElement};

impl ToFormatElement for NameRef {
	fn to_format_element(&self) -> crate::FormatElement {
		if let Some(name_ref) = self.ident_token() {
			return format_syntax_token(name_ref);
		}
		panic!("What the hell")
	}
}
