use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::ReferencesExtensions;
use rome_js_syntax::{JsCatchClause, JsSyntaxNode};
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow reassigning exceptions in catch clauses.
    ///
    /// Assignment to a `catch` parameter can be misleading and confusing.
    /// It is often unintended and indicative of a programmer error.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-ex-assign
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
    pub(crate) NoCatchAssign {
        version: "0.7.0",
        name: "noCatchAssign",
        recommended: true,
    }
}

impl Rule for NoCatchAssign {
    // Why use [JsCatchClause] instead of [JsIdentifierAssignment] ?
    // Because this could reduce search range.
    // We only compare the declaration of [JsCatchClause] with all descent
    // [JsIdentifierAssignment] of its body.
    type Query = Semantic<JsCatchClause>;
    // The first element of `State` is the reassignment of catch parameter,
    // the second element of `State` is the declaration of catch clause.
    type State = (JsSyntaxNode, JsSyntaxNode);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let catch_clause = ctx.query();
        let model = ctx.model();

        catch_clause
            .declaration()
            .and_then(|decl| {
                let catch_binding = decl.binding().ok()?;
                // Only [JsIdentifierBinding] is allowed to use `model.all_references` now,
                // so there's need to make sure this is a [JsIdentifierBinding].
                let identifier_binding = catch_binding
                    .as_any_js_binding()?
                    .as_js_identifier_binding()?;
                let catch_binding_syntax = catch_binding.syntax();
                let mut invalid_assignment = vec![];
                for reference in identifier_binding.all_writes(model) {
                    invalid_assignment
                        .push((reference.syntax().clone(), catch_binding_syntax.clone()));
                }

                Some(invalid_assignment)
            })
            .unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (assignment, catch_binding_syntax) = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                assignment.text_trimmed_range(),
                markup! {
                    "Reassigning a "<Emphasis>"catch parameter"</Emphasis>" is confusing."
                },
            )
            .detail(
                catch_binding_syntax.text_trimmed_range(),
                markup! {
                    "The "<Emphasis>"catch parameter"</Emphasis>" is declared here:"
                },
            )
            .note("Use a local variable instead."),
        )
    }
}
