use crate::prelude::*;
use rome_formatter::write;
use rome_json_syntax::{JsonRoot, JsonRootFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonRoot;

impl FormatNodeRule<JsonRoot> for FormatJsonRoot {
    fn fmt_fields(&self, node: &JsonRoot, f: &mut JsonFormatter) -> FormatResult<()> {
        let JsonRootFields { value, eof_token } = node.as_fields();

        write!(
            f,
            [
                value.format(),
                format_removed(&eof_token?),
                hard_line_break()
            ]
        )
    }
}
