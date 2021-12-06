use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyAssignmentPattern;

impl ToFormatElement for JsAnyAssignmentPattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyAssignmentPattern::JsAnyAssignment(assignment) => {
				assignment.to_format_element(formatter)
			}
			JsAnyAssignmentPattern::JsArrayAssignmentPattern(_) => todo!(),
			JsAnyAssignmentPattern::JsObjectAssignmentPattern(_) => todo!(),
		}
	}
}
