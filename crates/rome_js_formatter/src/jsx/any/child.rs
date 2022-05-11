//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyChild;
use crate::prelude::*;
use rome_js_syntax::JsxAnyChild;
impl FormatRule<JsxAnyChild> for FormatJsxAnyChild {
    fn format(node: &JsxAnyChild, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsxAnyChild::JsxElement(node) => formatted![formatter, [node.format()]],
            JsxAnyChild::JsxSelfClosingElement(node) => formatted![formatter, [node.format()]],
            JsxAnyChild::JsxText(node) => formatted![formatter, [node.format()]],
            JsxAnyChild::JsxExpressionChild(node) => formatted![formatter, [node.format()]],
            JsxAnyChild::JsxSpreadChild(node) => formatted![formatter, [node.format()]],
            JsxAnyChild::JsxFragment(node) => formatted![formatter, [node.format()]],
        }
    }
}
