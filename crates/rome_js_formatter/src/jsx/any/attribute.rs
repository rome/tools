//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsxAnyAttribute;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsxAnyAttribute;
impl FormatRule<JsxAnyAttribute> for FormatJsxAnyAttribute {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyAttribute::JsxAttribute(node) => node.format().fmt(f),
            JsxAnyAttribute::JsxSpreadAttribute(node) => node.format().fmt(f),
        }
    }
}
