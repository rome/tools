use rome_js_syntax::{JsAnyExpression, JsAnyInProperty};
use rome_rowan::SyntaxResult;

/// Returns `true` if the `node`s first token isn't a lookahead token (a `{`).
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

/// Resolves the left hand side of a binary like expression.
///
/// Returns the `left` most node or the passed in expression
fn resolve_left_most_expression(expression: JsAnyExpression) -> SyntaxResult<JsAnyExpression> {
    use JsAnyExpression::*;
    let mut current = expression;

    loop {
        current = match current {
            JsLogicalExpression(logical) => logical.left()?,
            JsBinaryExpression(binary) => binary.left()?,
            JsInExpression(in_expression) => match in_expression.property()? {
                JsAnyInProperty::JsAnyExpression(expression) => expression,
                JsAnyInProperty::JsPrivateName(_) => {
                    current = JsInExpression(in_expression);
                    break;
                }
            },
            JsInstanceofExpression(instance_of) => instance_of.left()?,
            _ => {
                break;
            }
        }
    }

    Ok(current)
}
