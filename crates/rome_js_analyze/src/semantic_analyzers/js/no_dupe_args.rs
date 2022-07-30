use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::SemanticScopeExtensions;
use rome_js_syntax::{
    JsAnyFunction, JsArrowFunctionExpression, JsFunctionDeclaration, JsFunctionExpression,
    JsIdentifierBinding, JsLabeledStatement, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::{declare_node_union, AstNode};
use rustc_hash::FxHashSet;

declare_rule! {
    ///  Disallow labels that share a name with a variable
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const x1 = "test";
    /// x1: expr;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const x = "test";
    /// z: expr;
    /// ```
    pub(crate) NoDupeArgs {
        version: "0.7.0",
        name: "noDupeArgs",
        recommended: true
    }
}

impl Rule for NoDupeArgs {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsAnyFunction>;
    /// The first element of the tuple is the name of the binding, the second element of the tuple is the label name
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
                    .find_map(|item| JsIdentifierBinding::cast(item))
                {
                    println!("{}", args.syntax());
                    let mut set = FxHashSet::default();
                    let model = ctx.model();
                    let scope = binding.scope(model);
                    for binding in scope.bindings() {
                        println!("{}", binding.syntax());
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
        // for arg in args.items() {
        //     let arg = arg.ok()?;
        // }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding_syntax_node = state;
        // let name = label_token.text_trimmed();

        Some(RuleDiagnostic::warning(
            binding_syntax_node.text_trimmed_range(),
            markup! {
                "Duplicate argument name"
            },
        ))
        // .secondary(binding_syntax_node.text_trimmed_range(), markup! {
        //     "The variable is declared here"
        // },)
        // .footer_note(markup! {"Creating a label with the same name as an in-scope variable leads to confusion."}))
    }
}

// declare_node_union! {
//     /// Matches an if statement or a conditional expression
//     pub(crate) JsAnyFunctionLike = JsFunctionDeclaration | JsFunctionExpression | JsArrowFunctionExpression
// }
