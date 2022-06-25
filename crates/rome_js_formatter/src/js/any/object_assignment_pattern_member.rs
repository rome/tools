//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyObjectAssignmentPatternMember;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyObjectAssignmentPatternMember;
impl FormatRule<JsAnyObjectAssignmentPatternMember> for FormatJsAnyObjectAssignmentPatternMember {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &JsAnyObjectAssignmentPatternMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                node,
            ) => node.format().fmt(f),
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(node) => {
                node.format().fmt(f)
            }
            JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(node) => {
                node.format().fmt(f)
            }
            JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(node) => node.format().fmt(f),
        }
    }
}
