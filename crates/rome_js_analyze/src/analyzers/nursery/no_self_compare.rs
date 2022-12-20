use crate::utils::is_node_equal;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_js_syntax::{JsBinaryExpression, JsBinaryOperator};
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow comparisons where both sides are exactly the same.
    ///
    /// > Comparing a variable against itself is usually an error, either a typo or refactoring error. It is confusing to the reader and may potentially introduce a runtime error.
    ///
    /// > The only time you would compare a variable against itself is when you are testing for `NaN`. However, it is far more appropriate to use `typeof x === 'number' && isNaN(x)` or the [Number.isNaN ES2015 function](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/isNaN) for that use case rather than leaving the reader of the code to determine the intent of self comparison.
    ///
    /// Source: [no-self-compare](https://eslint.org/docs/latest/rules/no-self-compare).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (x === x) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (a.b.c() !== a.b .c()) {}
    /// ```
    ///
    pub(crate) NoSelfCompare {
        version: "12.0.0",
        name: "noSelfCompare",
        recommended: true,
    }
}

impl Rule for NoSelfCompare {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !matches!(
            &node.operator(),
            Ok(JsBinaryOperator::Equality
                | JsBinaryOperator::Inequality
                | JsBinaryOperator::GreaterThan
                | JsBinaryOperator::GreaterThanOrEqual
                | JsBinaryOperator::LessThan
                | JsBinaryOperator::LessThanOrEqual
                | JsBinaryOperator::StrictEquality
                | JsBinaryOperator::StrictInequality)
        ) {
            return None;
        }

        let left = node.left().ok()?;
        let right = node.right().ok()?;

        if is_node_equal(left.syntax(), right.syntax()) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "Comparing to itself is potentially pointless.",
        ))
    }
}
