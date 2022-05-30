//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectMemberName;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMemberName;
impl FormatRule<JsAnyObjectMemberName> for FormatJsAnyObjectMemberName {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyObjectMemberName,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyObjectMemberName::JsLiteralMemberName(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectMemberName::JsComputedMemberName(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
