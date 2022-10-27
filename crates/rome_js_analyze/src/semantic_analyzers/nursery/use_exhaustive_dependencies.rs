use crate::react::hooks::*;
use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::Capture;
use rome_js_syntax::{
    JsCallExpression, JsIdentifierBinding, JsSyntaxKind, JsVariableDeclaration,
    JsVariableDeclarator, TextRange, TsIdentifierBinding,
};
use rome_rowan::{AstNode, SyntaxNodeCast};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

declare_rule! {
    /// Enforce all dependencies are correctly specified.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let a = 1;
    /// useEffect(() => {
    ///     console.log(a);
    /// })
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let b = 1;
    /// useEffect(() => {
    /// }, [b])
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const [name, setName] = useState();
    /// useEffect(() => {
    ///     console.log(name);
    ///     setName("");
    /// }, [name, setName])
    /// ```
    /// 
    /// ```js,expect_diagnostic
    /// let a = 1;
    /// const b = a + 1;
    /// useEffect(() => {
    ///     console.log(b);
    /// })
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// let a = 1;
    /// useEffect(() => {
    ///     console.log(a);
    /// }, [a]);
    /// ```
    /// 
    /// ```js
    /// const a = 1;
    /// useEffect(() => {
    ///     console.log(a);
    /// });
    /// ```
    ///
    /// ```js
    /// const [name, setName] = useState();
    /// useEffect(() => {
    ///     console.log(name);
    ///     setName("");
    /// }, [name])
    /// ```
    ///
    pub(crate) UseExhaustiveDependencies {
        version: "10.0.0",
        name: "useExhaustiveDependencies",
        recommended: false,
    }
}

lazy_static::lazy_static! {
    static ref OPTIONS: ReactExtensiveDependenciesOptions = ReactExtensiveDependenciesOptions::new();
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ReactExtensiveDependenciesOptions {
    hooks_config: HashMap<String, ReactHookConfiguration>,
    stable_config: HashSet<StableReactHookConfiguration>,
}

impl ReactExtensiveDependenciesOptions {
    pub fn new() -> Self {
        let hooks_config = HashMap::from_iter([
            ("useEffect".to_string(), (0, 1).into()),
            ("useLayoutEffect".to_string(), (0, 1).into()),
            ("useInsertionEffect".to_string(), (0, 1).into()),
            ("useCallback".to_string(), (0, 1).into()),
            ("useMemo".to_string(), (0, 1).into()),
            ("useImperativeHandle".to_string(), (1, 2).into()),
        ]);

        let stable_config: HashSet<StableReactHookConfiguration> = HashSet::from_iter([
            StableReactHookConfiguration::new("useState", Some(1)),
            StableReactHookConfiguration::new("useReducer", Some(1)),
            StableReactHookConfiguration::new("useTransition", Some(1)),
            StableReactHookConfiguration::new("useRef", None),
            StableReactHookConfiguration::new("useContext", None),
            StableReactHookConfiguration::new("useId", None),
            StableReactHookConfiguration::new("useSyncExternalStore", None),
        ]);

        Self {
            hooks_config,
            stable_config,
        }
    }
}

/// Flags the possible fixes that were found
pub enum Fix {
    /// When a dependency needs to be added.
    AddDependency(TextRange, Vec<Capture>),
    /// When a dependency needs to be removed.
    RemoveDependency(TextRange, Vec<TextRange>),
}

impl Rule for UseExhaustiveDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = Fix;
    type Signals = Vec<Self::State>;
    type Options = ReactExtensiveDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let options = ctx.options().unwrap_or(&OPTIONS);

        let mut signals = vec![];

        let node = ctx.query();
        if let Some(result) = react_hook_with_dependency(node, &options.hooks_config) {
            let model = ctx.model();

            let captures: Vec<_> = result
                .all_captures(model)
                .into_iter()
                .filter_map(|capture| {
                    capture.declaration().and_then(|declaration| {
                        let declaration_syntax = declaration.syntax();
                        let node = declaration_syntax.parent()?;
                        use JsSyntaxKind::*;
                        match node.kind() {
                            JS_FUNCTION_DECLARATION
                            | JS_CLASS_DECLARATION
                            | TS_ENUM_DECLARATION
                            | TS_TYPE_ALIAS_DECLARATION
                            | TS_DECLARE_FUNCTION_DECLARATION => None,
                            _ => {
                                // Ignore if imported
                                if let Some(true) = declaration_syntax
                                    .clone()
                                    .cast::<JsIdentifierBinding>()
                                    .map(|node| model.is_imported(&node))
                                    .or_else(|| {
                                        Some(
                                            model.is_imported(&node.cast::<TsIdentifierBinding>()?),
                                        )
                                    })
                                {
                                    None
                                } else {
                                    let binding = declaration
                                        .syntax()
                                        .clone()
                                        .cast::<JsIdentifierBinding>()?;

                                    // Ignore if constant
                                    if let Some(declarator) =
                                        binding.parent::<JsVariableDeclarator>()
                                    {
                                        let declaration = declarator
                                            .syntax()
                                            .ancestors()
                                            .filter_map(JsVariableDeclaration::cast)
                                            .next()?;

                                        if declaration.is_const() {
                                            let initializer = declarator.initializer()?;
                                            let expr = initializer.expression().ok()?;
                                            if model.is_constant(&expr) {
                                                return None;
                                            }
                                        }
                                    }

                                    // Ignore if stable
                                    let not_stable =
                                        !is_binding_react_stable(&binding, &options.stable_config);
                                    not_stable.then_some(capture)
                                }
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

            // Search for dependencies not captured
            for dep in deps {
                if !captures.iter().any(|x| x.0 == dep.0) {
                    remove_deps.push(dep.1);
                }
            }

            // Generate signals
            for (_, captures) in add_deps {
                signals.push(Fix::AddDependency(result.function_name_range, captures));
            }

            if !remove_deps.is_empty() {
                signals.push(Fix::RemoveDependency(
                    result.function_name_range,
                    remove_deps,
                ));
            }
        }

        signals
    }

    fn diagnostic(_: &RuleContext<Self>, dep: &Self::State) -> Option<RuleDiagnostic> {
        match dep {
            Fix::AddDependency(use_effect_range, captures) => {
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    use_effect_range,
                    markup! {
                        "This hook do not specify all of its dependencies."
                    },
                );

                for capture in captures.iter() {
                    let node = capture.node();
                    diag = diag.detail(
                        node.text_trimmed_range(),
                        "This dependency is not specified in the hook dependency list.",
                    );
                }

                Some(diag)
            }
            Fix::RemoveDependency(use_effect_range, ranges) => {
                let mut diag = RuleDiagnostic::new(
                    rule_category!(),
                    use_effect_range,
                    markup! {
                        "This hook specifies more dependencies than necessary."
                    },
                );

                for range in ranges.iter() {
                    diag = diag.detail(range, "This dependency can be removed from the list.");
                }

                Some(diag)
            }
        }
    }
}
