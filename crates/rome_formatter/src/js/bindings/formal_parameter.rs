use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use crate::utils::format_initializer_clause;
use rslint_parser::ast::JsFormalParameter;

impl ToFormatElement for JsFormalParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let type_annotation = self.type_annotation().format_or_empty(formatter)?;
        let initializer = format_initializer_clause(formatter, self.initializer())?;

        Ok(format_elements![
            self.binding().format(formatter)?,
            type_annotation,
            initializer
        ])
    }
}
