//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyAssignmentPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyAssignmentPattern;
impl FormatRule<JsAnyAssignmentPattern> for FormatJsAnyAssignmentPattern {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyAssignmentPattern, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyAssignmentPattern::JsAnyAssignment(node) => node.format().fmt(f),
            JsAnyAssignmentPattern::JsArrayAssignmentPattern(node) => node.format().fmt(f),
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(node) => node.format().fmt(f),
        }
    }
}
