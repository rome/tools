use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_js_semantic::Binding;
use rome_js_syntax::{JsForStatement, JsReferenceIdentifier, JsVariableDeclaration};
use rome_rowan::AstNode;

declare_rule! {
    /// Require `const` declarations for variables that are never reassigned after declared.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let a = 3;
    /// console.log(a);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // `i` is redefined (not reassigned) on each loop step.
    /// for (let a of [1, 2, 3]) {
    ///     console.log(a);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // `a` is redefined (not reassigned) on each loop step.
    /// for (let a in [1, 2, 3]) {
    ///     console.log(a);
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// let a = 2;
    /// a = 3;
    /// console.log(a);
    /// ```
    pub(crate) UseConst {
        version: "11.0.0",
        name: "useConst",
        recommended: false,
    }
}

impl Rule for UseConst {
    type Query = Semantic<JsReferenceIdentifier>;
    type State = Binding;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let name = ctx.query();
        let model = ctx.model();

        let binding = model.declaration(name)?;
        let decl = binding
            .syntax()
            .ancestors()
            .find_map(JsVariableDeclaration::cast)?;
        let is_for_init = decl.parent::<JsForStatement>().is_some();
        if !is_for_init & !decl.is_const() && binding.all_writes().len() == 0 {
            Some(binding)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();
        let token = binding.value_token().ok()?;

        let diag = RuleDiagnostic::new(
            rule_category!(),
            state.syntax().text_trimmed_range(),
            markup! {
                "'"{ token.text_trimmed() }"' is never reassigned. Use 'const' instead."
            },
        );

        Some(diag)
    }

    fn action(_ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
