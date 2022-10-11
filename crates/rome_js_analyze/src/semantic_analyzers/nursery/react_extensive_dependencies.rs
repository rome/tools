use crate::semantic_services::Semantic;
use crate::utils::react::*;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::{Capture, SemanticModel};
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsArrayExpression, JsArrowFunctionExpression,
    JsCallExpression, JsIdentifierBinding, JsSyntaxKind, TextRange,
};
use rome_rowan::{AstNode, SyntaxNodeCast};
use serde::{Serialize, Deserialize};
use std::{collections::{HashMap, HashSet, BTreeMap}, borrow::Cow};

declare_rule! {
    /// Enforce all dependencies are correctly specified.
    ///
    pub(crate) ReactExtensiveDependencies {
        version: "0.10.0",
        name: "reactExtensiveDependencies",
        recommended: false,
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReactExtensiveDependenciesOptions {
    stables: HashSet<ReactHookStable>,
}

impl Default for ReactExtensiveDependenciesOptions {
    fn default() -> Self {
        let stables: HashSet<ReactHookStable> = HashSet::from_iter([
            ReactHookStable::new("useState", Some(1)),
            ReactHookStable::new("useReducer", Some(1)),
            ReactHookStable::new("useTransition", Some(1)),
            ReactHookStable::new("useRef", None),
            ReactHookStable::new("useContext", None),
            ReactHookStable::new("useId", None),
            ReactHookStable::new("useSyncExternalStore", None),
        ]);

        Self { stables }
    }
}

pub enum Problem {
    MissingDependency (TextRange, Vec<Capture>),
    ExtraDependency(TextRange, TextRange)
}

impl Rule for ReactExtensiveDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = Problem;
    type Signals = Vec<Self::State>;
    type Options = ReactExtensiveDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let options = ctx.options()
            .map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(ReactExtensiveDependenciesOptions::default()));
        
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
                            _ => {
                                let declaration =
                                    declaration.syntax().clone().cast::<JsIdentifierBinding>()?;
                                let not_stable = !is_stable_binding(&declaration, &options.stables);
                                not_stable.then_some(capture)
                            }
                        }
                    })
                })
                .map(|x| (x.node().text_trimmed().to_string(), x))
                .collect();

            let deps: Vec<(String, TextRange)> = use_effect
                .deps()
                .map(|deps| {
                    deps.items()
                        .into_iter()
                        .map(|x| (
                            x.syntax().text_trimmed().to_string(),
                            x.syntax().text_trimmed_range(),
                        ))
                        .collect()
                })
                .unwrap_or_default();

            let mut add_deps: BTreeMap<String, Vec<Capture>> = BTreeMap::new();
            let mut remove_deps: Vec<TextRange> = vec![];

            // Search for captures not in the dependency
            for (text, capture) in captures.iter() {
                if !deps.iter().any(|x| &x.0 == text) {
                    let captures = add_deps.entry(text.clone()).or_default();
                    captures.push(capture.clone());
                }
            }

            //TODO Search for dependencies not captured
            for dep in deps {
                if !captures.iter().any(|x| x.0 == dep.0) {
                    remove_deps.push(dep.1);
                }
            }

            // Generate signals
            for (_, captures) in add_deps {
                signals.push(Problem::MissingDependency(range, captures));
            }

            for dep_range in remove_deps {
                signals.push(Problem::ExtraDependency(range, dep_range));
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, dep: &Self::State) -> Option<RuleDiagnostic> {
        match dep {
            Problem::MissingDependency(use_effect_range, captures) => {
                let diag = RuleDiagnostic::new(
                    rule_category!(),
                    use_effect_range,
                    markup! {
                        "This useEffect has missing dependencies"
                    },
                );
    
                let mut diag = diag;
    
                for capture in captures.iter() {
                    let node = capture.node();
                    diag = diag.secondary(
                        node.text_trimmed_range(),
                        "This capture is not in the dependency list",
                    );
                }
    
                Some(diag)
            },
            Problem::ExtraDependency(use_effect_range, dep_range) => {
                let diag = RuleDiagnostic::new(
                    rule_category!(),
                    use_effect_range,
                    markup! {
                        "This useEffect has dependencies that were not captured"
                    },
                ).secondary(
                    dep_range,
                    "This dependecy is not being captured",
                );
    
                Some(diag)
            }
        }
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
