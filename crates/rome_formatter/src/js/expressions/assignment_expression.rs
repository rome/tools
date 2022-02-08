use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    empty_element, format_elements, group_elements, soft_line_indent_or_space, space_token, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsAnyExpression, JsAnyInProperty, JsAssignmentExpression, JsAwaitExpression,
    JsBinaryExpression, JsComputedMemberExpression, JsConditionalExpression, JsInExpression,
    JsInstanceofExpression, JsLogicalExpression, JsNewExpression, JsParenthesizedExpression,
    JsThisExpression, JsUnaryExpression, JsYieldArgument, JsYieldExpression, NewTarget,
};

use rslint_parser::{token_set, T};

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
