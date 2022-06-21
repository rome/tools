use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsComputedMemberExpression;
use rome_js_syntax::JsComputedMemberExpressionFields;

impl FormatNodeFields<JsComputedMemberExpression> for FormatNodeRule<JsComputedMemberExpression> {
    fn fmt_fields(node: &JsComputedMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsComputedMemberExpressionFields {
            object,
            optional_chain_token,
            l_brack_token,
            member,
            r_brack_token,
        } = node.as_fields();

        write![
            f,
            [
                object.format(),
                group_elements(&format_args![
                    optional_chain_token.format(),
                    l_brack_token.format(),
                    soft_line_break(),
                    soft_block_indent(&member.format()),
                    r_brack_token.format()
                ]),
            ]
        ]
    }
}
