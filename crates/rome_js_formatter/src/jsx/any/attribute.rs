//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyAttribute;
use crate::prelude::*;
use rome_js_syntax::JsxAnyAttribute;
impl FormatRule<JsxAnyAttribute> for FormatJsxAnyAttribute {
    type Options = JsFormatOptions;
    fn format(
        node: &JsxAnyAttribute,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsxAnyAttribute::JsxAttribute(node) => formatted![formatter, [node.format()]],
            JsxAnyAttribute::JsxSpreadAttribute(node) => formatted![formatter, [node.format()]],
        }
    }
}
