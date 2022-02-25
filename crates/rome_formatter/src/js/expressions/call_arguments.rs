use crate::{format_elements, hard_group_elements};
use crate::{
    formatter_traits::FormatTokenAndNode, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{
    JsAnyFunction, JsAnyFunctionBody, JsArrayExpression, JsArrayExpressionFields, JsCallArguments,
    JsFormalParameter, JsFormalParameterFields, JsFunctionBodyFields, JsIdentifierBinding,
    JsIdentifierBindingFields, JsObjectExpression, JsObjectExpressionFields, JsParametersFields,
};
use rslint_parser::ast::{JsCallArgumentsFields, JsParameters};
use rslint_parser::{AstNode, AstSeparatedList, SyntaxNodeExt, SyntaxResult, SyntaxToken};

impl ToFormatElement for JsCallArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = self.as_fields();

        if is_simple_function_arguments(self)? {
            return Ok(hard_group_elements(format_elements![
                l_paren_token.format(formatter)?,
                args.format(formatter)?,
                r_paren_token.format(formatter)?,
            ]));
        }

        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            args.format(formatter)?,
            &r_paren_token?,
        )
    }
}

/// Returns true if the passed [JsCallArguments] has a single argument
/// that is a simple function expression, array expression or object expression
fn is_simple_function_arguments(node: &JsCallArguments) -> SyntaxResult<bool> {
    let JsCallArgumentsFields {
        l_paren_token,
        args,
        r_paren_token,
    } = node.as_fields();

    if token_has_comments(l_paren_token?) || token_has_comments(r_paren_token?) {
        return Ok(false);
    }

    if args.syntax_list().len() > 1 {
        return Ok(false);
    }

    for item in args {
        let node = item?;
        if let Some(func) = JsAnyFunction::cast(node.syntax().clone()) {
            if !is_simple_function_expression(func)? {
                return Ok(false);
            }
        } else if let Some(array) = JsArrayExpression::cast(node.syntax().clone()) {
            if !is_simple_array_expression(array)? {
                return Ok(false);
            }
        } else if let Some(object) = JsObjectExpression::cast(node.syntax().clone()) {
            if !is_simple_object_expression(object)? {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
    }

    Ok(true)
}

/// Returns true if the passed [JsAnyFunction] does not have any comment, type
/// parameters, return type annotation and simple parameters (see [is_simple_function_parameters])
fn is_simple_function_expression(func: JsAnyFunction) -> SyntaxResult<bool> {
    if let Some(token) = func.async_token() {
        if token_has_comments(token) {
            return Ok(false);
        }
    }

    if let Some(token) = func.function_token()? {
        if token_has_comments(token) {
            return Ok(false);
        }
    }

    if let Some(token) = func.star_token() {
        if token_has_comments(token) {
            return Ok(false);
        }
    }

    if let Some(id) = func.id()? {
        if id.syntax().contains_comments() {
            return Ok(false);
        }
    }

    if func.type_parameters().is_some() {
        return Ok(false);
    }

    match JsParameters::cast(func.parameters()?.syntax().clone()) {
        Some(params) => {
            if !is_simple_function_parameters(params)? {
                return Ok(false);
            }
        }
        None => return Ok(false),
    }

    if func.return_type_annotation().is_some() {
        return Ok(false);
    }

    match func.body()? {
        JsAnyFunctionBody::JsFunctionBody(body) => {
            let JsFunctionBodyFields {
                l_curly_token,
                directives: _,
                statements: _,
                r_curly_token,
            } = body.as_fields();

            // Only account for the leading comments on the opening token and the trailing
            // comments on the closing tokens (the inner tokens will be part of the body group)
            if l_curly_token?.has_leading_comments() || r_curly_token?.has_trailing_comments() {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }

    Ok(true)
}

fn is_simple_array_expression(node: JsArrayExpression) -> SyntaxResult<bool> {
    let JsArrayExpressionFields {
        l_brack_token,
        elements: _,
        r_brack_token,
    } = node.as_fields();

    if l_brack_token?.has_leading_comments() || r_brack_token?.has_trailing_comments() {
        return Ok(false);
    }

    Ok(true)
}

fn is_simple_object_expression(node: JsObjectExpression) -> SyntaxResult<bool> {
    let JsObjectExpressionFields {
        l_curly_token,
        members: _,
        r_curly_token,
    } = node.as_fields();

    if l_curly_token?.has_leading_comments() || r_curly_token?.has_trailing_comments() {
        return Ok(false);
    }

    Ok(true)
}

/// Returns true if the passed [JsParameters] has 2 or less parameters that are
/// all simple parameters (see [is_simple_parameter]) with no comments trivia
fn is_simple_function_parameters(node: JsParameters) -> SyntaxResult<bool> {
    let JsParametersFields {
        l_paren_token,
        items,
        r_paren_token,
    } = node.as_fields();

    if token_has_comments(l_paren_token?) || token_has_comments(r_paren_token?) {
        return Ok(false);
    }

    if items.syntax_list().len() >= 3 {
        return Ok(false);
    }

    for item in &items {
        match JsFormalParameter::cast(item?.syntax().clone()) {
            Some(node) => {
                if !is_simple_parameter(node)? {
                    return Ok(false);
                }
            }
            None => return Ok(false),
        }
    }

    Ok(true)
}

/// Returns true if the passed [JsFormalParameter] is a single identifier
/// with no question mark token, type annotation or initializer
fn is_simple_parameter(node: JsFormalParameter) -> SyntaxResult<bool> {
    let JsFormalParameterFields {
        binding,
        question_mark_token,
        type_annotation,
        initializer,
    } = node.as_fields();

    match JsIdentifierBinding::cast(binding?.syntax().clone()) {
        Some(ident) => {
            let JsIdentifierBindingFields { name_token } = ident.as_fields();
            if token_has_comments(name_token?) {
                return Ok(false);
            }
        }
        None => return Ok(false),
    }

    if question_mark_token.is_some() {
        return Ok(false);
    }

    if type_annotation.is_some() {
        return Ok(false);
    }

    if initializer.is_some() {
        return Ok(false);
    }

    Ok(true)
}

/// Returns true if the passed [SyntaxToken] has any comments
fn token_has_comments(token: SyntaxToken) -> bool {
    token.has_leading_comments() || token.has_trailing_comments()
}
