use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_js_syntax::JsLiteralMemberNameFields;
use rome_js_syntax::{JsLiteralMemberName, JsSyntaxKind};

impl FormatNodeFields<JsLiteralMemberName> for FormatNodeRule<JsLiteralMemberName> {
    fn format_fields(
        node: &JsLiteralMemberName,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsLiteralMemberNameFields { value } = node.as_fields();

        let value = value?;

        match value.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                formatted![
                    formatter,
                    [FormatLiteralStringToken::new(
                        &value,
                        StringLiteralParentKind::Expression
                    )]
                ]
            }
            _ => formatted![formatter, [value.format()]],
        }
    }
}
