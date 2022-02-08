use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, group_elements, soft_line_indent_or_space, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsAssignmentExpression;

impl ToFormatElement for JsAssignmentExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(format_elements![
            self.left().format(formatter)?,
            space_token(),
            self.operator_token().format(formatter)?,
            group_elements(soft_line_indent_or_space(self.right().format(formatter)?)),
        ]))
    }
}
