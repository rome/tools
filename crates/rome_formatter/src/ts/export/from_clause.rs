use crate::{
    empty_element, format_elements, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsExportFromClause;

impl ToFormatElement for JsExportFromClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let star = formatter.format_token(&self.star_token()?)?;
        let export_as = if let Some(export_as) = self.export_as() {
            format_elements![formatter.format_node(&export_as)?, space_token()]
        } else {
            empty_element()
        };
        let from = formatter.format_token(&self.from_token()?)?;
        let source = formatter.format_node(&self.source()?)?;
        let assertion = if let Some(assertion) = self.assertion() {
            formatter.format_node(&assertion)?
        } else {
            empty_element()
        };
        let semicolon = if let Some(semicolon) = self.semicolon_token() {
            formatter.format_token(&semicolon)?
        } else {
            token(";")
        };
        Ok(format_elements![
            star,
            space_token(),
            export_as,
            from,
            space_token(),
            source,
            assertion,
            semicolon
        ])
    }
}
