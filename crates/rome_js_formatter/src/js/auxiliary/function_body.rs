use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsFunctionBody;
use rome_js_syntax::JsFunctionBodyFields;

impl FormatNodeFields<JsFunctionBody> for FormatNodeRule<JsFunctionBody> {
    fn format_fields(node: &JsFunctionBody, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = node.as_fields();

        formatter.format_delimited_block_indent(
            &l_curly_token?,
            formatted![
                formatter,
                [directives.format(), formatter.format_list(&statements),]
            ]?,
            &r_curly_token?,
        )
    }
}
