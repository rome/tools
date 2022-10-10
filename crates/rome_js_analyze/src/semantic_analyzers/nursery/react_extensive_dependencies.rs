use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::{Capture, SemanticModel};
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsArrayExpression, JsArrowFunctionExpression,
    JsCallExpression, JsSyntaxKind, TextRange,
};
use rome_rowan::AstNode;
use std::collections::HashMap;
use crate::utils::react::*;

declare_rule! {
    /// Enforce all dependencies are correctly specified.
    ///
    pub(crate) ReactExtensiveDependencies {
        version: "0.10.0",
        name: "reactExtensiveDependencies",
        recommended: false,
    }
}

impl Rule for ReactExtensiveDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = (TextRange, Vec<Capture>);
    type Signals = Vec<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let mut signals = vec![];

        let node = ctx.query();
        if let Some(use_effect) = ReactUseEffectCallExpression::new(node) {
            let range = match use_effect.callee_trimmed_range() {
                Some(range) => range,
                None => return signals,
            };

            let model = ctx.model();
            let function = use_effect.effect().unwrap();
            let captures: Vec<_> = function
                .all_captures(model)
                .filter_map(|capture| {
                    capture.declaration().and_then(|declaration| {
                        let node = declaration.syntax().parent()?;
                        use JsSyntaxKind::*;
                        match node.kind() {
                            JS_FUNCTION_DECLARATION 
                            | JS_CLASS_DECLARATION
                            | TS_ENUM_DECLARATION 
                            | TS_TYPE_ALIAS_DECLARATION 
                            | TS_DECLARE_FUNCTION_DECLARATION => None,
                            _ => Some(capture)
                        }
                    })
                })
                .map(|x| (x.node().text_trimmed().to_string(), x))
                .collect();

            let deps: Vec<String> = use_effect
                .deps()
                .map(|deps| {
                    deps.items()
                        .into_iter()
                        .map(|x| x.syntax().text_trimmed().to_string())
                        .collect()
                })
                .unwrap_or_default();

            dbg!(&captures.iter().map(|x| &x.0).collect::<Vec<_>>());
            dbg!(&deps);

            let mut add_deps: HashMap<String, Vec<Capture>> = HashMap::new();

            // Search for captures not in the dependency
            for (text, capture) in captures.iter() {
                if !deps.contains(text) {
                    let captures = add_deps.entry(text.clone()).or_default();
                    captures.push(capture.clone());
                }
            }

            // Search for dependencies not captured

            for (_, captures) in add_deps {
                signals.push((range, captures));
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, dep: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            dep.0,
            markup! {
                "This useEffect has missing dependencies"
            },
        );

        let mut diag = diag;

        for capture in dep.1.iter() {
            let node = capture.node();
            diag = diag.secondary(
                node.text_trimmed_range(),
                "This capture is not in the dependency list",
            );
        }

        Some(diag)
    }
}

struct ReactUseEffectCallExpression<'a> {
    call: &'a JsCallExpression,
    effect: Option<JsAnyCallArgument>,
    deps: Option<JsAnyCallArgument>,
}

pub enum ReactUseEffectEffect<'a> {
    JsArrowFunctionExpression(&'a JsArrowFunctionExpression),
}

impl<'a> ReactUseEffectEffect<'a> {
    pub fn all_captures(&self, model: &SemanticModel) -> impl Iterator<Item = Capture> {
        use ReactUseEffectEffect::*;
        match self {
            JsArrowFunctionExpression(node) => {
                let closure = model.closure(*node);
                closure.all_captures()
            }
        }
    }
}

pub enum ReactUseEffectDeps<'a> {
    JsArrayExpression(&'a JsArrayExpression),
}

impl<'a> ReactUseEffectDeps<'a> {
    pub fn items(&self) -> Vec<JsAnyExpression> {
        match self {
            ReactUseEffectDeps::JsArrayExpression(node) => node
                .elements()
                .into_iter()
                .filter_map(|x| x.ok()?.as_js_any_expression().cloned())
                .collect(),
        }
    }
}

impl<'a> ReactUseEffectCallExpression<'a> {
    pub fn new(call: &'a JsCallExpression) -> Option<Self> {
        let name = call.callee().ok()?.syntax().text_trimmed();
        (name == "useEffect").then(|| {
            let (effect, deps) = call
                .arguments()
                .map(|args| {
                    let mut args = args.args().into_iter();
                    let effect = args.next().and_then(|x| x.ok());
                    let deps = args.next().and_then(|x| x.ok());
                    (effect, deps)
                })
                .unwrap_or((None, None));

            Self { call, effect, deps }
        })
    }

    pub fn callee_trimmed_range(&self) -> Option<TextRange> {
        Some(self.call.callee().ok()?.syntax().text_trimmed_range())
    }

    pub fn effect(&self) -> Option<ReactUseEffectEffect> {
        let expr = self.effect.as_ref()?.as_js_any_expression()?;
        match expr.syntax().kind() {
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                let expr = expr.as_js_arrow_function_expression()?;
                Some(ReactUseEffectEffect::JsArrowFunctionExpression(expr))
            }
            _ => None,
        }
    }

    pub fn deps(&self) -> Option<ReactUseEffectDeps> {
        let expr = self.deps.as_ref()?.as_js_any_expression()?;
        match expr.syntax().kind() {
            JsSyntaxKind::JS_ARRAY_EXPRESSION => {
                let expr = expr.as_js_array_expression()?;
                Some(ReactUseEffectDeps::JsArrayExpression(expr))
            }
            _ => None,
        }
    }
}
