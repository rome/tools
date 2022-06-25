//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsxAnyTag;
#[derive(Debug, Clone, Default)]
pub struct FormatJsxAnyTag;
impl FormatRule<JsxAnyTag> for FormatJsxAnyTag {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyTag, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyTag::JsxElement(node) => node.format().fmt(f),
            JsxAnyTag::JsxSelfClosingElement(node) => node.format().fmt(f),
            JsxAnyTag::JsxFragment(node) => node.format().fmt(f),
        }
    }
}
