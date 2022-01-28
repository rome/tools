use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsCallExpression;

impl ToFormatElement for JsCallExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let name = formatter.format_node(&self.callee()?)?;
        let option = formatter.format_token(&self.optional_chain_token_token())?;
        let arguments = formatter.format_node(&self.arguments()?)?;

        Ok(format_elements![name, option, arguments])
    }
}
