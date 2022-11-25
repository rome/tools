//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyArrayAssignmentPatternElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyArrayAssignmentPatternElement;
impl FormatRule<JsAnyArrayAssignmentPatternElement> for FormatJsAnyArrayAssignmentPatternElement {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &JsAnyArrayAssignmentPatternElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            JsAnyArrayAssignmentPatternElement::JsAssignmentWithDefault(node) => {
                node.format().fmt(f)
            }
            JsAnyArrayAssignmentPatternElement::JsAnyAssignmentPattern(node) => {
                node.format().fmt(f)
            }
            JsAnyArrayAssignmentPatternElement::JsArrayAssignmentPatternRestElement(node) => {
                node.format().fmt(f)
            }
            JsAnyArrayAssignmentPatternElement::JsArrayHole(node) => node.format().fmt(f),
        }
    }
}
