//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsxAnyAttributeValue;
use crate::prelude::*;
use rome_js_syntax::JsxAnyAttributeValue;
impl FormatRule<JsxAnyAttributeValue> for FormatJsxAnyAttributeValue {
    type Context = JsFormatContext;
    fn fmt(node: &JsxAnyAttributeValue, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyAttributeValue::JsxAnyTag(node) => node.format().fmt(f),
            JsxAnyAttributeValue::JsxString(node) => node.format().fmt(f),
            JsxAnyAttributeValue::JsxExpressionAttributeValue(node) => node.format().fmt(f),
        }
    }
}
