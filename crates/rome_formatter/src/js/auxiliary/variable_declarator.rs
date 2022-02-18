use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_initializer_clause;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsVariableDeclarator;
use rslint_parser::ast::JsVariableDeclaratorFields;

impl ToFormatElement for JsVariableDeclarator {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclaratorFields {
            id,
            variable_annotation,
            initializer,
        } = self.as_fields();

        let initializer = format_initializer_clause(formatter, initializer)?;

        Ok(format_elements![id.format(formatter)?, initializer])
    }
}
