use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, group_elements, soft_block_indent,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsCallArguments;

impl ToFormatElement for JsCallArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(formatter.format_delimited(
            &self.l_paren_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    self.args().format(formatter)?,
                    close_token_leading
                ]))
            },
            &self.r_paren_token()?,
        )?))
    }
}
