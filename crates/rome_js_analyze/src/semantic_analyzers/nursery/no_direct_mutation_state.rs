use crate::semantic_services::{Semantic, SemanticServices};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsCallExpression, JsSyntaxKind, JsThisExpression, JsStaticMemberAssignment, TextRange, JsReferenceIdentifier, JsIdentifierExpression, JsExtendsClause, JsClassDeclaration, AnyJsIdentifierUsage};
use rome_rowan::AstNode;

declare_rule! {
    /// Put your description here
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// ```
    ///
    pub(crate) NoDirectMutationState {
        version: "vNext",
        name: "noDirectMutationState",
        recommended: false,
    }
}

fn is_under_class_declaration_extends(reference: &AnyJsIdentifierUsage) -> Option<JsClassDeclaration> {
    reference.parent::<JsIdentifierExpression>()?
        .parent::<JsExtendsClause>()?
        .parent::<JsClassDeclaration>()
}

impl Rule for NoDirectMutationState {
    type Query = SemanticServices;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let mut signals = vec![];

        let model = ctx.query();

        let Some(import) = model.import("react") else {
            return vec![];
        };
        
        let Some(binding) = import.binding_importing("Component") else {
            return vec![];
        };

        for r in binding.all_reads() {
            let reference = r.tree();
            if let Some(class) = is_under_class_declaration_extends(&reference) {
                for member in class.members() {
                    for this_reference in model.all_references_to_this(member) {
                        let this_reference = this_reference.tree();
                        if let Some(member) = this_reference.parent::<JsStaticMemberAssignment>() {
                            if member.text() == "this.state" {
                                signals.push(member.syntax().text_trimmed_range());
                            }
                        }
                    }
                }
            }            
        }
        
        signals
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Do not mutate React components state directly"
                },
            )
            .note(markup! {
                "See React dpcumentation"
            }),
        )        
    }
}
