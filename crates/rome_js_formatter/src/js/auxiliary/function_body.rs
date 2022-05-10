use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsFunctionBody;
use rome_js_syntax::JsFunctionBodyFields;

impl FormatNodeFields<JsFunctionBody> for FormatNodeRule<JsFunctionBody> {
    fn format_fields(
        node: &JsFunctionBody,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_curly_token?,
                formatted![
                    formatter,
                    [
                        directives.format(),
                        formatter.format_list_with_hard_line(&statements),
                    ]
                ]?,
                &r_curly_token?,
            )
            .block_indent()
            .finish()
    }
}
