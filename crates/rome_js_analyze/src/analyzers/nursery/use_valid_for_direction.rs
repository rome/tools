use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAssignmentOperator,
    JsBinaryOperator, JsForStatement, JsIdentifierAssignment, JsIdentifierExpression,
    JsPostUpdateOperator, JsUnaryOperator,
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

            let is_less_than = matches!(
                operator,
                JsBinaryOperator::LessThan | JsBinaryOperator::LessThanOrEqual
            );
            let is_greater_than = matches!(
                operator,
                JsBinaryOperator::GreaterThan | JsBinaryOperator::GreaterThanOrEqual
            );

            if !is_less_than && !is_greater_than {
                return None;
            }

            match n.update() {
                Some(JsAnyExpression::JsPostUpdateExpression(update_expr)) => {
                    if let (
                        Ok(JsAnyExpression::JsIdentifierExpression(counter_ident)),
                        Ok(JsAnyAssignment::JsIdentifierAssignment(update_ident)),
                    ) = (binary_expr.left(), update_expr.operand())
                    {
                        if is_identifier_same(counter_ident, update_ident) != Some(true) {
                            return None;
                        }

                        if update_expr.operator() == Ok(JsPostUpdateOperator::Increment)
                            && is_greater_than
                        {
                            return Some(());
                        }

                        if update_expr.operator() == Ok(JsPostUpdateOperator::Decrement)
                            && is_less_than
                        {
                            return Some(());
                        }
                    }
                }
                Some(JsAnyExpression::JsAssignmentExpression(assignment_expr)) => {
                    if let (
                        Ok(JsAnyExpression::JsIdentifierExpression(counter_ident)),
                        Ok(JsAnyAssignmentPattern::JsAnyAssignment(
                            JsAnyAssignment::JsIdentifierAssignment(update_ident),
                        )),
                    ) = (binary_expr.left(), assignment_expr.left())
                    {
                        if is_identifier_same(counter_ident, update_ident) != Some(true) {
                            return None;
                        }

                        if let Ok(JsAnyExpression::JsIdentifierExpression(_)) =
                            assignment_expr.right()
                        {
                            return None;
                        }

                        if assignment_expr.operator() == Ok(JsAssignmentOperator::AddAssign) {
                            if is_greater_than {
                                return Some(());
                            }

                            if let Ok(JsAnyExpression::JsUnaryExpression(unary_expr)) =
                                assignment_expr.right()
                            {
                                if unary_expr.operator() == Ok(JsUnaryOperator::Minus)
                                    && is_less_than
                                {
                                    return Some(());
                                }
                            }
                        }

                        if assignment_expr.operator() == Ok(JsAssignmentOperator::SubtractAssign) {
                            if is_less_than {
                                return Some(());
                            }

                            if let Ok(JsAnyExpression::JsUnaryExpression(unary_expr)) =
                                assignment_expr.right()
                            {
                                if unary_expr.operator() == Ok(JsUnaryOperator::Minus)
                                    && is_greater_than
                                {
                                    return Some(());
                                }
                            }
                        }
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

fn is_identifier_same(
    counter_ident: JsIdentifierExpression,
    update_ident: JsIdentifierAssignment,
) -> Option<bool> {
    Some(
        counter_ident
            .name()
            .ok()?
            .value_token()
            .ok()?
            .text_trimmed()
            == update_ident.name_token().ok()?.text_trimmed(),
    )
}
