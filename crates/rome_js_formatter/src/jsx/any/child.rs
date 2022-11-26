//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsxAnyChild;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxAnyChild;
impl FormatRule<JsxAnyChild> for FormatJsxAnyChild {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyChild, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyChild::JsxElement(node) => node.format().fmt(f),
            JsxAnyChild::JsxSelfClosingElement(node) => node.format().fmt(f),
            JsxAnyChild::JsxText(node) => node.format().fmt(f),
            JsxAnyChild::JsxExpressionChild(node) => node.format().fmt(f),
            JsxAnyChild::JsxSpreadChild(node) => node.format().fmt(f),
            JsxAnyChild::JsxFragment(node) => node.format().fmt(f),
        }
    }
}
