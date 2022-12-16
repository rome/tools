use crate::prelude::*;
use rome_formatter::write;
use rome_json_syntax::JsonArrayValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonArrayValue;
impl FormatNodeRule<JsonArrayValue> for FormatJsonArrayValue {
    fn fmt_fields(&self, node: &JsonArrayValue, f: &mut JsonFormatter) -> FormatResult<()> {
        write!(f, [node.l_brack_token().format()])?;

        write!(f, [group(&soft_block_indent(&node.elements().format()))])?;

        write!(f, [node.r_brack_token().format()])
    }
}
