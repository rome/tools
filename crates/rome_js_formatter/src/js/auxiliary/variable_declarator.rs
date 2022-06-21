use crate::prelude::*;
use crate::utils::FormatInitializerClause;

use rome_formatter::write;
use rome_js_syntax::JsVariableDeclarator;
use rome_js_syntax::JsVariableDeclaratorFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsVariableDeclarator;

impl FormatNodeRule<JsVariableDeclarator> for FormatJsVariableDeclarator {
    fn fmt_fields(node: &JsVariableDeclarator, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVariableDeclaratorFields {
            id,
            variable_annotation,
            initializer,
        } = node.as_fields();

        write![
            f,
            [
                id.format(),
                variable_annotation.format(),
                FormatInitializerClause::new(initializer.as_ref())
            ]
        ]
    }
}
