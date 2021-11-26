use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyBinding;

impl ToFormatElement for JsAnyBinding {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyBinding::RestPattern(_) => todo!(),
			JsAnyBinding::AssignPattern(pattern) => pattern.to_format_element(formatter),
			JsAnyBinding::ObjectPattern(_) => todo!(),
			JsAnyBinding::ArrayPattern(array_pattern) => array_pattern.to_format_element(formatter),
			JsAnyBinding::ExprPattern(_) => todo!(),
			JsAnyBinding::JsIdentifierBinding(single) => single.to_format_element(formatter),
			JsAnyBinding::JsUnknownBinding(_) => todo!(),
		}
	}
}
