use crate::prelude::*;
use crate::{format_or_verbatim, FormatNodeFields};
use rome_formatter::{format_args, write};
use rome_js_syntax::JsFunctionBody;
use rome_js_syntax::JsFunctionBodyFields;

impl FormatNodeFields<JsFunctionBody> for FormatNodeRule<JsFunctionBody> {
    fn fmt_fields(node: &JsFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = node.as_fields();

        let format_statements = format_with(|f| {
            let mut join = f.join_nodes_with_hardline();

            for stmt in &statements {
                join.entry(stmt.syntax(), &format_or_verbatim(&stmt));
            }

            join.finish()
        });

        write!(
            f,
            [format_delimited(
                &l_curly_token?,
                &format_args![directives.format(), format_statements],
                &r_curly_token?,
            )
            .block_indent()]
        )
    }
}
