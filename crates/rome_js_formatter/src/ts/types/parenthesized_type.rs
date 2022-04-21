use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::TsParenthesizedType;
use rome_js_syntax::TsParenthesizedTypeFields;

impl FormatNode for TsParenthesizedType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsParenthesizedTypeFields {
            l_paren_token,
            ty,
            r_paren_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            ty.format(formatter)?,
            &r_paren_token?,
        )
    }
}
