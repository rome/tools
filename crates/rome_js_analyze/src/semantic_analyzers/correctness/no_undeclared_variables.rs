use crate::globals::browser::BROWSER;
use crate::globals::node::NODE;
use crate::globals::runtime::ES_2021;
use crate::globals::typescript::TYPESCRIPT_BUILTIN;
use crate::semantic_services::SemanticServices;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{TextRange, TsAsExpression, TsReferenceType};
use rome_rowan::AstNode;

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

impl Rule for NoUndeclaredVariables {
    type Query = SemanticServices;
    type State = (TextRange, String);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query()
            .all_unresolved_references()
            .filter_map(|reference| {
                let identifier = reference.tree();
                let under_as_expression = identifier
                    .parent::<TsReferenceType>()
                    .and_then(|ty| ty.parent::<TsAsExpression>())
                    .is_some();

                let token = identifier.value_token().ok()?;
                let text = token.text_trimmed();

                // Typescript Const Assertion
                if text == "const" && under_as_expression {
                    return None;
                }

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
