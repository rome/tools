use crate::format_traits::FormatOptional;

use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsExportNamedSpecifier;
use rome_js_syntax::JsExportNamedSpecifierFields;

impl FormatNode for JsExportNamedSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedSpecifierFields {
            type_token,
            local_name,
            as_token,
            exported_name,
        } = self.as_fields();

        let type_token = type_token.format_with_or_empty(formatter, |type_token| {
            format_elements![type_token, space_token()]
        })?;
        let as_token = as_token.format(formatter)?;
        let local_name = local_name.format(formatter)?;
        let exported_name = exported_name.format(formatter)?;

        Ok(format_elements![
            type_token,
            local_name,
            space_token(),
            as_token,
            space_token(),
            exported_name
        ])
    }
}
