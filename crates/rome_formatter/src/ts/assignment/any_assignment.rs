use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyAssignment;

impl ToFormatElement for JsAnyAssignment {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyAssignment::JsIdentifierAssignment(identifier) => {
				identifier.to_format_element(formatter)
			}
			JsAnyAssignment::JsStaticMemberAssignment(_) => todo!(),
			JsAnyAssignment::JsComputedMemberAssignment(_) => todo!(),
			JsAnyAssignment::JsParenthesizedAssignment(_) => todo!(),
			JsAnyAssignment::JsUnknownAssignment(_) => todo!(),
		}
	}
}
