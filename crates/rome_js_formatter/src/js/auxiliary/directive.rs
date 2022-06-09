use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, FormatWithSemicolon, StringLiteralParentKind};
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsDirective;
use rome_js_syntax::JsDirectiveFields;

impl FormatNodeFields<JsDirective> for FormatNodeRule<JsDirective> {
    fn fmt_fields(node: &JsDirective, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDirectiveFields {
            value_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &FormatLiteralStringToken::new(&value_token?, StringLiteralParentKind::Directive),
                semicolon_token.as_ref()
            )]
        )
    }
}
