use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyExpression, TsNonNullAssertionAssignment, TsNonNullAssertionExpression,
};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Prevents the wrong usage of the non-null assertion operator (`!`) in TypeScript files.
    ///
    /// > The `!` non-null assertion operator in TypeScript is used to assert that a value's type does not include `null` or `undefined`. Using the operator any more than once on a single value does nothing.
    ///
    /// Source: https://typescript-eslint.io/rules/no-extra-non-null-assertion
    ///
    /// ## Examples
    ///
    /// ### Invalid
    /// ```ts,expect_diagnostic
    /// const bar = foo!!.bar;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function fn(bar?: { n: number }) {
    ///   return bar!?.n;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function fn(bar?: { n: number }) {
    ///   return ((bar!))?.();
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```ts
    /// const bar = foo!.bar;
    ///
    /// obj?.string!.trim();
    ///
    /// function fn(key: string | null) {
    ///   const obj = {};
    ///   return obj?.[key!];
    /// }
    /// ```
    ///
    pub(crate) NoExtraNonNullAssertion {
        version: "11.0.0",
        name: "noExtraNonNullAssertion",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) TsAnyNonNullAssertion = TsNonNullAssertionAssignment | TsNonNullAssertionExpression
}

impl Rule for NoExtraNonNullAssertion {
    type Query = Ast<TsAnyNonNullAssertion>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            TsAnyNonNullAssertion::TsNonNullAssertionAssignment(_) => {
                let parent = node.parent::<JsAnyAssignment>()?;

                // Cases considered as invalid:
                // - TsNonNullAssertionAssignment > TsNonNullAssertionAssignment
                let has_extra_non_assertion = match parent {
                    JsAnyAssignment::TsNonNullAssertionAssignment(_) => true,
                    _ => false,
                };

                if has_extra_non_assertion {
                    return Some(());
                }
            }
            TsAnyNonNullAssertion::TsNonNullAssertionExpression(_) => {
                let parent = node.parent::<JsAnyExpression>()?;

                // Cases considered as invalid:
                // - TsNonNullAssertionAssignment > TsNonNullAssertionExpression
                // - TsNonNullAssertionExpression > TsNonNullAssertionExpression
                // - JsCallExpression[optional] > TsNonNullAssertionExpression
                // - JsStaticMemberExpression[optional] > TsNonNullAssertionExpression
                let has_extra_non_assertion = match parent.omit_parentheses() {
                    JsAnyExpression::JsAssignmentExpression(expr) => expr
                        .left()
                        .ok()?
                        .as_js_any_assignment()?
                        .as_ts_non_null_assertion_assignment()
                        .is_some(),
                    JsAnyExpression::TsNonNullAssertionExpression(_) => true,
                    JsAnyExpression::JsStaticMemberExpression(expr) => expr.is_optional(),
                    JsAnyExpression::JsCallExpression(expr) => expr.is_optional(),
                    _ => false,
                };

                if has_extra_non_assertion {
                    return Some(());
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "Forbidden extra non-null assertion.",
        );

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();

        let excl_token = match node {
            TsAnyNonNullAssertion::TsNonNullAssertionAssignment(assignment) => {
                assignment.excl_token().ok()?
            }
            TsAnyNonNullAssertion::TsNonNullAssertionExpression(expression) => {
                expression.excl_token().ok()?
            }
        };

        mutation.remove_token(excl_token);

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Remove extra non-null assertion." }.to_owned(),
            mutation,
        })
    }
}
