use rslint_parser::ast::JsFunctionBody;

use crate::ts::statements::format_statements;
use crate::{
    block_indent, format_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsFunctionBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited(
            &self.l_curly_token()?,
            |leading, trailing| {
                Ok(block_indent(format_elements![
                    leading,
                    format_statements(self.statements(), formatter),
                    trailing,
                ]))
            },
            &self.r_curly_token()?,
        )
    }
}
