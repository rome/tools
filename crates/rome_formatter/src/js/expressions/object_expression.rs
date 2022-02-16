use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsObjectExpression;

impl ToFormatElement for JsObjectExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let members = self.members().format(formatter)?;

        if members.is_empty() {
            formatter.format_delimited_soft_block_indent(
                &self.l_curly_token()?,
                members,
                &self.r_curly_token()?,
            )
        } else {
            formatter.format_delimited_soft_block_spaces(
                &self.l_curly_token()?,
                members,
                &self.r_curly_token()?,
            )
        }
    }
}
