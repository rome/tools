use crate::prelude::*;
use rome_json_syntax::JsonString;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonString;

impl FormatNodeRule<JsonString> for FormatJsonString {
    fn fmt_fields(&self, node: &JsonString, f: &mut JsonFormatter) -> FormatResult<()> {
        node.value_token()?.format().fmt(f)
    }
}
