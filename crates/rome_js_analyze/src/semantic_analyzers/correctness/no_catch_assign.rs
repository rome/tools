use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsCatchClause, JsSyntaxNode};
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
    pub(crate) NoCatchAssign {
        version: "0.7.0",
        name: "noCatchAssign",
        recommended: true,
    }
}

impl Rule for NoCatchAssign {
    /// Why use [JsCatchClause] instead of [JsIdentifierAssignment] ? Because this could reduce search range.
    /// We only compare the declaration of [JsCatchClause] with all descent [JsIdentifierAssignment] of its body.
    type Query = Semantic<JsCatchClause>;
    /// The first element of `State` is the reassignment of catch parameter, the second element of `State` is the declaration of catch clause.
    type State = (JsSyntaxNode, JsSyntaxNode);
    type Signals = Vec<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let catch_clause = ctx.query();
        let model = ctx.model();

        catch_clause
            .declaration()
            .and_then(|decl| {
                // catch_binding
                // ## Example
                // try {

                // } catch (catch_binding) {
                //          ^^^^^^^^^^^^^
                // }
                let catch_binding = decl.binding().ok()?;
                // Only [JsIdentifierBinding] is allowed to use `model.all_references` now, so I need to make sure this is
                // a [JsIdentifierBinding].
                let identifier_binding = catch_binding
                    .as_js_any_binding()?
                    .as_js_identifier_binding()?;
                let catch_binding_syntax = catch_binding.syntax();
                let mut invalid_assignment = vec![];
                for reference in model.all_writes(identifier_binding) {
                    invalid_assignment
                        .push((reference.node().clone(), catch_binding_syntax.clone()));
                }

                Some(invalid_assignment)
            })
            .unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (assignment, catch_binding_syntax) = state;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            assignment.text_trimmed_range(),
            markup! {
                " Do not "<Emphasis>"reassign catch parameters."</Emphasis>""
            },
        )
        .secondary(
            catch_binding_syntax.text_trimmed_range(),
            markup! {
                "The catch parameter is declared here"
            },
        );

        Some(diagnostic.footer_note("Use a local variable instead."))
    }

    fn action(_: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
