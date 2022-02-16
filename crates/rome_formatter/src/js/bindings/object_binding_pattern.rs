use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsObjectBindingPattern;

impl ToFormatElement for JsObjectBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            self.properties().format(formatter)?,
            &self.r_curly_token()?,
        )
    }
}
