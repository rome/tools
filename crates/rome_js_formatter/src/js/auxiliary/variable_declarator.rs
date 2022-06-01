use crate::prelude::*;
use crate::utils::format_initializer_clause;

use crate::FormatNodeFields;
use rome_js_syntax::JsVariableDeclarator;
use rome_js_syntax::JsVariableDeclaratorFields;

impl FormatNodeFields<JsVariableDeclarator> for FormatNodeRule<JsVariableDeclarator> {
    fn format_fields(
        node: &JsVariableDeclarator,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsVariableDeclaratorFields {
            id,
            variable_annotation,
            initializer,
        } = node.as_fields();

        let initializer = format_initializer_clause(formatter, initializer)?;

        formatted![
            formatter,
            [id.format(), variable_annotation.format(), initializer]
        ]
    }
}
