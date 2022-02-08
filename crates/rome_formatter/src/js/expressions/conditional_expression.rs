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

impl ToFormatElement for JsConditionalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.test().format(formatter)?,
            space_token(),
            self.question_mark_token().format(formatter)?,
            space_token(),
            self.consequent().format(formatter)?,
            space_token(),
            self.colon_token().format(formatter)?,
            space_token(),
            self.alternate().format(formatter)?,
        ])
    }
}
