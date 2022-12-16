use crate::prelude::*;
use rome_formatter::{format_args, write};
use rome_json_syntax::JsonMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMember;

impl FormatNodeRule<JsonMember> for FormatJsonMember {
    fn fmt_fields(&self, node: &JsonMember, f: &mut JsonFormatter) -> FormatResult<()> {
        write!(
            f,
            [group(&format_args![
                &node.name().format(),
                node.colon_token().format(),
                space(),
                node.value().format()
            ])]
        )
    }
}
