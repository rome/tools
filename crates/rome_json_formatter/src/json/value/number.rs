use crate::prelude::*;
use rome_json_syntax::JsonNumber;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonNumber;

impl FormatNodeRule<JsonNumber> for FormatJsonNumber {
    fn fmt_fields(&self, node: &JsonNumber, f: &mut JsonFormatter) -> FormatResult<()> {
        node.value_token()?.format().fmt(f)
    }
}
