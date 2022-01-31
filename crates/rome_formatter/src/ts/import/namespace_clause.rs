use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsImportNamespaceClause;

impl ToFormatElement for JsImportNamespaceClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let star = formatter.format_token(&self.star_token()?)?;
        let as_token = formatter.format_token(&self.as_token()?)?;
        let local_name = formatter.format_node(&self.local_name()?)?;
        let source = formatter.format_node(&self.source()?)?;
        let from = formatter.format_token(&self.from_token()?)?;
        let assertion = if let Some(assertion) = self.assertion() {
            format_elements![space_token(), formatter.format_node(&assertion)?]
        } else {
            empty_element()
        };
        Ok(format_elements![
            star,
            space_token(),
            as_token,
            space_token(),
            local_name,
            space_token(),
            from,
            space_token(),
            source,
            assertion
        ])
    }
}
