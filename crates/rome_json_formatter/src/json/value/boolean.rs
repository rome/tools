use crate::prelude::*;
use rome_json_syntax::JsonBoolean;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonBoolean;

impl FormatNodeRule<JsonBoolean> for FormatJsonBoolean {
    fn fmt_fields(&self, node: &JsonBoolean, f: &mut JsonFormatter) -> FormatResult<()> {
        node.value_token()?.format().fmt(f)
    }
}
