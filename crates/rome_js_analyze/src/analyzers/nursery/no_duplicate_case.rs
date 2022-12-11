use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_js_syntax::{AnyJsSwitchClause, JsCaseClause, JsSwitchStatement};
use rome_rowan::AstNode;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

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
    /// ```ts,expect_diagnostic
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
    /// ```ts,expect_diagnostic
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
    /// ```ts,expect_diagnostic
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
    /// ```ts
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
    /// ```ts
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
    /// ```ts
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
        version: "11.0.0",
        name: "noDuplicateCase",
        recommended: true,
    }
}

impl Rule for NoDuplicateCase {
    type Query = Ast<JsSwitchStatement>;
    type State = JsCaseClause;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let mut defined_cases: HashMap<String, JsCaseClause> = HashMap::new();
        let mut signals = Vec::new();

        for case in node.cases() {
            if let AnyJsSwitchClause::JsCaseClause(case) = case {
                if let Ok(test) = case.test() {
                    let text = test.text();

                    match defined_cases.entry(text) {
                        Entry::Occupied(_) => {
                            signals.push(case);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(case);
                        }
                    }
                }
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, case: &Self::State) -> Option<RuleDiagnostic> {
        case.test().ok().map(|test| {
            RuleDiagnostic::new(rule_category!(), test.range(), "Duplicate case label.")
        })
    }
}
