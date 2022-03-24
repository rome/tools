//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyAssignment;
impl ToFormatElement for JsAnyAssignment {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsIdentifierAssignment(node) => node.format(formatter),
            Self::JsStaticMemberAssignment(node) => node.format(formatter),
            Self::JsComputedMemberAssignment(node) => node.format(formatter),
            Self::JsParenthesizedAssignment(node) => node.format(formatter),
            Self::TsNonNullAssertionAssignment(node) => node.format(formatter),
            Self::TsAsAssignment(node) => node.format(formatter),
            Self::TsTypeAssertionAssignment(node) => node.format(formatter),
            Self::JsUnknownAssignment(node) => node.format(formatter),
        }
    }
}
