use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyAssignmentTarget;

impl ToFormatElement for JsAnyAssignmentTarget {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyAssignmentTarget::JsAnySimpleAssignmentTarget(simple) => {
				simple.to_format_element(formatter)
			}
			JsAnyAssignmentTarget::JsArrayAssignmentTarget(_) => todo!(),
			JsAnyAssignmentTarget::JsObjectAssignmentTarget(_) => todo!(),
		}
	}
}
