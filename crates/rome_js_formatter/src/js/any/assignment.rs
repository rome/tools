//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyAssignment;
use crate::prelude::*;
use rome_js_syntax::JsAnyAssignment;
impl FormatRule<JsAnyAssignment> for FormatJsAnyAssignment {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyAssignment::JsIdentifierAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::JsStaticMemberAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::JsComputedMemberAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::JsParenthesizedAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::TsNonNullAssertionAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::TsAsAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::TsTypeAssertionAssignment(node) => node.format().fmt(f),
            JsAnyAssignment::JsUnknownAssignment(node) => node.format().fmt(f),
        }
    }
}
