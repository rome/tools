use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
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
                    [FormatLiteralStringToken::from_parent_expression(&value)]
                ]
            }
            _ => formatted![formatter, [value.format()]],
        }
    }
}
