//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyObjectAssignmentPatternMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyObjectAssignmentPatternMember;
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
            JsAnyObjectAssignmentPatternMember::JsBogusAssignment(node) => node.format().fmt(f),
        }
    }
}
