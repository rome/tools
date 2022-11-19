use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsAnyExpression, JsParenthesizedExpression, TsNonNullAssertionExpression};
use rome_rowan::{AstNode, BatchMutationExt};
use std::iter;

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

impl Rule for NoExtraNonNullAssertion {
    type Query = Ast<TsNonNullAssertionExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let parent = node.syntax().parent()?;

        if has_extra_non_null_assertion(JsAnyExpression::cast(parent)?) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {"Forbidden extra non-null assertion."}.to_owned(),
        );

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();

        mutation.remove_token(node.excl_token().ok()?);

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Remove extra non-null assertion." }.to_owned(),
            mutation,
        })
    }
}

/// Verify if a given expression has an extra non-null assertion.
///
/// Cases considered as invalid:
/// - TsNonNullAssertionExpression > TsNonNullAssertionExpression
/// - JsCallExpression[optional] > TsNonNullAssertionExpression
/// - JsStaticMemberExpression[optional] > TsNonNullAssertionExpression
/// - Any of above wrapped by JsParenthesizedExpression
fn has_extra_non_null_assertion(expression: JsAnyExpression) -> bool {
    match expression {
        JsAnyExpression::TsNonNullAssertionExpression(_) => return true,
        JsAnyExpression::JsStaticMemberExpression(static_member_exp) => {
            if static_member_exp.is_optional() {
                return true;
            }
        }
        JsAnyExpression::JsCallExpression(call_exp) => {
            if call_exp.is_optional() {
                return true;
            }
        }
        JsAnyExpression::JsParenthesizedExpression(parenthesized_exp) => {
            if let Some(parent) = get_chain_parent(parenthesized_exp) {
                return has_extra_non_null_assertion(parent);
            }
        }
        _ => {}
    }

    false
}

/// Traversal by parent to find the parent of the parenthesis chain.
fn get_chain_parent(expression: JsParenthesizedExpression) -> Option<JsAnyExpression> {
    iter::successors(expression.parent::<JsAnyExpression>(), |expression| {
        if matches!(expression, JsAnyExpression::JsParenthesizedExpression(_)) {
            expression.parent::<JsAnyExpression>()
        } else {
            None
        }
    })
    .last()
}
