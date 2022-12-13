use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_js_syntax::{
    JsBinaryExpression, JsBinaryOperator, JsSyntaxNode, JsSyntaxToken, WalkEvent,
};
use rome_rowan::{AstNode, Direction};
use std::iter;

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

        if is_node_equal(&left.into_syntax(), &right.into_syntax()) {
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

/// Verifies that both nodes are equal by checking their descendants (nodes included) kinds
/// and tokens (same kind and inner token text).
fn is_node_equal(a_node: &JsSyntaxNode, b_node: &JsSyntaxNode) -> bool {
    let a_tree = a_node.preorder_with_tokens(Direction::Next);
    let b_tree = b_node.preorder_with_tokens(Direction::Next);

    for (a_child, b_child) in iter::zip(a_tree, b_tree) {
        let a_event = match a_child {
            WalkEvent::Enter(event) => event,
            WalkEvent::Leave(event) => event,
        };

        let b_event = match b_child {
            WalkEvent::Enter(event) => event,
            WalkEvent::Leave(event) => event,
        };

        if a_event.kind() != b_event.kind() {
            return false;
        }

        let a_token = a_event.as_token();
        let b_token = b_event.as_token();

        // both are nodes
        if a_token.is_none() && b_token.is_none() {
            continue;
        }

        // one of them is a node
        if a_token.is_none() && b_token.is_some() || a_token.is_some() && b_token.is_none() {
            return false;
        }

        // both are tokens
        if let (Some(a_token), Some(b_token)) = (a_token, b_token) {
            if !is_token_text_equal(a_token, b_token) {
                return false;
            }
        }
    }

    true
}

/// Verify that tokens' inner text are equal
fn is_token_text_equal(a: &JsSyntaxToken, b: &JsSyntaxToken) -> bool {
    static QUOTES: [char; 2] = ['"', '\''];

    a.token_text_trimmed()
        .trim_start_matches(QUOTES)
        .trim_end_matches(QUOTES)
        == b.token_text_trimmed()
            .trim_start_matches(QUOTES)
            .trim_end_matches(QUOTES)
}
