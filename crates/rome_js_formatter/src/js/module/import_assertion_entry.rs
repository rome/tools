use crate::prelude::*;
use rome_formatter::write;

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use rome_js_syntax::JsImportAssertionEntryFields;
use rome_js_syntax::{JsImportAssertionEntry, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsImportAssertionEntry;

impl FormatNodeRule<JsImportAssertionEntry> for FormatJsImportAssertionEntry {
    fn fmt_fields(node: &JsImportAssertionEntry, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportAssertionEntryFields {
            key,
            colon_token,
            value_token,
        } = node.as_fields();

        let key = key?;

        match key.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                write!(
                    f,
                    [FormatLiteralStringToken::new(
                        &key,
                        StringLiteralParentKind::Expression
                    )]
                )?;
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
                FormatLiteralStringToken::new(&value_token?, StringLiteralParentKind::Expression),
            ]
        ]
    }
}
