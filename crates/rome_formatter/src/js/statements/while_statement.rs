use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::JsWhileStatement;

impl ToFormatElement for JsWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.while_token().format(formatter)?,
            space_token(),
            group_elements(formatter.format_delimited(
                &self.l_paren_token()?,
                |open_token_trailing, close_token_leading| Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    self.test().format(formatter)?,
                    close_token_leading,
                ])),
                &self.r_paren_token()?,
            )?),
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
