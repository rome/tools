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

impl ToFormatElement for JsNewExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let arguments = self
            .arguments()
            .format_or(formatter, || format_elements![token("("), token(")")])?;

        Ok(format_elements![
            self.new_token().format(formatter)?,
            // TODO handle TsTypeArgs
            space_token(),
            self.callee().format(formatter)?,
            arguments,
        ])
    }
}
