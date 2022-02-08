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

impl ToFormatElement for JsComputedMemberExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let optional_chain_token = self.optional_chain_token().format_or_empty(formatter)?;

        Ok(format_elements![
            self.object().format(formatter)?,
            optional_chain_token,
            self.l_brack_token().format(formatter)?,
            self.member().format(formatter)?,
            self.r_brack_token().format(formatter)?,
        ])
    }
}
