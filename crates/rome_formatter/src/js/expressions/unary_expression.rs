use crate::formatter_traits::FormatTokenAndNode;

use crate::utils::is_simple_expression;
use crate::{
    format_elements, group_elements, soft_block_indent, space_token, token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsUnaryExpressionFields;
use rome_js_syntax::T;
use rome_js_syntax::{JsAnyExpression, JsUnaryExpression};

impl ToFormatElement for JsUnaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsUnaryExpressionFields { operator, argument } = self.as_fields();

        let operator = operator?;
        let argument = argument?;

        // Insert a space between the operator and argument if its a keyword
        let is_keyword_operator = matches!(operator.kind(), T![delete] | T![void] | T![typeof]);
        if is_keyword_operator {
            return Ok(format_elements![
                operator.format(formatter)?,
                space_token(),
                argument.format(formatter)?,
            ]);
        }

        // Parenthesize the inner expression if it's a binary or pre-update
        // operation with an ambiguous operator (+ and ++ or - and --)
        let is_ambiguous_expression = match &argument {
            JsAnyExpression::JsUnaryExpression(expr) => {
                let inner_op = expr.operator()?;
                matches!(
                    (operator.kind(), inner_op.kind()),
                    (T![+], T![+]) | (T![-], T![-])
                )
            }
            JsAnyExpression::JsPreUpdateExpression(expr) => {
                let inner_op = expr.operator()?;
                matches!(
                    (operator.kind(), inner_op.kind()),
                    (T![+], T![++]) | (T![-], T![--])
                )
            }
            _ => false,
        };

        if is_ambiguous_expression {
            let parenthesized = if is_simple_expression(argument.clone())? {
                format_elements![
                    operator.format(formatter)?,
                    token("("),
                    argument.format(formatter)?,
                    token(")"),
                ]
            } else {
                format_elements![
                    operator.format(formatter)?,
                    group_elements(format_elements![
                        token("("),
                        soft_block_indent(argument.format(formatter)?),
                        token(")"),
                    ]),
                ]
            };

            return Ok(parenthesized);
        }

        Ok(format_elements![
            operator.format(formatter)?,
            argument.format(formatter)?,
        ])
    }
}
