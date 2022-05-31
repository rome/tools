use crate::prelude::*;
use crate::utils::has_leading_newline;
use crate::FormatNodeFields;
use rome_js_syntax::JsObjectExpression;
use rome_js_syntax::JsObjectExpressionFields;

impl FormatNodeFields<JsObjectExpression> for FormatNodeRule<JsObjectExpression> {
    fn format_fields(
        node: &JsObjectExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsObjectExpressionFields {
            l_curly_token,
            members,
            r_curly_token,
        } = node.as_fields();

        let has_newline = has_leading_newline(members.syntax());
        let members_content = formatted![formatter, [members.format()]]?;

        if members.is_empty() {
            formatter
                .delimited(&l_curly_token?, members_content, &r_curly_token?)
                .soft_block_indent()
                .finish()
        } else if has_newline {
            formatter
                .delimited(&l_curly_token?, members_content, &r_curly_token?)
                .block_indent()
                .finish()
        } else {
            formatter
                .delimited(&l_curly_token?, members_content, &r_curly_token?)
                .soft_block_spaces()
                .finish()
        }
    }
}
