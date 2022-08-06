use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::SemanticScopeExtensions;
use rome_js_syntax::{JsAnyFunction, JsIdentifierBinding, JsSyntaxNode};
use rome_rowan::AstNode;
use rustc_hash::FxHashSet;

declare_rule! {
    ///  Disallow duplicate function arguments name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var f = function(a, b, b) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function b(a, b, b) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function i(i, b, c) {}
    /// var j = function (j, b, c) {};
    /// function k({ k, b }, { c, d }) {}
    /// function l([, l]) {}
    /// function foo([[a, b], [c, d]]) {}
    /// ```
    pub(crate) NoDupeArgs {
        version: "0.9.0",
        name: "noDupeArgs",
        recommended: true,
    }
}

impl Rule for NoDupeArgs {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsAnyFunction>;
    type State = JsSyntaxNode;
    type Signals = Vec<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let function = ctx.query();
        Some(vec![])
            .and_then(|mut ret: Vec<Self::State>| {
                let args = match function {
                    JsAnyFunction::JsArrowFunctionExpression(func) => {
                        func.parameters().ok()?.as_js_parameters()?.clone()
                    }
                    JsAnyFunction::JsFunctionDeclaration(func) => func.parameters().ok()?,
                    JsAnyFunction::JsFunctionExportDefaultDeclaration(func) => {
                        func.parameters().ok()?
                    }
                    JsAnyFunction::JsFunctionExpression(func) => func.parameters().ok()?,
                };
                if let Some(binding) = args
                    .syntax()
                    .descendants()
                    .find_map(JsIdentifierBinding::cast)
                {
                    let mut set = FxHashSet::default();
                    let model = ctx.model();
                    let scope = binding.scope(model);
                    for binding in scope.bindings() {
                        let name = binding.syntax().text_trimmed().to_string();
                        if set.contains(&name) {
                            ret.push(binding.syntax().clone());
                        } else {
                            set.insert(name);
                        }
                    }
                };
                Some(ret)
            })
            .unwrap_or_default()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding_syntax_node = state;
        Some(RuleDiagnostic::new(
            binding_syntax_node.text_trimmed_range(),
            markup! {
                "Duplicate argument name"
            },
        ))
    }
}
