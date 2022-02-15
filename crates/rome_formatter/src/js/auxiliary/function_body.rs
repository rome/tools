use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::ast::JsFunctionBody;

impl ToFormatElement for JsFunctionBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_block_indent(
            &self.l_curly_token()?,
            format_elements![
                self.directives().format(formatter)?,
                formatter.format_list(self.statements()),
            ],
            &self.r_curly_token()?,
        )
    }
}
