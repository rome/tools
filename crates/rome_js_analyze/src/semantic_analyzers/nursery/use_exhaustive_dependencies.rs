use crate::react::hooks::*;
use crate::semantic_services::Semantic;
use rome_analyze::{
    context::RuleContext, declare_rule, DeserializableRuleOptions, Rule, RuleDiagnostic,
};
use rome_console::markup;
use rome_js_syntax::{
    JsCallExpression, JsIdentifierBinding, JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode,
    JsVariableDeclaration, JsVariableDeclarator, TextRange, TsIdentifierBinding,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactExtensiveDependenciesOptions {
    hooks_config: HashMap<String, ReactHookConfiguration>,
    stable_config: HashSet<StableReactHookConfiguration>,
}

impl Default for ReactExtensiveDependenciesOptions {
    fn default() -> Self {
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

impl DeserializableRuleOptions for ReactExtensiveDependenciesOptions {
    fn try_from(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        #[derive(Debug, Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Options {
            #[serde(default)]
            hooks: Vec<(String, usize, usize)>,
            #[serde(default)]
            stables: HashSet<StableReactHookConfiguration>,
        }

        let options: Options = serde_json::from_value(value)?;

        let mut default = ReactExtensiveDependenciesOptions::default();
        for (k, closure_index, dependencies_index) in options.hooks.into_iter() {
            default.hooks_config.insert(
                k,
                ReactHookConfiguration {
                    closure_index,
                    dependencies_index,
                },
            );
        }
        default
            .stable_config
            .extend(options.stables.into_iter());

        Ok(default)
    }
}

/// Flags the possible fixes that were found
pub enum Fix {
    /// When a dependency needs to be added.
    AddDependency(TextRange, Vec<TextRange>),
    /// When a dependency needs to be removed.
    RemoveDependency(TextRange, Vec<TextRange>),
    /// When a dependency is more deep than the capture
    DependencyTooDeep {
        function_name_range: TextRange,
        capture_range: TextRange,
        dependency_range: TextRange,
    },
}

fn get_whole_static_member_expression(
    reference: &JsSyntaxNode,
) -> Option<JsStaticMemberExpression> {
    let root = reference
        .ancestors()
        .skip(2) //IDENT and JS_REFERENCE_IDENTIFIER
        .take_while(|x| x.kind() == JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION)
        .last()?;
    root.cast()
}

impl Rule for UseExhaustiveDependencies {
    type Query = Semantic<JsCallExpression>;
    type State = Fix;
    type Signals = Vec<Self::State>;
    type Options = ReactExtensiveDependenciesOptions;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let options = ctx.options();

        let mut signals = vec![];

        let node = ctx.query();
        if let Some(result) = react_hook_with_dependency(node, &options.hooks_config) {
            let model = ctx.model();

            let captures: Vec<_> = result
                .all_captures(model)
                .into_iter()
                .filter_map(|capture| {
                    let binding = capture.binding();
                    let binding_syntax = binding.syntax();
                    let node = binding_syntax.parent()?;
                    use JsSyntaxKind::*;
                    match node.kind() {
                        JS_FUNCTION_DECLARATION
                        | JS_CLASS_DECLARATION
                        | TS_ENUM_DECLARATION
                        | TS_TYPE_ALIAS_DECLARATION
                        | TS_DECLARE_FUNCTION_DECLARATION => None,
                        _ => {
                            // Ignore if imported
                            if let Some(true) = binding_syntax
                                .clone()
                                .cast::<JsIdentifierBinding>()
                                .map(|node| model.is_imported(&node))
                                .or_else(|| {
                                    Some(model.is_imported(&node.cast::<TsIdentifierBinding>()?))
                                })
                            {
                                None
                            } else {
                                let binding =
                                    binding.syntax().clone().cast::<JsIdentifierBinding>()?;

                                // Ignore if constant
                                if let Some(declarator) = binding.parent::<JsVariableDeclarator>() {
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
                .map(|capture| {
                    let path = get_whole_static_member_expression(capture.node());

                    let (text, range) = if let Some(path) = path {
                        (
                            path.syntax().text_trimmed().to_string(),
                            path.syntax().text_trimmed_range(),
                        )
                    } else {
                        (
                            capture.node().text_trimmed().to_string(),
                            capture.node().text_trimmed_range(),
                        )
                    };

                    (text, range, capture)
                })
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

            let mut add_deps: BTreeMap<String, Vec<TextRange>> = BTreeMap::new();
            let mut remove_deps: Vec<TextRange> = vec![];

            // Evaluate all the captures
            for (capture_text, capture_range, _) in captures.iter() {
                let mut suggested_fix = None;
                let mut is_captured_covered = false;
                for (dependency_text, dependency_range) in deps.iter() {
                    let capture_deeper_than_dependency = capture_text.starts_with(dependency_text);
                    let dependency_deeper_than_capture = dependency_text.starts_with(capture_text);
                    match (
                        capture_deeper_than_dependency,
                        dependency_deeper_than_capture,
                    ) {
                        // capture == dependency
                        (true, true) => {
                            suggested_fix = None;
                            is_captured_covered = true;
                            break;
                        }
                        // example
                        // capture: a.b.c
                        // dependency: a
                        // this is ok, but we may suggest performance improvements
                        // in the future
                        (true, false) => {
                            // We need to continue, because it may still have a perfect match
                            // in the dependency list
                            is_captured_covered = true;
                        }
                        // example
                        // capture: a.b
                        // dependency: a.b.c
                        // This can be valid in some cases. We will flag an error nonetheless.
                        (false, true) => {
                            // We need to continue, because it may still have a perfect match
                            // in the dependency list
                            suggested_fix = Some(Fix::DependencyTooDeep {
                                function_name_range: result.function_name_range,
                                capture_range: *capture_range,
                                dependency_range: *dependency_range,
                            });
                        }
                        _ => {}
                    }
                }

                if let Some(fix) = suggested_fix {
                    signals.push(fix);
                }

                if !is_captured_covered {
                    let captures = add_deps.entry(capture_text.clone()).or_default();
                    captures.push(*capture_range);
                }
            }

            // Search for dependencies not captured
            for (dependency_text, dep_range) in deps {
                let mut covers_any_capture = false;
                for (capture_text, _, _) in captures.iter() {
                    let capture_deeper_dependency = capture_text.starts_with(&dependency_text);
                    let dependency_deeper_capture = dependency_text.starts_with(capture_text);
                    if capture_deeper_dependency || dependency_deeper_capture {
                        covers_any_capture = true;
                        break;
                    }
                }

                if !covers_any_capture {
                    remove_deps.push(dep_range);
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

                for range in captures.iter() {
                    diag = diag.detail(
                        range,
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
            Fix::DependencyTooDeep {
                function_name_range,
                capture_range,
                dependency_range,
            } => {
                let diag = RuleDiagnostic::new(
                    rule_category!(),
                    function_name_range,
                    markup! {
                        "This hook specifies a dependency more specific that its captures"
                    },
                )
                .detail(capture_range, "This capture is more generic than...")
                .detail(dependency_range, "...this dependency.");
                Some(diag)
            }
        }
    }
}
