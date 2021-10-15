use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::LiteralProp;

impl ToFormatElement for LiteralProp {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		formatter.format_token(&self.colon_token()?)
	}
}
