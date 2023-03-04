use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsCaseClause, JsDefaultClause};
use rome_rowan::{AstNode, Direction};

declare_rule! {
    /// Enforce default clauses in switch statements to be last
    ///
    /// A switch statement can optionally have a default clause.
    ///
    /// If present, it’s usually the last clause, but it doesn’t need to be. It is also allowed to put the default clause before all case clauses, or anywhere between.
    /// The behavior is mostly the same as if it was the last clause.
    ///
    /// The default block will be still executed only if there is no match in the case clauses (including those defined after the default),
    /// but there is also the ability to “fall through” from the default clause to the following clause in the list.
    /// However, such flow is not common and it would be confusing to the readers.
    ///
    /// Even if there is no "fall through" logic, it’s still unexpected to see the default clause before or between the case clauses. By convention, it is expected to be the last clause.
    ///
    /// Source: https://eslint.org/docs/latest/rules/default-case-last
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     default:
    ///         break;
    ///     case 0:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     default:
    ///         f();
    ///     case 0:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case 0:
    ///         break;
    ///     default:
    ///     case 1:
    ///         break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (foo) {
    ///     case 0:
    ///         break;
    ///     case 1:
    ///     default:
    ///         break;
    /// }
    /// ```
    ///
    /// ```js
    /// switch (foo) {
    ///     case 0:
    ///         break;
    /// }
    /// ```
    ///
    pub(crate) UseDefaultSwitchClauseLast {
        version: "11.0.0",
        name: "useDefaultSwitchClauseLast",
        recommended: true,
    }
}

impl Rule for UseDefaultSwitchClauseLast {
    type Query = Ast<JsDefaultClause>;
    type State = JsCaseClause;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let default_clause = ctx.query();
        let next_case = default_clause
            .syntax()
            .siblings(Direction::Next)
            .find_map(JsCaseClause::cast);
        next_case
    }

    fn diagnostic(ctx: &RuleContext<Self>, next_case: &Self::State) -> Option<RuleDiagnostic> {
        let default_clause = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            default_clause.range(),
            markup! {
                "The "<Emphasis>"default"</Emphasis>" clause should be the last "<Emphasis>"switch"</Emphasis>" clause."
            },
        ).detail(
            next_case.range(),
            markup! {
                "The following "<Emphasis>"case"</Emphasis>" clause is here:"
            },
        ).note(
            markup! {
                "Regardless its position, the "<Emphasis>"default"</Emphasis>" clause is always executed when there is no match. To avoid confusion, the "<Emphasis>"default"</Emphasis>" clause should be the last "<Emphasis>"switch"</Emphasis>" clause."
            }
        ))
    }
}
