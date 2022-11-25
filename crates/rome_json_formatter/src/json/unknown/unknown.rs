use crate::FormatUnknownNodeRule;
use rome_json_syntax::JsonUnknown;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonUnknown;

impl FormatUnknownNodeRule<JsonUnknown> for FormatJsonUnknown {}
