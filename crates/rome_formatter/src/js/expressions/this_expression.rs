use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsThisExpression;

impl ToFormatElement for JsThisExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.this_token().format(formatter)
    }
}
