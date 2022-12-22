use crate::prelude::*;
use rome_formatter::utils::number::CleanedNumberLiteralText;
use rome_json_syntax::JsonNumberValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonNumberValue;

impl FormatNodeRule<JsonNumberValue> for FormatJsonNumberValue {
    fn fmt_fields(&self, node: &JsonNumberValue, f: &mut JsonFormatter) -> FormatResult<()> {
        CleanedNumberLiteralText::from_number_literal_token(&node.value_token()?).fmt(f)
    }
}
