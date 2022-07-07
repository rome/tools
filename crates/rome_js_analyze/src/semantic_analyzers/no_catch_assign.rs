use std::ops::BitAndAssign;

use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyAssignment, JsAnyExpression, JsAnyStatement, JsCatchClause, JsExpressionStatement,
    JsIdentifierAssignment, JsReferenceIdentifier, JsSyntaxKind,
};
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of ```arguments```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///    console.log(arguments);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// /// ```js
    /// function f() {
    ///     let arguments = 1;
    ///     console.log(arguments);
    /// }
    /// ```
    pub(crate) NoCatchAssign = "noCatchAssign"
}

impl Rule for NoCatchAssign {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsCatchClause>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let catch_clause = ctx.query();
        let model = ctx.model();
        let decl = catch_clause.declaration()?;
        let decl_scope = model.scope(decl.syntax());
        let catch_binding_name = decl.syntax().text_trimmed();
        let body = catch_clause.body().ok()?;
        for assignment in body
            .syntax()
            .descendants()
            .find_map(JsIdentifierAssignment::cast)
        {
            let name = assignment.name_token().ok()?;
            if name.text_trimmed() != catch_binding_name {
                continue;
            }
            let mut cur_scope = Some(ctx.model().scope(assignment.syntax()));
            while let Some(scope) = cur_scope {
                let binding = scope.get_binding(name.text_trimmed());
                if binding.is_some() && scope.get_id() == decl_scope.get_id() {
                    // print!("found you: {:?}, {}", scope, binding_name);
                    return Some(());
                }
                cur_scope = scope.parent();
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::warning(
            node.syntax().text_trimmed_range(),
            markup! {
                "Use the "<Emphasis>"rest parameters"</Emphasis>" instead of "<Emphasis>"arguments"</Emphasis>"."
            },
        ).footer_note(markup! {<Emphasis>"arguments"</Emphasis>" does not have "<Emphasis>"Array.prototype"</Emphasis>" methods and can be inconvenient to use."}))
    }

    fn action(_: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
