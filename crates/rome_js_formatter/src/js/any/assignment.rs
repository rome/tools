//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyAssignment;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyAssignment;
impl FormatRule<JsAnyAssignment> for FormatJsAnyAssignment {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyAssignment::JsIdentifierAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::JsStaticMemberAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::JsComputedMemberAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::JsParenthesizedAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::TsNonNullAssertionAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::TsAsAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::TsSatisfiesAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::TsTypeAssertionAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::JsBogusAssignment(node) => node.format().fmt(f),
        }
    }
}
