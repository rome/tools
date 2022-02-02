use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsExpressionStatement;

impl ToFormatElement for JsExpressionStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.expression().format(formatter)?,
            self.semicolon_token().format_or(formatter, || token(";"))?
        ])
    }
}
