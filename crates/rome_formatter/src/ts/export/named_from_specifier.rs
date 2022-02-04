use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsExportNamedFromSpecifier;

impl ToFormatElement for JsExportNamedFromSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_token = self
            .type_token()
            .format_with_or_empty(formatter, |type_token| {
                format_elements![type_token, space_token()]
            })?;
        let export_as = self
            .export_as()
            .format_with_or_empty(formatter, |export_as| {
                format_elements![export_as, space_token()]
            })?;
        let source = self.source_name().format(formatter)?;

        Ok(format_elements![type_token, export_as, source])
    }
}
