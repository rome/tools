use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsImportDefaultClause;

impl ToFormatElement for JsImportDefaultClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let local_name = formatter.format_node(&self.local_name()?)?;
        let from = formatter.format_token(&self.from_token()?)?;
        let source = formatter.format_node(&self.source()?)?;
        let assertions = if let Some(assertion) = self.assertion() {
            format_elements![space_token(), formatter.format_node(&assertion)?]
        } else {
            empty_element()
        };

        Ok(format_elements![
            local_name,
            space_token(),
            from,
            space_token(),
            source,
            assertions
        ])
    }
}
