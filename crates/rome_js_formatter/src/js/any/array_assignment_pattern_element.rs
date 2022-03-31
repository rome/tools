//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyArrayAssignmentPatternElement;
impl ToFormatElement for JsAnyArrayAssignmentPatternElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAssignmentWithDefault(node) => node.to_format_element(formatter),
            Self::JsAnyAssignmentPattern(node) => node.to_format_element(formatter),
            Self::JsArrayAssignmentPatternRestElement(node) => node.to_format_element(formatter),
            Self::JsArrayHole(node) => node.to_format_element(formatter),
            Self::JsUnknownAssignment(node) => node.to_format_element(formatter),
        }
    }
}
