use crate::utils::format_binaryish_expression;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsLogicalExpression;
use rslint_parser::AstNode;

impl ToFormatElement for JsLogicalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binaryish_expression(self.syntax(), formatter)
    }
}
