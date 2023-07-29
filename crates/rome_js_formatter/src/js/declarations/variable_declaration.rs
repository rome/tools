use crate::prelude::*;
use rome_formatter::{format_args, write};

use rome_js_syntax::JsVariableDeclaration;
use rome_js_syntax::JsVariableDeclarationFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVariableDeclaration;

impl FormatNodeRule<JsVariableDeclaration> for FormatJsVariableDeclaration {
    fn fmt_fields(&self, node: &JsVariableDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVariableDeclarationFields {
            await_token,
            kind,
            declarators,
        } = node.as_fields();

        if let Some(await_token) = await_token {
            write![
                f,
                [group(&format_args![
                    await_token.format(),
                    space(),
                    kind.format(),
                    space(),
                    declarators.format()
                ])]
            ]
        } else {
            write![
                f,
                [group(&format_args![
                    kind.format(),
                    space(),
                    declarators.format()
                ])]
            ]
        }
    }
}
