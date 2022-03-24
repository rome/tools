//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyAssignmentPattern;
impl ToFormatElement for JsAnyAssignmentPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyAssignment(node) => node.format(formatter),
            Self::JsArrayAssignmentPattern(node) => node.format(formatter),
            Self::JsObjectAssignmentPattern(node) => node.format(formatter),
        }
    }
}
