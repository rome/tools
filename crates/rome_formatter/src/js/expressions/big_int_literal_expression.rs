use std::borrow::Cow;

use crate::format_element::normalize_newlines;

use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_element::Token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::{
    JsAnyLiteralExpression, JsBigIntLiteralExpression, JsBooleanLiteralExpression,
    JsNullLiteralExpression, JsNumberLiteralExpression, JsStringLiteralExpression,
};

impl ToFormatElement for JsBigIntLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value_token().format(formatter)
    }
}
