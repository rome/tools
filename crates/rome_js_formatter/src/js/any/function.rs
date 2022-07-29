use crate::prelude::*;
use crate::utils::is_simple_expression;
use rome_formatter::{format_args, write};
use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyExpression, JsAnyFunction, JsAnyFunctionBody,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyFunction;

impl FormatRule<JsAnyFunction> for FormatJsAnyFunction {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsAnyFunction, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(async_token) = node.async_token() {
            write!(f, [async_token.format(), space()])?;
        }

        write!(
            f,
            [node.function_token().format(), node.star_token().format()]
        )?;

        if !matches!(node, JsAnyFunction::JsArrowFunctionExpression(_)) {
            match node.id()? {
                Some(id) => {
                    write!(f, [space(), id.format()])?;
                }
                None => {
                    write!(f, [space()])?;
                }
            }
        }

        write!(f, [node.type_parameters().format()])?;

        match node.parameters()? {
            JsAnyArrowFunctionParameters::JsAnyBinding(binding) => write!(
                f,
                [format_parenthesize(
                    binding.syntax().first_token().as_ref(),
                    &format_args![binding.format(), if_group_breaks(&text(",")),],
                    binding.syntax().last_token().as_ref(),
                )
                .grouped_with_soft_block_indent()]
            )?,
            JsAnyArrowFunctionParameters::JsParameters(params) => write![f, [params.format()]]?,
        }

        write![f, [node.return_type_annotation().format(), space()]]?;

        // We create a new group for everything after the parameters. That way if the parameters
        // get broken, we don't line break the arrow and the body if they can fit on the same line.
        // For instance:
        //
        //   (
        //     a = [abcdefghijklmnopqrstuvwxyz123456789],
        //     b = [abcdefghijklmnopqrstuvwxyz123456789],
        //   ) =>
        //     a + b
        //
        // The line break for `a + b` is not necessary
        //
        if let JsAnyFunction::JsArrowFunctionExpression(arrow) = node {
            write![f, [arrow.fat_arrow_token().format(), space()]]?;
        }

        let body = node.body()?;
        // With arrays, arrow functions and objects, they have a natural line breaking strategy:
        // Arrays and objects become blocks:
        //
        //    [
        //      100000,
        //      200000,
        //      300000
        //    ]
        //
        // Arrow functions get line broken after the `=>`:
        //
        //  (foo) => (bar) =>
        //     (foo + bar) * (foo + bar)
        //
        // Therefore if our body is an arrow function, array, or object, we
        // do not have a soft line break after the arrow because the body is
        // going to get broken anyways.
        //
        // TODO: Explore the concept of hierarchical line breaking
        //   or vertical stacking for arrow functions
        let body_has_soft_line_break = match body {
            JsAnyFunctionBody::JsFunctionBody(_) => true,
            JsAnyFunctionBody::JsAnyExpression(expr) => match expr {
                JsAnyExpression::JsArrowFunctionExpression(_) => true,
                JsAnyExpression::JsParenthesizedExpression(_) => true,
                JsAnyExpression::JsxTagExpression(_) => true,
                expr => is_simple_expression(&expr)?,
            },
        };

        if body_has_soft_line_break {
            write![f, [node.body().format()]]?;
        } else {
            write!(
                f,
                [group(&soft_line_indent_or_space(&node.body().format()))]
            )?;
        }

        Ok(())
    }
}
