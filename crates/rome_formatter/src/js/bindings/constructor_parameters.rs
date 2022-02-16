use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsConstructorParameters;

impl ToFormatElement for JsConstructorParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_indent(
            &self.l_paren_token()?,
            self.parameters().format(formatter)?,
            &self.r_paren_token()?,
        )
    }
}
