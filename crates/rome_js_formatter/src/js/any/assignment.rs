//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyAssignment;
use crate::prelude::*;
use rome_js_syntax::JsAnyAssignment;
impl FormatRule<JsAnyAssignment> for FormatJsAnyAssignment {
    type Context = JsFormatContext;
    fn format(node: &JsAnyAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyAssignment::JsIdentifierAssignment(node) => node.format().format(f),
            JsAnyAssignment::JsStaticMemberAssignment(node) => node.format().format(f),
            JsAnyAssignment::JsComputedMemberAssignment(node) => node.format().format(f),
            JsAnyAssignment::JsParenthesizedAssignment(node) => node.format().format(f),
            JsAnyAssignment::TsNonNullAssertionAssignment(node) => node.format().format(f),
            JsAnyAssignment::TsAsAssignment(node) => node.format().format(f),
            JsAnyAssignment::TsTypeAssertionAssignment(node) => node.format().format(f),
            JsAnyAssignment::JsUnknownAssignment(node) => node.format().format(f),
        }
    }
}
