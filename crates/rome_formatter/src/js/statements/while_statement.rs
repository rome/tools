use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsWhileStatement;

impl ToFormatElement for JsWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.while_token().format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &self.l_paren_token()?,
                self.test().format(formatter)?,
                &self.r_paren_token()?,
            )?,
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
