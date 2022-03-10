use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_initializer_clause;
use crate::{
    format_elements, hard_group_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::JsVariableDeclarator;
use rome_js_syntax::JsVariableDeclaratorFields;

impl ToFormatElement for JsVariableDeclarator {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclaratorFields {
            id,
            variable_annotation,
            initializer,
        } = self.as_fields();

        let initializer = format_initializer_clause(formatter, initializer)?;

        Ok(format_elements![
            hard_group_elements(id.format(formatter)?),
            variable_annotation.format_or_empty(formatter)?,
            initializer
        ])
    }
}
