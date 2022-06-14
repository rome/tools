use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsLiteralMemberNameFields;
use rome_js_syntax::{JsLiteralMemberName, JsSyntaxKind};

impl FormatNodeFields<JsLiteralMemberName> for FormatNodeRule<JsLiteralMemberName> {
    fn fmt_fields(node: &JsLiteralMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLiteralMemberNameFields { value } = node.as_fields();

        let value = value?;

        match value.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                write![
                    f,
                    [FormatLiteralStringToken::new(
                        &value,
                        StringLiteralParentKind::Expression
                    )]
                ]
            }
            _ => write![f, [value.format()]],
        }
    }
}
