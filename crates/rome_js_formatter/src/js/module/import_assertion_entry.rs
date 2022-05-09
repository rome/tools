use crate::prelude::*;

use crate::utils::format_string_literal_token;
use crate::FormatNodeFields;
use rome_js_syntax::JsImportAssertionEntryFields;
use rome_js_syntax::{JsImportAssertionEntry, JsSyntaxKind};

impl FormatNodeFields<JsImportAssertionEntry> for FormatNodeRule<JsImportAssertionEntry> {
    fn format_fields(
        node: &JsImportAssertionEntry,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsImportAssertionEntryFields {
            key,
            colon_token,
            value_token,
        } = node.as_fields();

        let key = key?;

        let formatted_key = match key.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => format_string_literal_token(key, formatter),
            _ => formatted![formatter, key.format()]?,
        };

        formatted![
            formatter,
            formatted_key,
            colon_token.format(),
            space_token(),
            format_string_literal_token(value_token?, formatter),
        ]
    }
}
