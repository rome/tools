use crate::{token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{
    JsAnyLiteralExpression, JsBigIntLiteralExpression, JsBooleanLiteralExpression,
    JsNullLiteralExpression, JsNumberLiteralExpression, JsStringLiteralExpression,
};

impl ToFormatElement for JsStringLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let value_token = self.value_token()?;
        let quoted = value_token.text_trimmed();

        // uses single quotes
        let content = if quoted.starts_with('\'') {
            let s = &quoted[1..quoted.len() - 1];
            let s = format!("\"{}\"", s);
            token(s)
        } else {
            token(quoted)
        };

        formatter.format_replaced(&value_token, content)
    }
}

impl ToFormatElement for JsBooleanLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_token(&self.value_token()?)
    }
}

impl ToFormatElement for JsNullLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_token(&self.value_token()?)
    }
}

impl ToFormatElement for JsNumberLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_token(&self.value_token()?)
    }
}

impl ToFormatElement for JsBigIntLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        formatter.format_token(&self.value_token()?)
    }
}

impl ToFormatElement for JsAnyLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyLiteralExpression::JsBooleanLiteralExpression(boolean) => {
                boolean.to_format_element(formatter)
            }
            JsAnyLiteralExpression::JsStringLiteralExpression(string_literal) => {
                string_literal.to_format_element(formatter)
            }
            JsAnyLiteralExpression::JsNumberLiteralExpression(number) => {
                number.to_format_element(formatter)
            }
            JsAnyLiteralExpression::JsBigIntLiteralExpression(big_int) => {
                big_int.to_format_element(formatter)
            }
            JsAnyLiteralExpression::JsNullLiteralExpression(null_literal) => {
                null_literal.to_format_element(formatter)
            }
            JsAnyLiteralExpression::JsRegexLiteralExpression(_) => todo!(),
        }
    }
}
