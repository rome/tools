use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    empty_element, format_elements, group_elements, indent, join_elements,
    soft_line_break_or_space, space_token, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rslint_parser::{
    ast::{JsVariableDeclaration, JsVariableDeclarations, JsVariableStatement},
    AstSeparatedList,
};

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
