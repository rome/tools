use crate::semantic_services::Semantic;
use crate::utils::react::*;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::Capture;
use rome_js_syntax::{JsCallExpression, JsIdentifierBinding, JsSyntaxKind, TextRange};
use rome_rowan::{AstNode, SyntaxNodeCast};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

declare_rule! {
    /// Enforce all dependencies are correctly specified.
    ///
    pub(crate) ReactExtensiveDependencies {
        version: "10.0.0",
        name: "reactExtensiveDependencies",
        recommended: false,
    }
}

lazy_static::lazy_static! {
    static ref OPTIONS: ReactExtensiveDependenciesOptions = ReactExtensiveDependenciesOptions::new();
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReactExtensiveDependenciesOptions {
    hooks: HashMap<String, ReactHookClosureDependenciesPosition>,
    stables: HashSet<ReactHookStable>,
}

impl ReactExtensiveDependenciesOptions {
    pub fn new() -> Self {
        let hooks = HashMap::from_iter([
            ("useEffect".to_string(), (0, 1).into()),
            ("useLayoutEffect".to_string(), (0, 1).into()),
            ("useInsertionEffect".to_string(), (0, 1).into()),
            ("useCallback".to_string(), (0, 1).into()),
            ("useMemo".to_string(), (0, 1).into()),
            ("useImperativeHandle".to_string(), (1, 2).into()),
        ]);

        let stables: HashSet<ReactHookStable> = HashSet::from_iter([
            ReactHookStable::new("useState", Some(1)),
            ReactHookStable::new("useReducer", Some(1)),
            ReactHookStable::new("useTransition", Some(1)),
            ReactHookStable::new("useRef", None),
            ReactHookStable::new("useContext", None),
            ReactHookStable::new("useId", None),
            ReactHookStable::new("useSyncExternalStore", None),
        ]);

        Self { hooks, stables }
    }
}

pub enum Problem {
    MissingDependency(TextRange, Vec<Capture>),
    ExtraDependency(TextRange, TextRange),
}

impl Rule for ReactExtensiveDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = Problem;
    type Signals = Vec<Self::State>;
    type Options = ReactExtensiveDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let options = ctx.options()
            .unwrap_or(&OPTIONS);

        let mut signals = vec![];

        let node = ctx.query();
        if let Some(result) = react_hook_with_dependency(node, &options.hooks) {
            let model = ctx.model();

            let captures: Vec<_> = result
                .all_captures(model)
                .into_iter()
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

            let deps: Vec<(String, TextRange)> = result
                .all_dependencies()
                .into_iter()
                .map(|dep| {
                    (
                        dep.syntax().text_trimmed().to_string(),
                        dep.syntax().text_trimmed_range(),
                    )
                })
                .collect();

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
                signals.push(Problem::MissingDependency(
                    result.function_name_range,
                    captures,
                ));
            }

            for dep_range in remove_deps {
                signals.push(Problem::ExtraDependency(
                    result.function_name_range,
                    dep_range,
                ));
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
            }
            Problem::ExtraDependency(use_effect_range, dep_range) => {
                let diag = RuleDiagnostic::new(
                    rule_category!(),
                    use_effect_range,
                    markup! {
                        "This useEffect has dependencies that were not captured"
                    },
                )
                .secondary(dep_range, "This dependecy is not being captured");

                Some(diag)
            }
        }
    }
}
