use crate::prelude::*;
use crate::utils::format_initializer_clause;

use rome_js_syntax::JsVariableDeclarator;
use rome_js_syntax::JsVariableDeclaratorFields;

impl FormatNode for JsVariableDeclarator {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclaratorFields {
            id,
            variable_annotation,
            initializer,
        } = self.as_fields();

        let initializer = format_initializer_clause(formatter, initializer)?;

        formatted![
            formatter,
            hard_group_elements(id.format(formatter)?),
            variable_annotation,
            initializer
        ]
    }
}
