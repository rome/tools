//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectAssignmentPatternMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectAssignmentPatternMember;
impl FormatRule<JsAnyObjectAssignmentPatternMember> for FormatJsAnyObjectAssignmentPatternMember {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyObjectAssignmentPatternMember,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                node,
            ) => formatted![formatter, [node.format()]],
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
