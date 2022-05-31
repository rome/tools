use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::FormatNodeFields;
use rome_js_syntax::JsImportAssertionEntryFields;
use rome_js_syntax::{JsImportAssertionEntry, JsSyntaxKind};

impl FormatNodeFields<JsImportAssertionEntry> for FormatNodeRule<JsImportAssertionEntry> {
    fn format_fields(
        node: &JsImportAssertionEntry,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsImportAssertionEntryFields {
            key,
            colon_token,
            value_token,
        } = node.as_fields();

        let key = key?;

        let formatted_key = match key.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => formatted![
                formatter,
                [FormatLiteralStringToken::from_parent_expression(&key)]
            ]?,
            _ => formatted![formatter, [key.format()]]?,
        };

        formatted![
            formatter,
            [
                formatted_key,
                colon_token.format(),
                space_token(),
                FormatLiteralStringToken::from_parent_expression(&value_token?)
            ]
        ]
    }
}
