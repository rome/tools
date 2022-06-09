//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyAttribute;
use crate::prelude::*;
use rome_js_syntax::JsxAnyAttribute;
impl FormatRule<JsxAnyAttribute> for FormatJsxAnyAttribute {
    type Context = JsFormatContext;
    fn fmt(node: &JsxAnyAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyAttribute::JsxAttribute(node) => node.format().fmt(f),
            JsxAnyAttribute::JsxSpreadAttribute(node) => node.format().fmt(f),
        }
    }
}
