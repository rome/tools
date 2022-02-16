use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsWithStatement;

impl ToFormatElement for JsWithStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.with_token().format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &self.l_paren_token()?,
                self.object().format(formatter)?,
                &self.r_paren_token()?,
            )?,
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
