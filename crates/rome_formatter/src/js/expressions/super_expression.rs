use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsSuperExpression;

impl ToFormatElement for JsSuperExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.super_token().format(formatter)
    }
}
