use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, group_elements, soft_block_indent,
    space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsObjectBindingPattern;

impl ToFormatElement for JsObjectBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(format_elements![
                    space_token(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        self.properties().format(formatter)?,
                        close_token_leading,
                    ]),
                    space_token(),
                ])
            },
            &self.r_curly_token()?,
        )?))
    }
}
