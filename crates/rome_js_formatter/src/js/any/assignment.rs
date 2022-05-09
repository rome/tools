//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyAssignment;
use crate::prelude::*;
use rome_js_syntax::JsAnyAssignment;
impl FormatRule<JsAnyAssignment> for FormatJsAnyAssignment {
    fn format(node: &JsAnyAssignment, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyAssignment::JsIdentifierAssignment(node) => formatted![formatter, node.format()],
            JsAnyAssignment::JsStaticMemberAssignment(node) => formatted![formatter, node.format()],
            JsAnyAssignment::JsComputedMemberAssignment(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyAssignment::JsParenthesizedAssignment(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyAssignment::TsNonNullAssertionAssignment(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyAssignment::TsAsAssignment(node) => formatted![formatter, node.format()],
            JsAnyAssignment::TsTypeAssertionAssignment(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyAssignment::JsUnknownAssignment(node) => formatted![formatter, node.format()],
        }
    }
}
