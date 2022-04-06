use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::is_simple_expression;
use crate::{
    concat_elements, empty_element, format_elements, group_elements, hard_group_elements,
    if_group_breaks, soft_block_indent, soft_line_indent_or_space, space_token, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyExpression, JsAnyFunction, JsAnyFunctionBody,
};

impl ToFormatElement for JsAnyFunction {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut tokens = vec![];

        tokens.push(
            self.async_token()
                .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?,
        );

        tokens.push(self.function_token().format_or_empty(formatter)?);
        tokens.push(self.star_token().format_or_empty(formatter)?);

        tokens.push(match self {
            JsAnyFunction::JsArrowFunctionExpression(_) => empty_element(),
            _ => self.id().format_with_or(
                formatter,
                |id| format_elements![space_token(), id],
                space_token,
            )?,
        });

        tokens.push(self.type_parameters().format_or_empty(formatter)?);

        tokens.push(match self.parameters()? {
            JsAnyArrowFunctionParameters::JsAnyBinding(binding) => {
                group_elements(format_elements![
                    token("("),
                    soft_block_indent(format_elements![
                        binding.format(formatter)?,
                        if_group_breaks(token(",")),
                    ]),
                    token(")"),
                ])
            }
            JsAnyArrowFunctionParameters::JsParameters(params) => params.format(formatter)?,
        });

        tokens.push(self.return_type_annotation().format_or_empty(formatter)?);

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
        let mut body_group = vec![];
        if let JsAnyFunction::JsArrowFunctionExpression(arrow) = self {
            body_group.push(arrow.fat_arrow_token().format(formatter)?);
            body_group.push(space_token());
        }

        let body = self.body()?;
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
                expr => is_simple_expression(expr)?,
            },
        };

        if body_has_soft_line_break {
            body_group.push(self.body().format(formatter)?);
        } else {
            body_group.push(soft_line_indent_or_space(self.body().format(formatter)?));
        }

        tokens.push(group_elements(concat_elements(body_group)));

        Ok(hard_group_elements(concat_elements(tokens)))
    }
}
