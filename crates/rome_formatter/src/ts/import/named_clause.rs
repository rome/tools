use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsImportNamedClause;

impl ToFormatElement for JsImportNamedClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let source = formatter.format_node(self.source()?)?;
        let default = if let Some(default) = self.default_specifier() {
            format_elements![formatter.format_node(default)?, space_token()]
        } else {
            empty_element()
        };
        let from = formatter.format_token(&self.from_token()?)?;
        let name = formatter.format_node(self.named_import()?)?;
        let assertion = if let Some(assertion) = self.assertion() {
            format_elements![space_token(), formatter.format_node(assertion)?]
        } else {
            empty_element()
        };
        Ok(format_elements![
            default,
            name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ])
    }
}
