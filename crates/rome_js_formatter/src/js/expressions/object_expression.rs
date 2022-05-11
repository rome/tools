use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsObjectExpression;
use rome_js_syntax::JsObjectExpressionFields;

impl FormatNodeFields<JsObjectExpression> for FormatNodeRule<JsObjectExpression> {
    fn format_fields(
        node: &JsObjectExpression,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectExpressionFields {
            l_curly_token,
            members,
            r_curly_token,
        } = node.as_fields();

        let members_content = formatted![formatter, [members.format()]]?;

        if members.is_empty() {
            formatter.format_delimited_soft_block_indent(
                &l_curly_token?,
                members_content,
                &r_curly_token?,
            )
        } else {
            formatter.format_delimited_soft_block_spaces(
                &l_curly_token?,
                members_content,
                &r_curly_token?,
            )
        }
    }
}
