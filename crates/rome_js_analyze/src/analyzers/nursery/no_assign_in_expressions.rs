use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAssignmentExpression, JsAssignmentOperator, JsExpressionStatement, JsForStatement,
    JsParenthesizedExpression, JsSequenceExpression, JsSyntaxKind,
};
use rome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow assignments in expressions.
    ///
    /// In expressions, it is common to mistype a comparison operator (such as `==`) as an assignment operator (such as `=`).
    /// Moreover, the use of assignments in expressions is confusing.
    /// Indeed, expressions are often considered as side-effect free.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let a, b;
    /// a = (b = 1) + 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let a;
    /// if (a = 1) {
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function f(a) {
    ///     return a = 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let a;
    /// a = 1;
    /// ```
    pub(crate) NoAssignInExpressions {
        version: "12.0.0",
        name: "noAssignInExpressions",
        recommended: true,
    }
}

impl Rule for NoAssignInExpressions {
    type Query = Ast<JsAssignmentExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assign = ctx.query();
        let mut ancestor = assign
            .syntax()
            .ancestors()
            .take_while(|x| {
                // Allow parens and multiple assign such as `a = b = (c = d)`
                JsAssignmentExpression::can_cast(x.kind())
                    || JsParenthesizedExpression::can_cast(x.kind())
            })
            .last()?;
        let mut prev_ancestor = ancestor;
        ancestor = prev_ancestor.parent()?;
        while JsSequenceExpression::can_cast(ancestor.kind()) {
            // Allow statements separated by sequences such as `a = 1, b = 2`
            prev_ancestor = ancestor;
            ancestor = prev_ancestor.parent()?;
        }
        if JsExpressionStatement::can_cast(ancestor.kind()) {
            None
        } else if let Some(for_stmt) = JsForStatement::cast(ancestor) {
            if let Some(for_test) = for_stmt.test() {
                // Disallow assignment in test part of a `for`
                (for_test.syntax() == &prev_ancestor).then_some(())
            } else {
                // Allow assignment in initializer and update parts of a `for`
                None
            }
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let assign = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            assign.range(),
            markup! {
                "The "<Emphasis>"assignment"</Emphasis>" should not be in an "<Emphasis>"expression"</Emphasis>"."
            },
        ).note(
            "The use of assignments in expressions is confusing.\nExpressions are often considered as side-effect free."
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let assign = ctx.query();
        let op = assign.operator().ok()?;
        if let JsAssignmentOperator::Assign = op {
            let operator_token = assign.operator_token().ok()?;
            let new_operator_token = make::token(JsSyntaxKind::EQ3);
            let mut mutation = ctx.root().begin();
            mutation.replace_token(operator_token, new_operator_token);
            Some(JsRuleAction {
                mutation,
                applicability: Applicability::MaybeIncorrect,
                category: ActionCategory::QuickFix,
                message: markup!("Did you mean '==='?").to_owned(),
            })
        } else {
            None
        }
    }
}
