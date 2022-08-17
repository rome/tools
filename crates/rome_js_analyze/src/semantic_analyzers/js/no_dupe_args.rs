use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyArrayBindingPatternElement, JsAnyBinding, JsAnyBindingPattern, JsAnyFormalParameter,
    JsAnyObjectBindingPatternMember, JsAnyParameter, JsArrowFunctionExpression,
    JsFunctionDeclaration, JsFunctionExportDefaultDeclaration, JsFunctionExpression,
    JsIdentifierBinding, JsMethodClassMember, JsMethodObjectMember,
};
use rome_rowan::{declare_node_union, AstNode};
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

    type Query = Ast<JsAnyFunctionAndMethod>;
    type State = JsIdentifierBinding;
    type Signals = Vec<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let function = ctx.query();
        Some(vec![])
            .and_then(|mut identifier_binding_vec: Vec<Self::State>| {
                let args = match function {
                    JsAnyFunctionAndMethod::JsArrowFunctionExpression(func) => {
                        func.parameters().ok()?.as_js_parameters()?.clone()
                    }
                    JsAnyFunctionAndMethod::JsFunctionDeclaration(func) => {
                        func.parameters().ok()?
                    }
                    JsAnyFunctionAndMethod::JsFunctionExportDefaultDeclaration(func) => {
                        func.parameters().ok()?
                    }
                    JsAnyFunctionAndMethod::JsFunctionExpression(func) => func.parameters().ok()?,
                    JsAnyFunctionAndMethod::JsMethodClassMember(member) => {
                        member.parameters().ok()?
                    }
                    JsAnyFunctionAndMethod::JsMethodObjectMember(member) => {
                        member.parameters().ok()?
                    }
                };
                let mut set = FxHashSet::default();
                // Traversing the parameters of the function in preorder and checking for duplicates,
                // reuse the `HashSet` and `Vec` to avoid extra allocations.
                for parameter in args.items().into_iter() {
                    let parameter = parameter.ok()?;
                    traverse_parameter(parameter, &mut set, &mut identifier_binding_vec);
                }

                Some(identifier_binding_vec)
            })
            .unwrap_or_default()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding_syntax_node = state;
        Some(RuleDiagnostic::new(
            binding_syntax_node.syntax().text_trimmed_range(),
            markup! {
                "Duplicate argument name"
            },
        ))
    }
}

/// Traverse the parameter recursively and check if it is duplicated.
fn traverse_parameter(
    parameter: JsAnyParameter,
    binding_set: &mut FxHashSet<String>,
    identifier_vec: &mut Vec<JsIdentifierBinding>,
) -> Option<()> {
    match parameter {
        JsAnyParameter::JsAnyFormalParameter(p) => match p {
            JsAnyFormalParameter::JsFormalParameter(parameter) => {
                traverse_binding(parameter.binding().ok()?, binding_set, identifier_vec);
            }
            JsAnyFormalParameter::JsUnknownParameter(_) => {}
        },
        JsAnyParameter::JsRestParameter(rest_parameter) => {
            traverse_binding(rest_parameter.binding().ok()?, binding_set, identifier_vec);
        }
        JsAnyParameter::TsThisParameter(_) => {}
    }
    Some(())
}

fn traverse_binding(
    binding: JsAnyBindingPattern,
    binding_set: &mut FxHashSet<String>,
    identifier_vec: &mut Vec<JsIdentifierBinding>,
) -> Option<()> {
    match binding {
        JsAnyBindingPattern::JsAnyBinding(inner_binding) => match inner_binding {
            JsAnyBinding::JsIdentifierBinding(id_binding) => {
                track_binding(id_binding, binding_set, identifier_vec);
            }
            JsAnyBinding::JsUnknownBinding(_) => {}
        },
        JsAnyBindingPattern::JsArrayBindingPattern(inner_binding) => {
            for ele in inner_binding.elements().into_iter() {
                let ele = ele.ok()?;
                match ele {
                    JsAnyArrayBindingPatternElement::JsAnyBindingPattern(pattern) => {
                        traverse_binding(pattern, binding_set, identifier_vec);
                    }
                    JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                        binding_rest,
                    ) => {
                        let binding_pattern = binding_rest.pattern().ok()?;
                        traverse_binding(binding_pattern, binding_set, identifier_vec);
                    }
                    JsAnyArrayBindingPatternElement::JsArrayHole(_) => {}
                    JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(
                        binding_with_default,
                    ) => {
                        let pattern = binding_with_default.pattern().ok()?;
                        traverse_binding(pattern, binding_set, identifier_vec);
                    }
                }
            }
        }
        JsAnyBindingPattern::JsObjectBindingPattern(pattern) => {
            for prop in pattern.properties().into_iter() {
                let prop = prop.ok()?;
                match prop {
                    JsAnyObjectBindingPatternMember::JsIdentifierBinding(id_binding) => {
                        track_binding(id_binding, binding_set, identifier_vec);
                    }
                    JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(pattern) => {
                        let pattern = pattern.pattern().ok()?;
                        traverse_binding(pattern, binding_set, identifier_vec);
                    }
                    JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                        let pattern = rest.binding().ok()?;
                        match pattern {
                            JsAnyBinding::JsIdentifierBinding(binding) => {
                                track_binding(binding, binding_set, identifier_vec);
                            }
                            JsAnyBinding::JsUnknownBinding(_) => {}
                        }
                    }
                    JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        shorthand_binding,
                    ) => match shorthand_binding.identifier().ok()? {
                        JsAnyBinding::JsIdentifierBinding(id_binding) => {
                            track_binding(id_binding, binding_set, identifier_vec)
                        }
                        JsAnyBinding::JsUnknownBinding(_) => {}
                    },
                    JsAnyObjectBindingPatternMember::JsUnknownBinding(_) => {}
                }
            }
        }
    }
    Some(())
}

#[inline]
/// If the name of binding has been seen in set, then we push the `JsIdentifierBinding` into `identifier_vec`.
/// Else we mark the name of binding as seen.
fn track_binding(
    id_binding: JsIdentifierBinding,
    binding_set: &mut FxHashSet<String>,
    identifier_vec: &mut Vec<JsIdentifierBinding>,
) {
    let binding_text = id_binding.text();
    if binding_set.contains(&binding_text) {
        identifier_vec.push(id_binding);
    } else {
        binding_set.insert(binding_text);
    }
}

declare_node_union! {
    /// A union of all possible FunctionLike `JsAstNode` in the JS grammar.
    pub(crate) JsAnyFunctionAndMethod = JsArrowFunctionExpression| JsFunctionDeclaration| JsFunctionExportDefaultDeclaration | JsFunctionExpression | JsMethodClassMember | JsMethodObjectMember
}
