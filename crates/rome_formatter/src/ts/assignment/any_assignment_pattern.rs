use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyAssignmentPattern;

impl ToFormatElement for JsAnyAssignmentPattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyAssignmentPattern::JsAnyAssignment(assignment) => {
				assignment.to_format_element(formatter)
			}
			JsAnyAssignmentPattern::JsArrayAssignmentPattern(array_assignment_pattern) => {
				array_assignment_pattern.to_format_element(formatter)
			}
			JsAnyAssignmentPattern::JsObjectAssignmentPattern(object_assignment_pattern) => {
				object_assignment_pattern.to_format_element(formatter)
			}
		}
	}
}
