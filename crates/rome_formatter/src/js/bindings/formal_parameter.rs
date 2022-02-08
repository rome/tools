use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsAnyFormalParameter, JsAnyParameter, JsFormalParameter, JsParameters, JsRestParameter,
};

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
