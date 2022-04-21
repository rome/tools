use crate::format_traits::FormatOptional;

use crate::{
    format_elements, space_token, Format, FormatElement, FormatNode, FormatResult, Formatter,
};

use rome_js_syntax::JsExportNamedShorthandSpecifier;
use rome_js_syntax::JsExportNamedShorthandSpecifierFields;

impl FormatNode for JsExportNamedShorthandSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedShorthandSpecifierFields { type_token, name } = self.as_fields();

        let type_token = type_token.format_with_or_empty(formatter, |type_token| {
            format_elements![type_token, space_token()]
        })?;
        let name = name.format(formatter)?;

        Ok(format_elements![type_token, name])
    }
}
