use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::TsParenthesizedType;
use rome_js_syntax::TsParenthesizedTypeFields;

impl FormatNodeFields<TsParenthesizedType> for FormatNodeRule<TsParenthesizedType> {
    fn format_fields(
        node: &TsParenthesizedType,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let TsParenthesizedTypeFields {
            l_paren_token,
            ty,
            r_paren_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            formatted![formatter, ty.format()]?,
            &r_paren_token?,
        )
    }
}
