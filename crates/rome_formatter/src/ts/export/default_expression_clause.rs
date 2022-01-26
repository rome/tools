use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExportDefaultExpressionClause;

impl ToFormatElement for JsExportDefaultExpressionClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let default_token = formatter.format_token(&self.default_token()?)?;
        let class = formatter.format_node(self.expression()?)?;
        let semicolon = if let Some(semicolon) = &self.semicolon_token() {
            formatter.format_token(semicolon)?
        } else {
            token(";")
        };
        Ok(format_elements![
            default_token,
            space_token(),
            class,
            semicolon
        ])
    }
}
