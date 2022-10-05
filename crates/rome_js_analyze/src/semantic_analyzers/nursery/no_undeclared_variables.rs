use crate::globals::browser::BROWSER;
use crate::globals::node::NODE;
use crate::globals::runtime::ES_2021;
use crate::globals::typescript::TYPESCRIPT_BUILTIN;
use crate::semantic_services::SemanticServices;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsIdentifierAssignment, JsReferenceIdentifier, JsxReferenceIdentifier, TextRange,
};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Prevents the usage of variables that haven't been declared inside the document
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foobar;
    /// ```
    pub(crate) NoUndeclaredVariables {
        version: "0.10.0",
        name: "noUndeclaredVariables",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) AnyIdentifier = JsReferenceIdentifier | JsIdentifierAssignment | JsxReferenceIdentifier
}

impl Rule for NoUndeclaredVariables {
    type Query = SemanticServices;
    type State = (TextRange, String);
    type Signals = Vec<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query()
            .all_unresolved_references()
            .filter_map(|reference| {
                let node = reference.node().clone();
                let node = AnyIdentifier::unwrap_cast(node);
                let token = match node {
                    AnyIdentifier::JsReferenceIdentifier(node) => node.value_token(),
                    AnyIdentifier::JsIdentifierAssignment(node) => node.name_token(),
                    AnyIdentifier::JsxReferenceIdentifier(node) => node.value_token(),
                };

                let token = token.ok()?;
                let text = token.text_trimmed();
                if is_global(text) {
                    return None;
                }

                let span = token.text_trimmed_range();
                let text = text.to_string();
                Some((span, text))
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (span, name): &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            *span,
            markup! {
                "The "<Emphasis>{name}</Emphasis>" variable is undeclared"
            },
        ))
    }
}

fn is_global(reference_name: &str) -> bool {
    ES_2021.binary_search(&reference_name).is_ok()
        || BROWSER.binary_search(&reference_name).is_ok()
        || NODE.binary_search(&reference_name).is_ok()
        || TYPESCRIPT_BUILTIN.binary_search(&reference_name).is_ok()
}
