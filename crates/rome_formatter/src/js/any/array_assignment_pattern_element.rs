//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyArrayAssignmentPatternElement;
impl ToFormatElement for JsAnyArrayAssignmentPatternElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAssignmentWithDefault(node) => node.format(formatter),
            Self::JsAnyAssignmentPattern(node) => node.format(formatter),
            Self::JsArrayAssignmentPatternRestElement(node) => node.format(formatter),
            Self::JsArrayHole(node) => node.format(formatter),
            Self::JsUnknownAssignment(node) => node.format(formatter),
        }
    }
}
