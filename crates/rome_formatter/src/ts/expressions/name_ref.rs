use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::NameRef;

impl ToFormatElement for NameRef {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		formatter.format_token(&self.ident_token()?)
	}
}
