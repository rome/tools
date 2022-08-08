use crate::utils::{JsAnyBinaryLikeExpression, JsAnyBinaryLikeLeftExpression};
use rome_js_syntax::JsAnyExpression;
use rome_rowan::{AstNode, SyntaxResult};

/// Returns `true` if the `expression`s first token isn't a lookahead token (a `{`).
///
/// The [`ExpressionStatement`](https://tc39.es/ecma262/#prod-ExpressionStatement) parsing rule (and others)
/// prohibit exclude tokens like a `{` from the lookahead because of ambiguity. This function allows to
/// test if the expression starts with such a lookahead token.
///
/// Note: The current implementation only supports testing for `{` yet. Other lookahead tokens like
/// `class, `function` and `let [` are not yet supported.
pub(crate) fn starts_with_no_lookahead_token(expression: JsAnyExpression) -> SyntaxResult<bool> {
    use JsAnyExpression::*;

    let mut node = resolve_left_most_expression(expression)?;

    let result = loop {
        node = match node {
            JsObjectExpression(_) => break true,
            JsParenthesizedExpression(parenthesized) => parenthesized.expression()?,
            JsStaticMemberExpression(member) => member.object()?,
            JsComputedMemberExpression(member) => member.object()?,
            JsTemplate(template) => {
                if let Some(tag) = template.tag() {
                    tag
                } else {
                    break false;
                }
            }
            JsCallExpression(call) => call.callee()?,
            JsConditionalExpression(conditional) => conditional.test()?,
            JsPostUpdateExpression(_) => {
                break false;
            }
            TsAsExpression(as_expression) => as_expression.expression()?,
            TsNonNullAssertionExpression(non_null) => non_null.expression()?,
            _ => {
                break false;
            }
        }
    };

    Ok(result)
}

/// Resolves the (recursively) left hand side if `expression` is a binary like node or the expression itself.
fn resolve_left_most_expression(expression: JsAnyExpression) -> SyntaxResult<JsAnyExpression> {
    let mut current = expression;

    let result = loop {
        current = match JsAnyBinaryLikeExpression::try_cast_node(current) {
            Ok(binary_like) => match binary_like.left()? {
                JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => expression,
                JsAnyBinaryLikeLeftExpression::JsPrivateName(_) => {
                    break JsAnyExpression::from(binary_like);
                }
            },
            Err(expression) => {
                break expression;
            }
        };
    };

    Ok(result)
}
