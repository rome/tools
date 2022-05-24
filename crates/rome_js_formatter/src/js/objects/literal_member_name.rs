use crate::prelude::*;

use crate::utils::format_string_literal_token;
use crate::FormatNodeFields;
use rome_js_syntax::JsLiteralMemberNameFields;
use rome_js_syntax::{JsLiteralMemberName, JsSyntaxKind};

impl FormatNodeFields<JsLiteralMemberName> for FormatNodeRule<JsLiteralMemberName> {
    fn format_fields(
        node: &JsLiteralMemberName,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsLiteralMemberNameFields { value } = node.as_fields();

        let value = value?;

        match value.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => Ok(format_string_literal_token(value, formatter)),
            _ => formatted![formatter, [value.format()]],
        }
    }
}
