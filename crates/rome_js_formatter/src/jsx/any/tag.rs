//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsxAnyTag;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxAnyTag;
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
