use crate::semantic_analyzers::nursery::no_undeclared_variables::AnyIdentifier;
use crate::semantic_services::SemanticServices;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::{Binding, DeclarationExtensions};
use rome_js_syntax::{JsReferenceIdentifier, JsxReferenceIdentifier, TextRange};
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
    pub(crate) AnyReferenceIdentifier = JsReferenceIdentifier |JsxReferenceIdentifier
}

const RESTRICTED_GLOBALS: [&str; 2] = ["event", "error"];

impl Rule for NoRestrictedGlobals {
    type Query = SemanticServices;
    type State = (TextRange, String);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        ctx.query()
            .all_unresolved_references()
            .chain(model.all_globals())
            .filter_map(|reference| {
                let node = reference.node().clone();
                let node = AnyIdentifier::unwrap_cast(node);
                let (token, declaration) = match node {
                    AnyIdentifier::JsReferenceIdentifier(node) => {
                        (node.value_token(), node.declaration(model))
                    }
                    AnyIdentifier::JsxReferenceIdentifier(node) => {
                        (node.value_token(), node.declaration(model))
                    }
                    AnyIdentifier::JsIdentifierAssignment(node) => {
                        (node.name_token(), node.declaration(model))
                    }
                };
                let token = token.ok()?;
                let text = token.text_trimmed();
                is_restricted(text, declaration).map(|text| (token.text_trimmed_range(), text))
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
