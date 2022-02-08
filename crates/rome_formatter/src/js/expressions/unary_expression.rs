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

impl ToFormatElement for JsUnaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let operator = self.operator()?;
        let space_or_empty =
            if token_set![T![delete], T![void], T![typeof]].contains(operator.kind()) {
                space_token()
            } else {
                empty_element()
            };
        Ok(format_elements![
            self.operator().format(formatter)?,
            space_or_empty,
            self.argument().format(formatter)?,
        ])
    }
}
