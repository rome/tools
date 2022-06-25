//! This module exposes utility functions for detecting "simple" expressions
//!
//! Simple expressions are expressions that are going to create a single group
//! anyway, so they don't need to be wrapped in a second one: this includes
//! object, array or parenthesized expressions, as well as function
//! declarations that have a high probability of breaking only in their body
//! group.
//! This last bit is defined recursively in [is_simple_function_expression] as
//! functions that only have a few (less than 3) identifier parameters, no type
//! parameter or return type and a block body: technically such a function
//! expression can still break in both the parameters and body group but the
//! small number of parameters makes it unlikely.
//!
//! The use case for detecting these "simple" expressions is to avoid creating
//! redundant groups in nested delimited expressions when only one would
//! suffice, for instance in call expressions:
//!
//! ```js
//! // Formatter output without handling of simple expressions
//! new Promise(
//!   (resolve, reject) => {
//!     resolve();
//!   },
//! );
//!
//! func(
//!   {
//!     key: 'value',
//!   },
//! );
//!
//! // Formatter output with handling of simple expressions
//! new Promise((resolve, reject) => {
//!   resolve();
//! });
//!
//! func({
//!   key: 'value',
//! });
//! ```

use rome_js_syntax::{
    JsAnyArrowFunctionParameters, JsAnyBinding, JsAnyBindingPattern, JsAnyExpression,
    JsAnyFormalParameter, JsAnyFunction, JsAnyFunctionBody, JsAnyParameter, JsArrayExpression,
    JsArrayExpressionFields, JsFormalParameter, JsFormalParameterFields, JsFunctionBodyFields,
    JsIdentifierBindingFields, JsObjectExpression, JsObjectExpressionFields, JsParameters,
    JsParametersFields, JsSyntaxToken,
};
use rome_rowan::{AstNode, AstSeparatedList, SyntaxResult};

/// Returns true is the passed [JsAnyExpression] is a simple function, array or object expression
pub(crate) fn is_simple_expression(node: &JsAnyExpression) -> SyntaxResult<bool> {
    match node {
        JsAnyExpression::JsArrayExpression(array) => is_simple_array_expression(array),
        JsAnyExpression::JsObjectExpression(object) => is_simple_object_expression(object),
        node => {
            if let Some(func) = JsAnyFunction::cast(node.syntax().clone()) {
                is_simple_function_expression(func)
            } else {
                Ok(false)
            }
        }
    }
}

/// Returns true if the passed [JsAnyFunction] does not have any comment, type
/// parameters, return type annotation and simple parameters (see [is_simple_function_parameters])
pub(crate) fn is_simple_function_expression(func: JsAnyFunction) -> SyntaxResult<bool> {
    if let Some(token) = func.async_token() {
        if token_has_comments(&token) {
            return Ok(false);
        }
    }

    if let Some(token) = func.function_token()? {
        if token_has_comments(&token) {
            return Ok(false);
        }
    }

    if let Some(token) = func.star_token() {
        if token_has_comments(&token) {
            return Ok(false);
        }
    }

    if let Some(id) = func.id()? {
        if id.syntax().has_comments_direct() {
            return Ok(false);
        }
    }

    if func.type_parameters().is_some() {
        return Ok(false);
    }

    match func.parameters()? {
        JsAnyArrowFunctionParameters::JsAnyBinding(JsAnyBinding::JsIdentifierBinding(
            identifier,
        )) => {
            if token_has_comments(&identifier.name_token()?) {
                return Ok(false);
            }
        }
        JsAnyArrowFunctionParameters::JsParameters(parameters) => {
            if !is_simple_function_parameters(parameters)? {
                return Ok(false);
            }
        }
        _ => {
            return Ok(false);
        }
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

fn is_simple_array_expression(node: &JsArrayExpression) -> SyntaxResult<bool> {
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

fn is_simple_object_expression(node: &JsObjectExpression) -> SyntaxResult<bool> {
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

    if token_has_comments(&l_paren_token?) || token_has_comments(&r_paren_token?) {
        return Ok(false);
    }

    if items.len() >= 3 {
        return Ok(false);
    }

    for item in &items {
        match item? {
            JsAnyParameter::JsAnyFormalParameter(JsAnyFormalParameter::JsFormalParameter(node)) => {
                if !is_simple_parameter(node)? {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
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

    match binding? {
        JsAnyBindingPattern::JsAnyBinding(JsAnyBinding::JsIdentifierBinding(ident)) => {
            let JsIdentifierBindingFields { name_token } = ident.as_fields();
            if token_has_comments(&name_token?) {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }

    if question_mark_token.is_some() || type_annotation.is_some() || initializer.is_some() {
        return Ok(false);
    }

    Ok(true)
}

/// Returns true if the passed [SyntaxToken] has any comments
pub(crate) fn token_has_comments(token: &JsSyntaxToken) -> bool {
    token.has_leading_comments() || token.has_trailing_comments()
}
