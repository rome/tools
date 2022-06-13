use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::FormatNodeFields;
use rome_js_syntax::JsLiteralExportName;
use rome_js_syntax::JsLiteralExportNameFields;
use rome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL;

impl FormatNodeFields<JsLiteralExportName> for FormatNodeRule<JsLiteralExportName> {
    fn fmt_fields(node: &JsLiteralExportName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLiteralExportNameFields { value } = node.as_fields();

        let value = value?;

        if value.kind() == JS_STRING_LITERAL {
            FormatLiteralStringToken::new(&value, StringLiteralParentKind::Expression).fmt(f)
        } else {
            value.format().fmt(f)
        }
    }
}
