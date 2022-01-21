use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsImport;

impl ToFormatElement for JsImport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let import_token = formatter.format_token(&self.import_token()?)?;
        let import_clause = formatter.format_node(self.import_clause()?)?;
        let semicolon = formatter
            .format_token(&self.semicolon_token())?
            .unwrap_or_else(|| token(';'));

        Ok(format_elements![
            import_token,
            space_token(),
            import_clause,
            semicolon
        ])
    }
}
