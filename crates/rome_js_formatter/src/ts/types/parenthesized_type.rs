use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsParenthesizedType;
use rome_js_syntax::TsParenthesizedTypeFields;

impl FormatNodeFields<TsParenthesizedType> for FormatNodeRule<TsParenthesizedType> {
    fn format_fields(
        node: &TsParenthesizedType,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsParenthesizedTypeFields {
            l_paren_token,
            ty,
            r_paren_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_paren_token?,
                formatted![formatter, [ty.format()]]?,
                &r_paren_token?,
            )
            .soft_block_indent()
            .finish()
    }
}
