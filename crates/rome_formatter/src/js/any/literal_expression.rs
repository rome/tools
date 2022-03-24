//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyLiteralExpression;
impl ToFormatElement for JsAnyLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsStringLiteralExpression(node) => node.format(formatter),
            Self::JsNumberLiteralExpression(node) => node.format(formatter),
            Self::JsBigIntLiteralExpression(node) => node.format(formatter),
            Self::JsBooleanLiteralExpression(node) => node.format(formatter),
            Self::JsNullLiteralExpression(node) => node.format(formatter),
            Self::JsRegexLiteralExpression(node) => node.format(formatter),
        }
    }
}
