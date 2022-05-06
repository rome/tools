use crate::format_traits::FormatOptional;
use rome_formatter::FormatResult;

use crate::{
    formatted, space_token, Format, FormatElement, FormatNode, Formatter,
};

use rome_js_syntax::JsShorthandNamedImportSpecifier;
use rome_js_syntax::JsShorthandNamedImportSpecifierFields;

impl FormatNode for JsShorthandNamedImportSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsShorthandNamedImportSpecifierFields {
            type_token,
            local_name,
        } = self.as_fields();

        let type_token = type_token.format_with_or_empty(formatter, |token| {
            formatted![formatter, token, space_token()]
        })?;

        let local_name = local_name.format(formatter)?;

        formatted![formatter, type_token, local_name]
    }
}
