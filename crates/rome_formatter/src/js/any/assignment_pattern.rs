//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyAssignmentPattern;
impl ToFormatElement for JsAnyAssignmentPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyAssignment(node) => node.to_format_element(formatter),
            Self::JsArrayAssignmentPattern(node) => node.to_format_element(formatter),
            Self::JsObjectAssignmentPattern(node) => node.to_format_element(formatter),
        }
    }
}
