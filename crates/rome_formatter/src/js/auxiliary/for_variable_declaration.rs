use rslint_parser::ast::{JsAnyForInOrOfInitializer, JsForInStatement, JsForVariableDeclaration};

use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, soft_block_indent, soft_line_break_or_space, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsForVariableDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.kind_token().format(formatter)?,
            space_token(),
            self.declaration().format(formatter)?,
        ])
    }
}
