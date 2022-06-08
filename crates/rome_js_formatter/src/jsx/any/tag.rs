//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyTag;
use crate::prelude::*;
use rome_js_syntax::JsxAnyTag;
impl FormatRule<JsxAnyTag> for FormatJsxAnyTag {
    type Context = JsFormatContext;
    fn fmt(node: &JsxAnyTag, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyTag::JsxElement(node) => node.format().fmt(f),
            JsxAnyTag::JsxSelfClosingElement(node) => node.format().fmt(f),
            JsxAnyTag::JsxFragment(node) => node.format().fmt(f),
        }
    }
}
