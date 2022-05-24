use crate::prelude::*;

use crate::generated::FormatJsAnyFunction;
use crate::utils::is_simple_expression;
use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyExpression, JsAnyFunction, JsAnyFunctionBody,
};

impl FormatRule<JsAnyFunction> for FormatJsAnyFunction {
    type Options = JsFormatOptions;

    fn format(
        node: &JsAnyFunction,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let mut tokens = vec![];

        tokens.push(formatted![
            formatter,
            [node
                .async_token()
                .format()
                .with_or_empty(|token| { formatted![formatter, [token, space_token()]] })]
        ]?);

        tokens.push(formatted![formatter, [node.function_token().format()]]?);
        tokens.push(formatted![formatter, [node.star_token().format()]]?);

        tokens.push(match node {
            JsAnyFunction::JsArrowFunctionExpression(_) => empty_element(),
            _ => formatted![
                formatter,
                [node
                    .id()
                    .format()
                    .with_or(|id| formatted![formatter, [space_token(), id]], space_token,)]
            ]?,
        });

        tokens.push(formatted![formatter, [node.type_parameters().format()]]?);

        tokens.push(match node.parameters()? {
            JsAnyArrowFunctionParameters::JsAnyBinding(binding) => group_elements(formatted![
                formatter,
                [
                    token("("),
                    soft_block_indent(formatted![
                        formatter,
                        [binding.format(), if_group_breaks(token(",")),]
                    ]?),
                    token(")"),
                ]
            ]?),
            JsAnyArrowFunctionParameters::JsParameters(params) => {
                formatted![formatter, [params.format()]]?
            }
        });

        tokens.push(formatted![
            formatter,
            [node.return_type_annotation().format()]
        ]?);

        tokens.push(space_token());

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
            tokens.push(formatted![
                formatter,
                [arrow.fat_arrow_token().format(), space_token()]
            ]?);
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
            tokens.push(formatted![formatter, [node.body().format()]]?);
        } else {
            tokens.push(group_elements(soft_line_indent_or_space(formatted![
                formatter,
                [node.body().format()]
            ]?)));
        }

        Ok(concat_elements(tokens))
    }
}
