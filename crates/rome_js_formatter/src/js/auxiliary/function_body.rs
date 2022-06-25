use crate::prelude::*;

use rome_formatter::{format_args, write};
use rome_js_syntax::JsFunctionBody;
use rome_js_syntax::JsFunctionBodyFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsFunctionBody;

impl FormatNodeRule<JsFunctionBody> for FormatJsFunctionBody {
    fn fmt_fields(&self, node: &JsFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = node.as_fields();

        write!(
            f,
            [format_delimited(
                &l_curly_token?,
                &format_args![directives.format(), statements.format()],
                &r_curly_token?,
            )
            .block_indent()]
        )
    }
}
