use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsRegexLiteralExpression;

impl ToFormatElement for JsRegexLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_token(&self.value_token()?)
    }
}
