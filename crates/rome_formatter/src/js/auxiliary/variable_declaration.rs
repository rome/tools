use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsVariableDeclaration;

impl ToFormatElement for JsVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let initializer = self
            .initializer()
            .format_with_or_empty(formatter, |initializer| {
                format_elements![space_token(), initializer]
            })?;

        Ok(format_elements![self.id().format(formatter)?, initializer])
    }
}
