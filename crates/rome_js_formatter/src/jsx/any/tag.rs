//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyTag;
use crate::prelude::*;
use rome_js_syntax::JsxAnyTag;
impl FormatRule<JsxAnyTag> for FormatJsxAnyTag {
    type Context = JsFormatContext;
    fn format(node: &JsxAnyTag, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        match node {
            JsxAnyTag::JsxElement(node) => formatted![formatter, [node.format()]],
            JsxAnyTag::JsxSelfClosingElement(node) => formatted![formatter, [node.format()]],
            JsxAnyTag::JsxFragment(node) => formatted![formatter, [node.format()]],
        }
    }
}
