//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyAssignmentPattern;
use crate::prelude::*;
use rome_js_syntax::JsAnyAssignmentPattern;
impl FormatRule<JsAnyAssignmentPattern> for FormatJsAnyAssignmentPattern {
    fn format(node: &JsAnyAssignmentPattern, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyAssignmentPattern::JsAnyAssignment(node) => formatted![formatter, node.format()],
            JsAnyAssignmentPattern::JsArrayAssignmentPattern(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
