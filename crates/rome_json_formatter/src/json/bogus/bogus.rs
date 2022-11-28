use crate::FormatBogusNodeRule;
use rome_json_syntax::JsonBogus;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonBogus;

impl FormatBogusNodeRule<JsonBogus> for FormatJsonBogus {}
