use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, group_elements, soft_block_indent, space_token, token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsDoWhileStatement;

impl ToFormatElement for JsDoWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.do_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?,
            space_token(),
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
            self.semicolon_token().format_or(formatter, || token(";"))?
        ])
    }
}
