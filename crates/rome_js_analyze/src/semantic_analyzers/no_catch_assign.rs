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

        // println!("body: \n{}", body);
        println!("catch_binding: \n{}", catch_binding_syntax);
        for assignment in body
            .syntax()
            .descendants()
            .find_map(JsIdentifierAssignment::cast)
        {
            println!("decl_binding_syntax: {}", assignment.syntax());
            let decl_binding = model.declaration(&assignment).unwrap();
            if decl_binding.syntax() == catch_binding_syntax {
                invalid_assign.push(assignment);
            }
           
        }

        if !invalid_assign.is_empty() {
            Some(invalid_assign)
        } else {
            None
        }
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
