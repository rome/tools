use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};

use rome_js_syntax::JsExportNamedFromSpecifier;
use rome_js_syntax::JsExportNamedFromSpecifierFields;

impl FormatNode for JsExportNamedFromSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedFromSpecifierFields {
            type_token,
            source_name,
            export_as,
        } = self.as_fields();

        let type_token = type_token.format_with_or_empty(formatter, |type_token| {
            formatted![formatter, type_token, space_token()]
        })?;

        let source_name = source_name.format(formatter)?;

        let export_as = export_as.format_with_or_empty(formatter, |export_as| {
            formatted![formatter, space_token(), export_as]
        })?;

        formatted![formatter, type_token, source_name, export_as]
    }
}
