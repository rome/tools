//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_json_syntax::JsonAnyValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonAnyValue;
impl FormatRule<JsonAnyValue> for FormatJsonAnyValue {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &JsonAnyValue, f: &mut JsonFormatter) -> FormatResult<()> {
        match node {
            JsonAnyValue::JsonString(node) => node.format().fmt(f),
            JsonAnyValue::JsonBoolean(node) => node.format().fmt(f),
            JsonAnyValue::JsonNull(node) => node.format().fmt(f),
            JsonAnyValue::JsonNumber(node) => node.format().fmt(f),
            JsonAnyValue::JsonArray(node) => node.format().fmt(f),
            JsonAnyValue::JsonObject(node) => node.format().fmt(f),
            JsonAnyValue::JsonUnknown(node) => node.format().fmt(f),
        }
    }
}
