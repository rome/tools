use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
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
                [FormatLiteralStringToken::new(
                    &key,
                    StringLiteralParentKind::Expression
                )]
            ]?,
            _ => formatted![formatter, [key.format()]]?,
        };

        formatted![
            formatter,
            [
                formatted_key,
                colon_token.format(),
                space_token(),
                FormatLiteralStringToken::new(&value_token?, StringLiteralParentKind::Expression)
            ]
        ]
    }
}
