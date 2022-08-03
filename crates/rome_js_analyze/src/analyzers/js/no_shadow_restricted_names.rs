use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::JsIdentifierBinding;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow identifiers from shadowing restricted names.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function NaN() {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let Set;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {	} catch(Object) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function Array() {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function test(JSON) {console.log(JSON)}
    /// ```
    pub(crate) NoShadowRestrictedNames {
        version: "0.8.0",
        name: "noShadowRestrictedNames",
        recommended: true,
    }
}

const RESTRICTEDNAMES: [&str; 9] = [
    "NaN",
    "undefined",
    "Infinity",
    "arguments",
    "eval",
    "Set",
    "Object",
    "Array",
    "JSON",
];

pub struct State {
    shadowed_name: String,
}

impl Rule for NoShadowRestrictedNames {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsIdentifierBinding>;
    type State = State;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();
        let name = binding.name_token().ok()?;
        let name = name.text_trimmed();

        if RESTRICTEDNAMES.contains(&name) {
            Some(State {
                shadowed_name: name.to_string(),
            })
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();

        let diag = RuleDiagnostic::warning(
            binding.syntax().text_trimmed_range(),
            markup! {
                "Do not shadow the global \"" {state.shadowed_name} "\" property."
            },
        )
        .footer_note(
            markup! {"Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global."},
        );

        Some(diag)
    }
}
