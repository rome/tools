use crate::format_traits::FormatOptional;
use crate::utils::format_initializer_clause;
use crate::{
    formatted, hard_group_elements, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
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
            variable_annotation.format_or_empty(formatter)?,
            initializer
        ]
    }
}
