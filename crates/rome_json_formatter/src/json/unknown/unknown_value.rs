use crate::FormatUnknownNodeRule;
use rome_json_syntax::JsonUnknownValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonUnknownValue;

impl FormatUnknownNodeRule<JsonUnknownValue> for FormatJsonUnknownValue {}
