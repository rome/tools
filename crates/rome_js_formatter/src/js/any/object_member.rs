//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyObjectMember;
use crate::prelude::*;
use rome_js_syntax::JsAnyObjectMember;
impl FormatRule<JsAnyObjectMember> for FormatJsAnyObjectMember {
    type Context = JsFormatContext;
    fn format(node: &JsAnyObjectMember, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyObjectMember::JsPropertyObjectMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectMember::JsMethodObjectMember(node) => formatted![formatter, [node.format()]],
            JsAnyObjectMember::JsGetterObjectMember(node) => formatted![formatter, [node.format()]],
            JsAnyObjectMember::JsSetterObjectMember(node) => formatted![formatter, [node.format()]],
            JsAnyObjectMember::JsShorthandPropertyObjectMember(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyObjectMember::JsSpread(node) => formatted![formatter, [node.format()]],
            JsAnyObjectMember::JsUnknownMember(node) => formatted![formatter, [node.format()]],
        }
    }
}
