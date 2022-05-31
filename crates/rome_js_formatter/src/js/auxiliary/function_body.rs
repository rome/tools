use crate::formatter::TryFormatNodeListExtension;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsFunctionBody;
use rome_js_syntax::JsFunctionBodyFields;

impl FormatNodeFields<JsFunctionBody> for FormatNodeRule<JsFunctionBody> {
    fn format_fields(node: &JsFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = node.as_fields();

        let format_statements = format_with(|f| {
            f.join_with(&hard_line_break())
                .entries(statements.try_format_nodes())
                .finish()
        });

        write!(
            f,
            [f.delimited(
                &l_curly_token?,
                &format_args![directives.format(), format_statements],
                &r_curly_token?,
            )
            .block_indent()]
        )
    }
}
