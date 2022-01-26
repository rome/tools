use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::JsExportNamedFromSpecifier;

impl ToFormatElement for JsExportNamedFromSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_token = if let Some(type_token) = self.type_token() {
            format_elements![formatter.format_token(&type_token)?, space_token()]
        } else {
            empty_element()
        };
        let export_as = if let Some(export_as) = self.export_as() {
            format_elements![formatter.format_node(export_as)?, space_token()]
        } else {
            empty_element()
        };
        let source = formatter.format_node(self.source_name()?)?;

        Ok(format_elements![type_token, export_as, source])
    }
}
