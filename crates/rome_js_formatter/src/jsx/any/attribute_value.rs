//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyAttributeValue;
use crate::prelude::*;
use rome_js_syntax::JsxAnyAttributeValue;
impl FormatRule<JsxAnyAttributeValue> for FormatJsxAnyAttributeValue {
    type Options = JsFormatOptions;
    fn format(
        node: &JsxAnyAttributeValue,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsxAnyAttributeValue::JsxAnyTag(node) => formatted![formatter, [node.format()]],
            JsxAnyAttributeValue::JsxString(node) => formatted![formatter, [node.format()]],
            JsxAnyAttributeValue::JsxExpressionAttributeValue(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
