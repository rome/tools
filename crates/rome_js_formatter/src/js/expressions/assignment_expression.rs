use crate::prelude::*;
use crate::utils::{resolve_expression_syntax, JsAnyAssignmentLike};

use crate::parentheses::{is_first_in_statement, FirstInStatementMode, NeedsParentheses};
use rome_formatter::write;
use rome_js_syntax::{
    JsAnyAssignmentPattern, JsAnyForInitializer, JsAnyFunctionBody, JsArrowFunctionExpression,
    JsAssignmentExpression, JsForStatement, JsParenthesizedExpression, JsSyntaxKind, JsSyntaxNode,
};

#[derive(Debug, Clone, Default)]
pub struct FormatJsAssignmentExpression;

impl FormatNodeRule<JsAssignmentExpression> for FormatJsAssignmentExpression {
    fn fmt_fields(&self, node: &JsAssignmentExpression, f: &mut JsFormatter) -> FormatResult<()> {
        write![f, [JsAnyAssignmentLike::from(node.clone())]]
    }

    fn needs_parentheses(&self, item: &JsAssignmentExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsAssignmentExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        let grand_parent = parent
            .ancestors()
            .skip(1)
            .find(|parent| !JsParenthesizedExpression::can_cast(parent.kind()));

        match parent.kind() {
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => false,
            // `[a = b]`
            JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => false,

            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                let arrow = JsArrowFunctionExpression::unwrap_cast(parent.clone());

                match arrow.body() {
                    Ok(JsAnyFunctionBody::JsAnyExpression(expression)) => {
                        &resolve_expression_syntax(expression) == self.syntax()
                    }
                    _ => false,
                }
            }
            JsSyntaxKind::JS_FOR_STATEMENT => {
                let for_statement = JsForStatement::unwrap_cast(parent.clone());
                let is_initializer = match for_statement.initializer() {
                    Some(JsAnyForInitializer::JsAnyExpression(expression)) => {
                        &resolve_expression_syntax(expression) == self.syntax()
                    }
                    None | Some(_) => false,
                };

                let is_update = for_statement
                    .update()
                    .map(resolve_expression_syntax)
                    .as_ref()
                    == Some(self.syntax());

                !(is_initializer || is_update)
            }
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                // Parenthesize `{ a } = { a: 5 }`
                is_first_in_statement(
                    self.clone().into(),
                    FirstInStatementMode::ExpressionStatementOrArrow,
                ) && matches!(
                    self.left(),
                    Ok(JsAnyAssignmentPattern::JsObjectAssignmentPattern(_))
                )
            }
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => grand_parent
                .and_then(JsForStatement::cast)
                .map_or(true, |for_statement| {
                    let is_initializer = match for_statement.initializer() {
                        Some(JsAnyForInitializer::JsAnyExpression(expression)) => {
                            &resolve_expression_syntax(expression) == parent
                        }
                        None | Some(_) => false,
                    };
                    let is_update = for_statement
                        .update()
                        .map(resolve_expression_syntax)
                        .as_ref()
                        == Some(parent);

                    !(is_initializer || is_update)
                }),

            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsAssignmentExpression;

    #[test]
    fn needs_parentheses() {
        assert_not_needs_parentheses!("({ [a = 3]: value })", JsAssignmentExpression);
        assert_not_needs_parentheses!("class Test { [a = 3]: value }", JsAssignmentExpression);
        assert_not_needs_parentheses!("type Test  = { [a = 3]: value }", JsAssignmentExpression);
        assert_not_needs_parentheses!("interface Test { [a = 3]: value }", JsAssignmentExpression);

        assert_needs_parentheses!("a => (a = 3)", JsAssignmentExpression);
        assert_not_needs_parentheses!("a => { a = 3 }", JsAssignmentExpression);

        assert_not_needs_parentheses!("for(a = 3;;) {}", JsAssignmentExpression);
        assert_needs_parentheses!("for(; a = 3; ) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for(;;a = 3) {}", JsAssignmentExpression);

        assert_not_needs_parentheses!("for ((a, a = 3);;) {}", JsAssignmentExpression);
        assert_needs_parentheses!("for (; (a, a = 3);) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for (;;(a, a = 3)) {}", JsAssignmentExpression);

        assert_not_needs_parentheses!("a = 3", JsAssignmentExpression);
        assert_needs_parentheses!("({ a } = { a: 3 })", JsAssignmentExpression);
    }
}
