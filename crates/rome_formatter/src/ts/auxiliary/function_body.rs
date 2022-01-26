use rslint_parser::ast::JsFunctionBody;

use crate::ts::statements::format_statements;
use crate::{
    block_indent, format_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsFunctionBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(block_indent(format_elements![
                    open_token_trailing,
                    format_statements(self.statements(), formatter),
                    close_token_leading,
                ]))
            },
            &self.r_curly_token()?,
        )
    }
}
