use crate::utils::format_call_expression;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsCallExpression;
use rslint_parser::ast::JsCallExpressionFields;
use rslint_parser::AstNode;

impl ToFormatElement for JsCallExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCallExpressionFields {
            callee,
            optional_chain_token,
            type_arguments,
            arguments,
        } = self.as_fields();

        format_call_expression(self.syntax(), formatter)
    }
}
