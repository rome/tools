use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use crate::utils::format_string_literal_token;
use rome_js_syntax::JsImportAssertionEntryFields;
use rome_js_syntax::{JsImportAssertionEntry, JsSyntaxKind};

impl ToFormatElement for JsImportAssertionEntry {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsImportAssertionEntryFields {
            key,
            colon_token,
            value_token,
        } = self.as_fields();

        let key = key?;

        let formatted_key = match key.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => format_string_literal_token(key, formatter),
            _ => key.format(formatter)?,
        };

        Ok(format_elements![
            formatted_key,
            colon_token.format(formatter)?,
            space_token(),
            format_string_literal_token(value_token?, formatter),
        ])
    }
}
