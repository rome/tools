use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};

use rome_js_syntax::{JsAnyExpression, JsUnaryExpression};
use rome_js_syntax::JsUnaryExpressionFields;
use rome_js_syntax::T;
use rslint_parser::token_set;

impl ToFormatElement for JsUnaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsUnaryExpressionFields { operator, argument } = self.as_fields();

        let operator = operator?;
        let argument = argument?;

        // Insert a space between the operator and argument if its a keyword or
        // if the inner argument is a binary or pre-update operation
        let should_space = token_set![T![delete], T![void], T![typeof]].contains(operator.kind())
            || matches!(
                &argument,
                JsAnyExpression::JsUnaryExpression(_) | JsAnyExpression::JsPreUpdateExpression(_)
            );

        let space_or_empty = if should_space {
            space_token()
        } else {
            empty_element()
        };

        Ok(format_elements![
            operator.format(formatter)?,
            space_or_empty,
            argument.format(formatter)?,
        ])
    }
}
