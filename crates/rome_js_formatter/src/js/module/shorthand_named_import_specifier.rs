use crate::prelude::*;

use rome_js_syntax::JsShorthandNamedImportSpecifier;
use rome_js_syntax::JsShorthandNamedImportSpecifierFields;

impl FormatNode for JsShorthandNamedImportSpecifier {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsShorthandNamedImportSpecifierFields {
            type_token,
            local_name,
        } = self.as_fields();

        let type_token =
            type_token.with_or_empty(|token| formatted![formatter, token, space_token()]);

        let local_name = local_name.format(formatter)?;

        formatted![formatter, type_token, local_name]
    }
}
