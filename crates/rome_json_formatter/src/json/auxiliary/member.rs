use crate::prelude::*;
use rome_formatter::write;
use rome_json_syntax::JsonMember;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMember;

impl FormatNodeRule<JsonMember> for FormatJsonMember {
    fn fmt_fields(&self, node: &JsonMember, f: &mut JsonFormatter) -> FormatResult<()> {
        let content = format_with(move |f| {
            write!(
                f,
                [
                    group(&node.name().format()),
                    node.colon_token().format(),
                    space(),
                    node.value().format()
                ]
            )
        });

        write!(f, [group(&content)])
    }
}
