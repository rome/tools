//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyAssignmentPattern;
impl Format for JsAnyAssignmentPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyAssignment(node) => node.format(formatter),
            Self::JsArrayAssignmentPattern(node) => node.format(formatter),
            Self::JsObjectAssignmentPattern(node) => node.format(formatter),
        }
    }
}
