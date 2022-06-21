use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::TsParenthesizedType;
use rome_js_syntax::TsParenthesizedTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsParenthesizedType;

impl FormatNodeRule<TsParenthesizedType> for FormatTsParenthesizedType {
    fn fmt_fields(&self, node: &TsParenthesizedType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsParenthesizedTypeFields {
            l_paren_token,
            ty,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_delimited(&l_paren_token?, &ty.format(), &r_paren_token?,)
                    .soft_block_indent()
            ]
        )
    }
}
