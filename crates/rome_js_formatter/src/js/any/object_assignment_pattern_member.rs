//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectAssignmentPatternMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectAssignmentPatternMember;
impl FormatRule<JsAnyObjectAssignmentPatternMember> for FormatJsAnyObjectAssignmentPatternMember {
    type Context = JsFormatContext;
    fn format(node: &JsAnyObjectAssignmentPatternMember, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                node,
            ) => node.format().format(f),
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(node) => {
                node.format().format(f)
            }
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(node) => {
                node.format().format(f)
            }
            JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(node) => {
                node.format().format(f)
            }
        }
    }
}
