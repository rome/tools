use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsFormalParameter;

impl ToFormatElement for JsFormalParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let initializer = self
            .initializer()
            .format_with_or_empty(formatter, |initializer| {
                format_elements![space_token(), initializer]
            })?;

        Ok(format_elements![
            self.binding().format(formatter)?,
            initializer
        ])
    }
}
