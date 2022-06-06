use crate::prelude::*;
use crate::utils::node_has_leading_newline;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsObjectExpression;
use rome_js_syntax::JsObjectExpressionFields;

impl FormatNodeFields<JsObjectExpression> for FormatNodeRule<JsObjectExpression> {
    fn fmt_fields(node: &JsObjectExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsObjectExpressionFields {
            l_curly_token,
            members,
            r_curly_token,
        } = node.as_fields();

        let has_newline = node_has_leading_newline(members.syntax());

        if members.is_empty() {
            write!(
                f,
                [
                    format_delimited(&l_curly_token?, &members.format(), &r_curly_token?)
                        .soft_block_indent()
                ]
            )
        } else if has_newline {
            write!(
                f,
                [
                    format_delimited(&l_curly_token?, &members.format(), &r_curly_token?)
                        .block_indent()
                ]
            )
        } else {
            write!(
                f,
                [
                    format_delimited(&l_curly_token?, &members.format(), &r_curly_token?)
                        .soft_block_spaces()
                ]
            )
        }
    }
}
