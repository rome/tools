//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsxAnyAttributeValue;
#[derive(Debug, Clone, Default)]
pub struct FormatJsxAnyAttributeValue;
impl FormatRule<JsxAnyAttributeValue> for FormatJsxAnyAttributeValue {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsxAnyAttributeValue, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsxAnyAttributeValue::JsxAnyTag(node) => node.format().fmt(f),
            JsxAnyAttributeValue::JsxString(node) => node.format().fmt(f),
            JsxAnyAttributeValue::JsxExpressionAttributeValue(node) => node.format().fmt(f),
        }
    }
}
