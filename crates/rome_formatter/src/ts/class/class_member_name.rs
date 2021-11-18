use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyClassMemberName;

impl ToFormatElement for JsAnyClassMemberName {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyClassMemberName::JsLiteralMemberName(name) => name.to_format_element(formatter),
			JsAnyClassMemberName::JsComputedMemberName(_) => todo!(),
			JsAnyClassMemberName::JsPrivateClassMemberName(_) => todo!(),
		}
	}
}
