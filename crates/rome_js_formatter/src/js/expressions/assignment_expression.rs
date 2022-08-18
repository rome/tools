use crate::prelude::*;
use crate::utils::JsAnyAssignmentLike;

use crate::parentheses::{
    is_arrow_function_body, is_first_in_statement, ExpressionNode, FirstInStatementMode,
    NeedsParentheses,
};
use rome_formatter::write;

use rome_js_syntax::{
    JsAnyAssignmentPattern, JsAnyExpression, JsAnyForInitializer, JsAssignmentExpression,
    JsForStatement, JsSyntaxKind, JsSyntaxNode,
};
use rome_rowan::AstNode;

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
        match parent.kind() {
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => false,
            // `[a = b]`
            JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => false,

            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                is_arrow_function_body(self.syntax(), parent)
            }
            JsSyntaxKind::JS_FOR_STATEMENT => {
                let for_statement = JsForStatement::unwrap_cast(parent.clone());
                let is_initializer = match for_statement.initializer() {
                    Some(JsAnyForInitializer::JsAnyExpression(expression)) => {
                        &expression.resolve_syntax() == self.syntax()
                    }
                    None | Some(_) => false,
                };

                let is_update = for_statement
                    .update()
                    .map(ExpressionNode::into_resolved_syntax)
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
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let mut child = parent.clone();

                for ancestor in parent.ancestors().skip(1) {
                    match ancestor.kind() {
                        JsSyntaxKind::JS_SEQUENCE_EXPRESSION
                        | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => child = ancestor,
                        JsSyntaxKind::JS_FOR_STATEMENT => {
                            let for_statement = JsForStatement::unwrap_cast(ancestor);

                            let is_initializer = match for_statement.initializer() {
                                Some(JsAnyForInitializer::JsAnyExpression(expression)) => {
                                    expression.syntax() == &child
                                }
                                None | Some(_) => false,
                            };

                            let is_update =
                                for_statement.update().map(AstNode::into_syntax).as_ref()
                                    == Some(&child);

                            return !(is_initializer || is_update);
                        }
                        _ => break,
                    }
                }

                true
            }

            _ => true,
        }
    }
}

impl ExpressionNode for JsAssignmentExpression {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        self.clone().into()
    }

    #[inline]
    fn into_resolved(self) -> JsAnyExpression {
        self.into()
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
        assert_not_needs_parentheses!("for(a = 3, b = 2;;) {}", JsAssignmentExpression[1]);
        assert_not_needs_parentheses!("for(a = 3, b = 2, c= 3;;) {}", JsAssignmentExpression[2]);
        assert_needs_parentheses!("for(; a = 3; ) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for(;;a = 3) {}", JsAssignmentExpression);

        assert_not_needs_parentheses!("for ((a, a = 3);;) {}", JsAssignmentExpression);
        assert_needs_parentheses!("for (; (a, a = 3);) {}", JsAssignmentExpression);
        assert_not_needs_parentheses!("for (;;(a, a = 3)) {}", JsAssignmentExpression);

        assert_not_needs_parentheses!("a = 3", JsAssignmentExpression);
        assert_needs_parentheses!("({ a } = { a: 3 })", JsAssignmentExpression);
    }
}
