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

        let r_curly_token = r_curly_token?;

        if statements.is_empty() && directives.is_empty() {
            write!(
                f,
                [
                    l_curly_token.format(),
                    format_dangling_comments(node.syntax()).indented(),
                    r_curly_token.format()
                ]
            )
        } else {
            write!(
                f,
                [
                    l_curly_token.format(),
                    block_indent(&format_args![directives.format(), statements.format()]),
                    r_curly_token.format(),
                ]
            )
        }
    }
}
