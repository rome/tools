//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsxAnyAttribute;
#[derive(Debug, Clone, Default)]
pub struct FormatJsxAnyAttribute;
impl FormatRule<JsxAnyAttribute> for FormatJsxAnyAttribute {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyAttribute::JsxAttribute(node) => node.format().fmt(f),
            JsxAnyAttribute::JsxSpreadAttribute(node) => node.format().fmt(f),
        }
    }
}
