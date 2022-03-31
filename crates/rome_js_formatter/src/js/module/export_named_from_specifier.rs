use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsExportNamedFromSpecifier;
use rome_js_syntax::JsExportNamedFromSpecifierFields;

impl ToFormatElement for JsExportNamedFromSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedFromSpecifierFields {
            type_token,
            source_name,
            export_as,
        } = self.as_fields();

        let type_token = type_token.format_with_or_empty(formatter, |type_token| {
            format_elements![type_token, space_token()]
        })?;

        let source_name = source_name.format(formatter)?;

        let export_as = export_as.format_with_or_empty(formatter, |export_as| {
            format_elements![space_token(), export_as]
        })?;

        Ok(format_elements![type_token, source_name, export_as])
    }
}
