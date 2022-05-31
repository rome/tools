use crate::prelude::*;
use crate::utils::FormatLiteralStringToken;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsLiteralMemberNameFields;
use rome_js_syntax::{JsLiteralMemberName, JsSyntaxKind};

impl FormatNodeFields<JsLiteralMemberName> for FormatNodeRule<JsLiteralMemberName> {
    fn format_fields(node: &JsLiteralMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLiteralMemberNameFields { value } = node.as_fields();

        let value = value?;

        match value.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                write!(f, [FormatLiteralStringToken::from_string(&value)])
            }
            _ => write![f, [value.format()]],
        }
    }
}
