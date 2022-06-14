use crate::prelude::*;
use crate::utils::FormatInitializerClause;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsVariableDeclarator;
use rome_js_syntax::JsVariableDeclaratorFields;

impl FormatNodeFields<JsVariableDeclarator> for FormatNodeRule<JsVariableDeclarator> {
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
