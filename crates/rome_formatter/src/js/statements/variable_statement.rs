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

impl ToFormatElement for JsVariableStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.declarations().format(formatter)?,
            self.semicolon_token().format_or(formatter, || token(";"))?,
        ])
    }
}
