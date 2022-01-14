use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyAssignment;

impl ToFormatElement for JsAnyAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyAssignment::JsIdentifierAssignment(identifier) => {
                identifier.to_format_element(formatter)
            }
            JsAnyAssignment::JsStaticMemberAssignment(static_member) => {
                static_member.to_format_element(formatter)
            }
            JsAnyAssignment::JsComputedMemberAssignment(computed_member_assignment) => {
                computed_member_assignment.to_format_element(formatter)
            }
            JsAnyAssignment::JsParenthesizedAssignment(parenthesized_assignment) => {
                parenthesized_assignment.to_format_element(formatter)
            }
            JsAnyAssignment::JsUnknownAssignment(_) => todo!(),
        }
    }
}
