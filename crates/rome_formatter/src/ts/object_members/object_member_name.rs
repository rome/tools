use rslint_parser::ast::JsAnyObjectMemberName;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for JsAnyObjectMemberName {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyObjectMemberName::JsComputedMemberName(_) => todo!(),
			JsAnyObjectMemberName::JsLiteralMemberName(ident) => ident.to_format_element(formatter),
		}
	}
}
