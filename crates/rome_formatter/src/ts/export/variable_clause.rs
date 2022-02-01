use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsExportVariableClause;

impl ToFormatElement for JsExportVariableClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let declarations = formatter.format_node(&self.declarations()?)?;
        let semicolon = if let Some(semicolon) = self.semicolon_token() {
            formatter.format_token(&semicolon)?
        } else {
            token(";")
        };

        Ok(format_elements![declarations, semicolon])
    }
}
