use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::{is_simple_expression, resolve_expression, starts_with_no_lookahead_token};
use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyExpression, JsAnyFunctionBody, JsArrowFunctionExpression,
    JsArrowFunctionExpressionFields,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsArrowFunctionExpression;

impl FormatNodeRule<JsArrowFunctionExpression> for FormatJsArrowFunctionExpression {
    fn fmt_fields(
        &self,
        node: &JsArrowFunctionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        use self::JsAnyExpression::*;
        use JsAnyFunctionBody::*;

        let JsArrowFunctionExpressionFields {
            async_token,
            type_parameters,
            parameters,
            return_type_annotation,
            fat_arrow_token,
            body,
        } = node.as_fields();

        if let Some(async_token) = async_token {
            write!(f, [async_token.format(), space()])?;
        }

        write!(f, [type_parameters.format()])?;

        match parameters? {
            JsAnyArrowFunctionParameters::JsAnyBinding(binding) => write!(
                f,
                [format_parenthesize(
                    binding.syntax().first_token().as_ref(),
                    &format_args![binding.format(), if_group_breaks(&text(",")),],
                    binding.syntax().last_token().as_ref(),
                )
                .grouped_with_soft_block_indent()]
            )?,
            JsAnyArrowFunctionParameters::JsParameters(params) => {
                write![f, [group(&params.format())]]?
            }
        }

        write![
            f,
            [
                return_type_annotation.format(),
                space(),
                fat_arrow_token.format(),
                space()
            ]
        ]?;

        let body = body?;

        // With arrays, arrow selfs and objects, they have a natural line breaking strategy:
        // Arrays and objects become blocks:
        //
        //    [
        //      100000,
        //      200000,
        //      300000
        //    ]
        //
        // Arrow selfs get line broken after the `=>`:
        //
        //  (foo) => (bar) =>
        //     (foo + bar) * (foo + bar)
        //
        // Therefore if our body is an arrow self, array, or object, we
        // do not have a soft line break after the arrow because the body is
        // going to get broken anyways.
        let (body_has_soft_line_break, should_add_parens) = match &body {
            JsFunctionBody(_) => (true, false),
            JsAnyExpression(expr) => match expr {
                JsArrowFunctionExpression(_)
                | JsArrayExpression(_)
                | JsObjectExpression(_)
                | JsxTagExpression(_) => (true, false),
                JsParenthesizedExpression(expression) => {
                    let resolved = resolve_expression(expression.expression()?);

                    match resolved {
                        JsConditionalExpression(conditional) => {
                            (false, !starts_with_no_lookahead_token(conditional.into())?)
                        }
                        _ => (true, false),
                    }
                }
                JsConditionalExpression(conditional) => (
                    false,
                    !starts_with_no_lookahead_token(conditional.clone().into())?,
                ),
                expr => (is_simple_expression(expr)?, false),
            },
        };

        if body_has_soft_line_break {
            write![f, [body.format()]]
        } else {
            write!(
                f,
                [group(&soft_line_indent_or_space(&format_with(|f| {
                    if should_add_parens {
                        write!(f, [if_group_fits_on_line(&text("("))])?;
                    }

                    write!(f, [body.format()])?;

                    if should_add_parens {
                        write!(f, [if_group_fits_on_line(&text(")"))])?;
                    }

                    Ok(())
                })))]
            )
        }
    }
}
