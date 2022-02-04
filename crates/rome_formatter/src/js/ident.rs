use rslint_parser::ast::Ident;

use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for Ident {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.ident_token().format(formatter)
    }
}
