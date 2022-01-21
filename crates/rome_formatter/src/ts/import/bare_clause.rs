use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsImportBareClause;

impl ToFormatElement for JsImportBareClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let source = formatter.format_node(self.source()?)?;
        let assertion = if let Some(assertion) = self.assertion() {
            formatter.format_node(assertion)?
        } else {
            empty_element()
        };

        Ok(format_elements![source, space_token(), assertion])
    }
}
