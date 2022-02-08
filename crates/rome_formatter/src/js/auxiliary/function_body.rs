use crate::{
    block_indent, format_elements, ts::directives::format_directives_list, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsFunctionBody;

impl ToFormatElement for JsFunctionBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(block_indent(format_elements![
                    open_token_trailing,
                    format_directives_list(self.directives(), formatter),
                    formatter.format_list(self.statements()),
                    close_token_leading,
                ]))
            },
            &self.r_curly_token()?,
        )
    }
}
