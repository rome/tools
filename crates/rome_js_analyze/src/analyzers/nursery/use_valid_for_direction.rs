use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAssignmentOperator,
    JsBinaryOperator, JsForStatement, JsPostUpdateOperator, JsUnaryOperator,
};

declare_rule! {
    /// Enforce "for" loop update clause moving the counter in the right direction.
    ///
    /// A for loop with a stop condition that can never be reached,
    /// such as one with a counter that moves in the wrong direction, will run infinitely.
    /// While there are occasions when an infinite loop is intended, the convention is to construct such loops as while loops.
    /// More typically, an infinite for loop is a bug.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (var i = 0; i < 10; i--) {
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (var i = 10; i >= 0; i++) {
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (var i = 0; i > 10; i++) {
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (var i = 0; i < 10; i++) {
    /// }
    /// ```
    pub(crate) UseValidForDirection {
        version: "0.10.0",
        name: "useValidForDirection",
        recommended: false,
    }
}

impl Rule for UseValidForDirection {
    type Query = Ast<JsForStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let n = ctx.query();

        if let Some(JsAnyExpression::JsBinaryExpression(binary_expr)) = n.test() {
            let operator = binary_expr.operator().ok()?;
            let wrong_direction;

            if matches!(
                operator,
                JsBinaryOperator::LessThan | JsBinaryOperator::LessThanOrEqual
            ) {
                wrong_direction = -1;
            } else if matches!(
                operator,
                JsBinaryOperator::GreaterThan | JsBinaryOperator::GreaterThanOrEqual
            ) {
                wrong_direction = 1
            } else {
                return None;
            }

            match n.update() {
                Some(JsAnyExpression::JsPostUpdateExpression(update_expr)) => {
                    match (binary_expr.left().ok(), update_expr.operand().ok()) {
                        (
                            Some(JsAnyExpression::JsIdentifierExpression(counter_ident)),
                            Some(JsAnyAssignment::JsIdentifierAssignment(update_ident)),
                        ) => {
                            if counter_ident
                                .name()
                                .ok()?
                                .value_token()
                                .ok()?
                                .text_trimmed()
                                != update_ident.name_token().ok()?.text_trimmed()
                            {
                                return None;
                            }

                            match update_expr.operator().ok() {
                                Some(JsPostUpdateOperator::Increment) => {
                                    if wrong_direction == 1 {
                                        return Some(());
                                    }
                                }
                                Some(JsPostUpdateOperator::Decrement) => {
                                    if wrong_direction == -1 {
                                        return Some(());
                                    }
                                }
                                _ => return None,
                            }
                        }
                        _ => return None,
                    }
                }
                Some(JsAnyExpression::JsAssignmentExpression(assignment_expr)) => {
                    match (binary_expr.left().ok(), assignment_expr.left().ok()) {
                        (
                            Some(JsAnyExpression::JsIdentifierExpression(counter_ident)),
                            Some(JsAnyAssignmentPattern::JsAnyAssignment(
                                JsAnyAssignment::JsIdentifierAssignment(update_ident),
                            )),
                        ) => {
                            if counter_ident
                                .name()
                                .ok()?
                                .value_token()
                                .ok()?
                                .text_trimmed()
                                != update_ident.name_token().ok()?.text_trimmed()
                            {
                                return None;
                            }

                            match assignment_expr.operator().ok() {
                                Some(JsAssignmentOperator::AddAssign) => {
                                    match assignment_expr.right().ok() {
                                        Some(JsAnyExpression::JsUnaryExpression(unary_expr)) => {
                                            if unary_expr.operator().ok()
                                                == Some(JsUnaryOperator::Minus)
                                            {
                                                if wrong_direction == -1 {
                                                    return Some(());
                                                }
                                            } else {
                                                if wrong_direction == -1 {
                                                    return Some(());
                                                }
                                            }
                                        }
                                        Some(JsAnyExpression::JsIdentifierExpression(_)) => {
                                            return None
                                        }
                                        _ => {
                                            if wrong_direction == 1 {
                                                return Some(());
                                            }
                                        }
                                    }
                                }
                                Some(JsAssignmentOperator::SubtractAssign) => {
                                    match assignment_expr.right().ok() {
                                        Some(JsAnyExpression::JsUnaryExpression(unary_expr)) => {
                                            if unary_expr.operator().ok()
                                                == Some(JsUnaryOperator::Minus)
                                            {
                                                if wrong_direction == 1 {
                                                    return Some(());
                                                }
                                            } else {
                                                if wrong_direction == -1 {
                                                    return Some(());
                                                }
                                            }
                                        }
                                        Some(JsAnyExpression::JsIdentifierExpression(_)) => {
                                            return None
                                        }
                                        _ => {
                                            if wrong_direction == -1 {
                                                return Some(());
                                            }
                                        }
                                    }
                                }
                                _ => return None,
                            }
                        }
                        _ => return None,
                    }
                }
                _ => return None,
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        // SAFETY: These tokens have been checked for errors in `run` already
        let for_range = node.for_token().unwrap().text_trimmed_range();
        let r_paren_range = node.r_paren_token().unwrap().text_trimmed_range();

        Some(RuleDiagnostic::new(
            rule_category!(),
            for_range.cover(r_paren_range),
            markup! {
                "The update clause in this loop moves the variable in the wrong direction."
            },
        ))
    }
}
