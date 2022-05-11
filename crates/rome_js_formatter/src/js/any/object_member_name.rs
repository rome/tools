//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectMemberName;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMemberName;
impl FormatRule<JsAnyObjectMemberName> for FormatJsAnyObjectMemberName {
    fn format(node: &JsAnyObjectMemberName, formatter: &Formatter) -> FormatResult<FormatElement> {
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
