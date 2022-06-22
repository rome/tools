use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, FormatWithSemicolon, StringLiteralParentKind};
use rome_formatter::write;

use rome_js_syntax::JsDirective;
use rome_js_syntax::JsDirectiveFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsDirective;

impl FormatNodeRule<JsDirective> for FormatJsDirective {
    fn fmt_fields(&self, node: &JsDirective, f: &mut JsFormatter) -> FormatResult<()> {
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
