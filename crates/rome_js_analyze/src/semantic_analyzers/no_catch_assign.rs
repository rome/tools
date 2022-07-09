use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsCatchClause, JsIdentifierAssignment};
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow reassigning exceptions in catch clauses
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///
    /// } catch (e) {
    ///   e;
    ///   e = 10;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// try {
    ///
    /// } catch (e) {
    ///   let e = 10;
    ///   e = 100;
    /// }
    /// ```
    pub(crate) NoCatchAssign = "noCatchAssign"
}

impl Rule for NoCatchAssign {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    /// Why use [JsCatchClause] instead of [JsIdentifierAssignment] ? Because this could reduce search range.
    /// We only compare the declaration of [JsCatchClause] with all descent [JsIdentifierAssignment] of its body.
    type Query = Semantic<JsCatchClause>;
    type State = Vec<JsIdentifierAssignment>;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let catch_clause = ctx.query();
        let model = ctx.model();

        let decl = catch_clause.declaration()?;

        // catch_binding
        // ## Example
        // try {

        // } catch (catch_binding) {
        //          ^^^^^^^^^^^^^
        // }
        let catch_binding = decl.binding().ok()?;
        let catch_binding_syntax = catch_binding.syntax();
        let body = catch_clause.body().ok()?;
        let mut invalid_assign = vec![];

        for assignment in body
            .syntax()
            .descendants()
            .filter_map(JsIdentifierAssignment::cast)
        {
            let decl_binding = model.declaration(&assignment).unwrap();
            if decl_binding.syntax() == catch_binding_syntax {
                invalid_assign.push(assignment);
            }
        }

        (!invalid_assign.is_empty()).then(|| invalid_assign)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let mut diagnostic = RuleDiagnostic::warning(
            node.syntax().text_trimmed_range(),
            markup! {
                " Do not "<Emphasis>"reassign catch parameters."</Emphasis>""
            },
        );

        for assign in state.iter() {
            diagnostic = diagnostic.secondary(
                assign.syntax().text_trimmed_range(),
                markup! {
                    "Don't re assign "<Emphasis>{assign.syntax().text_trimmed().to_string()}</Emphasis>"."
                },
            );
        }

        Some(diagnostic.footer_note("Use a local variable instead."))
    }

    fn action(_: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
