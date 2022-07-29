use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsVariableDeclaration;
use rome_js_syntax::JsVariableDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsVariableDeclaration;

impl FormatNodeRule<JsVariableDeclaration> for FormatJsVariableDeclaration {
    fn fmt_fields(&self, node: &JsVariableDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVariableDeclarationFields { kind, declarators } = node.as_fields();

        write![f, [kind.format(), space(), declarators.format(),]]
    }
}
