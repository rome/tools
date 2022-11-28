use crate::prelude::*;
use rome_json_syntax::JsonMemberName;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMemberName;

impl FormatNodeRule<JsonMemberName> for FormatJsonMemberName {
    fn fmt_fields(&self, node: &JsonMemberName, f: &mut JsonFormatter) -> FormatResult<()> {
        node.value_token()?.format().fmt(f)
    }
}
