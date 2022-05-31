use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatLiteralStringToken;
use crate::FormatNodeFields;
use rome_js_syntax::JsImportAssertionEntryFields;
use rome_js_syntax::{JsImportAssertionEntry, JsSyntaxKind};

impl FormatNodeFields<JsImportAssertionEntry> for FormatNodeRule<JsImportAssertionEntry> {
    fn format_fields(node: &JsImportAssertionEntry, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportAssertionEntryFields {
            key,
            colon_token,
            value_token,
        } = node.as_fields();

        let key = key?;

        match key.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                write!(f, [FormatLiteralStringToken::from_string(&key)])?;
            }
            _ => {
                write![f, [key.format()]]?;
            }
        };

        write![
            f,
            [
                colon_token.format(),
                space_token(),
                FormatLiteralStringToken::from_string(&value_token?),
            ]
        ]
    }
}
