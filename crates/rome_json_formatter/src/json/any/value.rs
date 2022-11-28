//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_json_syntax::JsonAnyValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonAnyValue;
impl FormatRule<JsonAnyValue> for FormatJsonAnyValue {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &JsonAnyValue, f: &mut JsonFormatter) -> FormatResult<()> {
        match node {
            JsonAnyValue::JsonStringValue(node) => node.format().fmt(f),
            JsonAnyValue::JsonBooleanValue(node) => node.format().fmt(f),
            JsonAnyValue::JsonNullValue(node) => node.format().fmt(f),
            JsonAnyValue::JsonNumberValue(node) => node.format().fmt(f),
            JsonAnyValue::JsonArrayValue(node) => node.format().fmt(f),
            JsonAnyValue::JsonObjectValue(node) => node.format().fmt(f),
            JsonAnyValue::JsonUnknownValue(node) => node.format().fmt(f),
        }
    }
}
