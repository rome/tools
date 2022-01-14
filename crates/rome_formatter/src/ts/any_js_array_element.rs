use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyArrayElement;

impl ToFormatElement for JsAnyArrayElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyArrayElement::JsSpread(spread) => spread.to_format_element(formatter),
            JsAnyArrayElement::JsAnyExpression(expr) => expr.to_format_element(formatter),
            JsAnyArrayElement::JsArrayHole(hole) => hole.to_format_element(formatter),
        }
    }
}
