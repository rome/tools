use rslint_parser::ast::JsExpressionStatement;

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for JsExpressionStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(self.expression()?)?,
            formatter
                .format_token(&self.semicolon_token())?
                .unwrap_or_else(|| token(";"))
        ])
    }
}
