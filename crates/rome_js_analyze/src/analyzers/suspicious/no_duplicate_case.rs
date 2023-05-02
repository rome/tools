use crate::utils::is_node_equal;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_js_syntax::{AnyJsExpression, AnyJsSwitchClause, JsCaseClause, JsSwitchStatement};
use rome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Disallow duplicate case labels.
    /// If a switch statement has duplicate test expressions in case clauses, it is likely that a programmer copied a case clause but forgot to change the test expression.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-duplicate-case
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case 1:
    ///         break;
    ///     case 1:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case one:
    ///         break;
    ///     case one:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (a) {
    ///     case "1":
    ///         break;
    ///     case "1":
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (a) {
    ///     case 1:
    ///         break;
    ///     case 2:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js
    /// switch (a) {
    ///     case one:
    ///         break;
    ///     case two:
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js
    /// switch (a) {
    ///     case "1":
    ///         break;
    ///     case "2":
    ///         break;
    ///     default:
    ///         break;
    /// }
    /// ```
    pub(crate) NoDuplicateCase {
        version: "12.0.0",
        name: "noDuplicateCase",
        recommended: true,
    }
}

impl Rule for NoDuplicateCase {
    type Query = Ast<JsSwitchStatement>;
    type State = (TextRange, JsCaseClause);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_tests: Vec<AnyJsExpression> = Vec::new();
        let mut signals = Vec::new();

        for case in node.cases() {
            if let AnyJsSwitchClause::JsCaseClause(case) = case {
                if let Ok(test) = case.test() {
                    let define_test = defined_tests
                        .iter()
                        .find(|define_test| is_node_equal(define_test.syntax(), test.syntax()));

                    match define_test {
                        Some(define_test) => {
                            signals.push((define_test.range(), case));
                        }
                        None => {
                            defined_tests.push(test);
                        }
                    }
                }
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (first_label_range, case) = state;
        case.test().ok().map(|test| {
            RuleDiagnostic::new(rule_category!(), test.range(), "Duplicate case label.")
                .detail(first_label_range, "The first similar label is here:")
        })
    }
}
