use crate::semantic_services::SemanticServices;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::{Binding, BindingExtensions};
use rome_js_syntax::{
    JsIdentifierAssignment, JsReferenceIdentifier, JsxReferenceIdentifier, TextRange,
};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// This rule allows you to specify global variable names that you don’t want to use in your application.
    ///
    /// > Disallowing usage of specific global variables can be useful if you want to allow a set of
    /// global variables by enabling an environment, but still want to disallow some of those.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// console.log(event)
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// function f(event) {
    ///     console.log(event)
    /// }
    /// ```
    pub(crate) NoRestrictedGlobals {
        version: "0.10.0",
        name: "noRestrictedGlobals",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) AnyIdentifier = JsReferenceIdentifier | JsIdentifierAssignment | JsxReferenceIdentifier
}

const RESTRICTED_GLOBALS: [&str; 2] = ["event", "error"];

impl Rule for NoRestrictedGlobals {
    type Query = SemanticServices;
    type State = (TextRange, String);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();

        let unresolved_reference_nodes = model
            .all_unresolved_references()
            .map(|reference| reference.syntax().clone());
        let global_references_nodes = model
            .all_global_references()
            .map(|reference| reference.syntax().clone());

        unresolved_reference_nodes
            .chain(global_references_nodes)
            .filter_map(|node| {
                let node = AnyIdentifier::unwrap_cast(node);
                let (token, binding) = match node {
                    AnyIdentifier::JsReferenceIdentifier(node) => {
                        (node.value_token(), node.binding(model))
                    }
                    AnyIdentifier::JsxReferenceIdentifier(node) => {
                        (node.value_token(), node.binding(model))
                    }
                    AnyIdentifier::JsIdentifierAssignment(node) => {
                        (node.name_token(), node.binding(model))
                    }
                };
                let token = token.ok()?;
                let text = token.text_trimmed();
                is_restricted(text, binding).map(|text| (token.text_trimmed_range(), text))
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (span, text): &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *span,
                markup! {
                    "Do not use the global variable "<Emphasis>{text}</Emphasis>"."
                },
            )
            .note(markup! {
                "Use a local variable instead."
            }),
        )
    }
}

fn is_restricted(name: &str, binding: Option<Binding>) -> Option<String> {
    if binding.is_none() && RESTRICTED_GLOBALS.contains(&name) {
        Some(name.to_string())
    } else {
        None
    }
}
