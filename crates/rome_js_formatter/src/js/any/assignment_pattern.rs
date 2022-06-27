//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyAssignmentPattern;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyAssignmentPattern;
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
