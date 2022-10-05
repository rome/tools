use crate::globals::browser::BROWSER;
use crate::globals::node::NODE;
use crate::globals::runtime::ES_2021;
use crate::globals::typescript::TYPESCRIPT_BUILTIN;
use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::Scope;
use rome_js_syntax::{JsReferenceIdentifier, JsxReferenceIdentifier};
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
    pub(crate) NoUndeclaredVariablesQuery = JsReferenceIdentifier| JsxReferenceIdentifier
}

impl Rule for NoUndeclaredVariables {
    type Query = Semantic<NoUndeclaredVariablesQuery>;
    type State = String;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match node {
            NoUndeclaredVariablesQuery::JsxReferenceIdentifier(reference) => {
                let value = reference.value_token().ok()?;
                let reference_name = value.text_trimmed();
                let scope = model.scope(reference.syntax());
                if !is_declared(reference_name, scope) {
                    return Some(reference_name.to_string());
                }
            }
            NoUndeclaredVariablesQuery::JsReferenceIdentifier(reference) => {
                let value = reference.value_token().ok()?;
                let reference_name = value.text_trimmed();
                let scope = model.scope(reference.syntax());
                if !is_declared(reference_name, scope) {
                    return Some(reference_name.to_string());
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "The "<Emphasis>{state}</Emphasis>" variable is undeclared"
            },
        ))
    }
}

fn is_declared(reference_name: &str, scope: Scope) -> bool {
    let binding = scope.get_binding(reference_name);

    // TODO: add here the check for global variables defined in the configuration, currently not supported
    binding.is_some()
        || ES_2021.binary_search(&reference_name).is_ok()
        || BROWSER.binary_search(&reference_name).is_ok()
        || NODE.binary_search(&reference_name).is_ok()
        || TYPESCRIPT_BUILTIN.binary_search(&reference_name).is_ok()
}
