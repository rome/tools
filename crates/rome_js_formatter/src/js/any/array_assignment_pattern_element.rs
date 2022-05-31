//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrayAssignmentPatternElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrayAssignmentPatternElement;
impl FormatRule<JsAnyArrayAssignmentPatternElement> for FormatJsAnyArrayAssignmentPatternElement {
    type Context = JsFormatContext;
    fn format(node: &JsAnyArrayAssignmentPatternElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(node) => {
                node.format().format(f)
            }
            JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(node) => {
                node.format().format(f)
            }
            JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(node) => {
                node.format().format(f)
            }
            JsAnyArrayAssignmentPatternElement::JsArrayHole(node) => node.format().format(f),
        }
    }
}
