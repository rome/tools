//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyLiteralExpression;
impl ToFormatElement for JsAnyLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsStringLiteralExpression(node) => node.to_format_element(formatter),
            Self::JsNumberLiteralExpression(node) => node.to_format_element(formatter),
            Self::JsBigIntLiteralExpression(node) => node.to_format_element(formatter),
            Self::JsBooleanLiteralExpression(node) => node.to_format_element(formatter),
            Self::JsNullLiteralExpression(node) => node.to_format_element(formatter),
            Self::JsRegexLiteralExpression(node) => node.to_format_element(formatter),
        }
    }
}
