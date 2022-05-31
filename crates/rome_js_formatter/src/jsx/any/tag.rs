//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyTag;
use crate::prelude::*;
use rome_js_syntax::JsxAnyTag;
impl FormatRule<JsxAnyTag> for FormatJsxAnyTag {
    type Context = JsFormatContext;
    fn format(node: &JsxAnyTag, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        match node {
            JsxAnyTag::JsxElement(node) => node.format().format(f),
            JsxAnyTag::JsxSelfClosingElement(node) => node.format().format(f),
            JsxAnyTag::JsxFragment(node) => node.format().format(f),
        }
    }
}
