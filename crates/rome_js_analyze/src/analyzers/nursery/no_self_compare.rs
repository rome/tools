use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_js_syntax::{JsBinaryExpression, JsBinaryOperator, JsLanguage, TextSize};
use rome_rowan::{AstNode, SyntaxNode, SyntaxToken, SyntaxTokenText};
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

static COMPARISON_OPERATORS: [JsBinaryOperator; 8] = [
    JsBinaryOperator::Equality,
    JsBinaryOperator::Inequality,
    JsBinaryOperator::GreaterThan,
    JsBinaryOperator::GreaterThanOrEqual,
    JsBinaryOperator::LessThan,
    JsBinaryOperator::LessThanOrEqual,
    JsBinaryOperator::StrictEquality,
    JsBinaryOperator::StrictInequality,
];

impl Rule for NoSelfCompare {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !COMPARISON_OPERATORS.contains(&node.operator().ok()?) {
            return None;
        }

        let left = node.left().ok()?;
        let right = node.right().ok()?;

        if is_node_equal(&left.into_syntax(), &right.into_syntax())? {
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

/// Verifies recursively that both nodes are equal by checking their kinds and their children nodes
/// and tokens (same kind and inner token text).
fn is_node_equal(a_node: &SyntaxNode<JsLanguage>, b_node: &SyntaxNode<JsLanguage>) -> Option<bool> {
    if a_node.kind() != b_node.kind() {
        return Some(false);
    }

    let mut is_equal = true;
    let a_children = a_node.children_with_tokens();
    let b_children = b_node.children_with_tokens();

    for (a_child, b_child) in iter::zip(a_children, b_children) {
        if let (Some(a_token), Some(b_token)) = (a_child.as_token(), b_child.as_token()) {
            if !is_token_equal(a_token, b_token) {
                return Some(false);
            } else {
                continue;
            }
        }

        is_equal &= is_node_equal(a_child.as_node()?, b_child.as_node()?)?
    }

    Some(is_equal)
}

/// Verify that tokens are equal, by checking if they have the same kind and inner text.
fn is_token_equal(a: &SyntaxToken<JsLanguage>, b: &SyntaxToken<JsLanguage>) -> bool {
    a.kind() == b.kind() && inner_string_text(a).text() == inner_string_text(b).text()
}

/// Get the inner text of a string not including the quotes
fn inner_string_text(token: &SyntaxToken<JsLanguage>) -> SyntaxTokenText {
    let mut text = token.token_text_trimmed();

    static QUOTES: [char; 2] = ['"', '\''];

    if text.starts_with(QUOTES) {
        let range = text.range().add_start(TextSize::from(1));
        text = text.slice(range);
    }

    if text.ends_with(QUOTES) {
        let range = text.range().sub_end(TextSize::from(1));
        text = text.slice(range);
    }

    text
}
