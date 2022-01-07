use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{JsAnyFunction, JsFunctionExpression};

impl ToFormatElement for JsFunctionExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		JsAnyFunction::from(self.clone()).to_format_element(formatter)
	}
}
