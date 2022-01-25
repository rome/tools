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

        if let JsAnyFunction::JsArrowFunctionExpression(arrow) = self {
            tokens.push(formatter.format_token(&arrow.fat_arrow_token()?)?);
            tokens.push(space_token());
        }

        let body = self.body()?;
        let body_has_soft_line_break = matches!(
            body,
            JsAnyFunctionBody::JsFunctionBody(_)
                | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsArrayExpression(_))
                | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsArrowFunctionExpression(_))
                | JsAnyFunctionBody::JsAnyExpression(JsAnyExpression::JsObjectExpression(_))
        );

        // If the body has a soft line break, we do not insert one.
        if body_has_soft_line_break {
            tokens.push(formatter.format_node(&self.body()?)?);
        } else {
            tokens.push(soft_line_indent_or_space(
                formatter.format_node(&self.body()?)?,
            ));
        }

        Ok(group_elements(concat_elements(tokens)))
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
