use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsForVariableDeclaration;
use rome_js_syntax::JsForVariableDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsForVariableDeclaration;

impl FormatNodeRule<JsForVariableDeclaration> for FormatJsForVariableDeclaration {
    fn fmt_fields(&self, node: &JsForVariableDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForVariableDeclarationFields {
            kind_token,
            declarator,
        } = node.as_fields();

        write![f, [kind_token.format(), space(), declarator.format(),]]
    }
}
