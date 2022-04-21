//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyArrayAssignmentPatternElement;
impl Format for JsAnyArrayAssignmentPatternElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAssignmentWithDefault(node) => node.format(formatter),
            Self::JsAnyAssignmentPattern(node) => node.format(formatter),
            Self::JsArrayAssignmentPatternRestElement(node) => node.format(formatter),
            Self::JsArrayHole(node) => node.format(formatter),
        }
    }
}
