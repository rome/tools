use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnySimpleAssignmentTarget;

impl ToFormatElement for JsAnySimpleAssignmentTarget {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnySimpleAssignmentTarget::JsIdentifierAssignmentTarget(identifier) => {
				identifier.to_format_element(formatter)
			}
			JsAnySimpleAssignmentTarget::JsStaticMemberAssignmentTarget(_) => todo!(),
			JsAnySimpleAssignmentTarget::JsComputedMemberAssignmentTarget(_) => todo!(),
			JsAnySimpleAssignmentTarget::JsParenthesizedAssignmentTarget(_) => todo!(),
		}
	}
}
