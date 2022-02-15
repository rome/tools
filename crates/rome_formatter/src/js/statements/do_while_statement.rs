use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
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
            formatter.format_delimited_soft_block_indent(
                &self.l_paren_token()?,
                self.test().format(formatter)?,
                &self.r_paren_token()?,
            )?,
            self.semicolon_token().format_or(formatter, || token(";"))?
        ])
    }
}
