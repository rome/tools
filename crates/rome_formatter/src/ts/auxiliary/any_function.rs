use crate::format_element::soft_line_indent_or_space;
use crate::{
    concat_elements, empty_element, format_elements, group_elements, space_token, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsAnyArrowFunctionParameters, JsAnyExpression, JsAnyFunction, JsAnyFunctionBody,
};

impl ToFormatElement for JsAnyFunction {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut tokens = vec![];

        if let Some(token) = self.async_token() {
            tokens.push(formatter.format_token(&token)?);
            tokens.push(space_token());
        }

        if let Some(function) = self.function_token()? {
            tokens.push(formatter.format_token(&function)?)
        }

        if let Some(token) = self.star_token() {
            tokens.push(formatter.format_token(&token)?);
        }

        tokens.push(match self {
            JsAnyFunction::JsArrowFunctionExpression(_) => empty_element(),
            _ => match self.id()? {
                Some(id) => format_elements![space_token(), formatter.format_node(&id)?],
                None => space_token(),
            },
        });

        tokens.push(match self.parameters()? {
            JsAnyArrowFunctionParameters::JsAnyBinding(binding) => {
                format_elements![token("("), formatter.format_node(&binding)?, token(")")]
            }
            JsAnyArrowFunctionParameters::JsParameters(params) => formatter.format_node(&params)?,
        });

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
            body_group.push(formatter.format_token(&arrow.fat_arrow_token()?)?);
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
        let body_has_soft_line_break = matches!(
            body,
            JsAnyFunctionBody::JsFunctionBody(_)
                | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsArrayExpression(_))
                | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsArrowFunctionExpression(_))
                | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsObjectExpression(_))
        );

        if body_has_soft_line_break {
            body_group.push(formatter.format_node(&self.body()?)?);
        } else {
            body_group.push(soft_line_indent_or_space(
                formatter.format_node(&self.body()?)?,
            ));
        }

        tokens.push(group_elements(concat_elements(body_group)));

        Ok(concat_elements(tokens))
    }
}

impl ToFormatElement for JsAnyFunctionBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyFunctionBody::JsFunctionBody(body) => body.to_format_element(formatter),
            JsAnyFunctionBody::JsAnyExpression(expr) => expr.to_format_element(formatter),
        }
    }
}
