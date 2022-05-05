use crate::utils::is_simple_expression;
use crate::{
    format_elements, group_elements, soft_block_indent, space_token, token, Format, FormatElement,
    FormatNode, Formatter,
};
use rome_formatter::FormatResult;

use crate::format_traits::FormatWith;
use rome_js_syntax::{JsAnyExpression, JsUnaryExpression};
use rome_js_syntax::{JsAwaitExpression, JsPreUpdateOperator};
use rome_js_syntax::{JsUnaryExpressionFields, JsUnaryOperator};
use rome_rowan::AstNode;

impl FormatNode for JsUnaryExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsUnaryExpressionFields {
            operator_token,
            argument,
        } = self.as_fields();

        let operation = self.operator()?;
        let operator_token = operator_token?;
        let argument = argument?;

        // Insert a space between the operator and argument if its a keyword
        let is_keyword_operator = matches!(
            operation,
            JsUnaryOperator::Delete | JsUnaryOperator::Void | JsUnaryOperator::Typeof
        );

        if is_keyword_operator {
            let needs_parenthesis = JsAwaitExpression::can_cast(argument.syntax().kind());

            return Ok(format_elements![
                operator_token.format(formatter)?,
                space_token(),
                argument.format_with(formatter, |argument| {
                    if needs_parenthesis {
                        format_elements![token("("), argument, token(")"),]
                    } else {
                        argument
                    }
                })?,
            ]);
        }

        // Parenthesize the inner expression if it's a binary or pre-update
        // operation with an ambiguous operator (+ and ++ or - and --)
        let is_ambiguous_expression = match &argument {
            JsAnyExpression::JsUnaryExpression(expr) => {
                let inner_op = expr.operator()?;
                matches!(
                    (operation, inner_op),
                    (JsUnaryOperator::Plus, JsUnaryOperator::Plus)
                        | (JsUnaryOperator::Minus, JsUnaryOperator::Minus)
                )
            }
            JsAnyExpression::JsPreUpdateExpression(expr) => {
                let inner_op = expr.operator()?;
                matches!(
                    (operation, inner_op),
                    (JsUnaryOperator::Plus, JsPreUpdateOperator::Increment)
                        | (JsUnaryOperator::Minus, JsPreUpdateOperator::Decrement)
                )
            }
            JsAnyExpression::JsAwaitExpression(_) => true,
            _ => false,
        };

        dbg!(&is_ambiguous_expression);
        if is_ambiguous_expression {
            let parenthesized = if is_simple_expression(argument.clone())? {
                format_elements![
                    operator_token.format(formatter)?,
                    token("("),
                    argument.format(formatter)?,
                    token(")"),
                ]
            } else {
                format_elements![
                    operator_token.format(formatter)?,
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
            operator_token.format(formatter)?,
            argument.format(formatter)?,
        ])
    }
}
