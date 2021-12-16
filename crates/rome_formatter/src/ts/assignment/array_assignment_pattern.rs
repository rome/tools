use crate::{
	format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space,
	FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
	JsAnyArrayAssignmentPatternElement, JsArrayAssignmentPattern,
	JsArrayAssignmentPatternRestElement,
};

impl ToFormatElement for JsArrayAssignmentPattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let elements = formatter.format_separated(self.elements())?;
		Ok(group_elements(format_elements![
			formatter.format_token(&self.l_brack_token()?)?,
			soft_indent(join_elements(soft_line_break_or_space(), elements)),
			formatter.format_token(&self.r_brack_token()?)?,
		]))
	}
}

impl ToFormatElement for JsAnyArrayAssignmentPatternElement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(
				assignment_with_default,
			) => assignment_with_default.to_format_element(formatter),
			JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(any_assignment_pattern) => {
				any_assignment_pattern.to_format_element(formatter)
			}
			JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(
				array_assignment_pattern_rest_element,
			) => array_assignment_pattern_rest_element.to_format_element(formatter),
			JsAnyArrayAssignmentPatternElement::JsArrayHole(array_hole) => {
				array_hole.to_format_element(formatter)
			}
			JsAnyArrayAssignmentPatternElement::JsUnknownAssignment(_unknown_assignment) => {
				todo!()
			}
		}
	}
}

impl ToFormatElement for JsArrayAssignmentPatternRestElement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.dotdotdot_token()?)?,
			formatter.format_node(self.pattern()?)?,
		])
	}
}
