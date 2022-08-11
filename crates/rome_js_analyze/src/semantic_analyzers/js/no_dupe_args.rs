use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyArrayBindingPatternElement, JsAnyBinding, JsAnyBindingPattern, JsAnyFunction,
    JsAnyObjectBindingPatternMember, JsAnyParameter, JsIdentifierBinding,
};
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

    type Query = Ast<JsAnyFunction>;
    type State = JsIdentifierBinding;
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
                let mut set = FxHashSet::default();
                // Traversing the parameters of the function in preorder and checking for duplicates,
                // reuse the `HashSet` and `Vec` to avoid extra allocations.
                for parameter in args.items().into_iter() {
                    let parameter = parameter.ok()?;
                    traverse_parameter(parameter, &mut set, &mut ret);
                }

                Some(ret)
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
    p: JsAnyParameter,
    set: &mut FxHashSet<String>,
    res: &mut Vec<JsIdentifierBinding>,
) -> Option<()> {
    match p {
        JsAnyParameter::JsAnyFormalParameter(p) => match p {
            rome_js_syntax::JsAnyFormalParameter::JsFormalParameter(parameter) => {
                traverse_binding(parameter.binding().ok()?, set, res);
            }
            rome_js_syntax::JsAnyFormalParameter::JsUnknownParameter(_) => {}
        },
        JsAnyParameter::JsRestParameter(rest_parameter) => {
            traverse_binding(rest_parameter.binding().ok()?, set, res);
        }
        JsAnyParameter::TsThisParameter(_) => {}
    }
    Some(())
}

fn traverse_binding(
    binding: JsAnyBindingPattern,
    set: &mut FxHashSet<String>,
    res: &mut Vec<JsIdentifierBinding>,
) -> Option<()> {
    match binding {
        JsAnyBindingPattern::JsAnyBinding(inner_binding) => match inner_binding {
            JsAnyBinding::JsIdentifierBinding(id_binding) => {
                check_binding(id_binding, set, res);
            }
            JsAnyBinding::JsUnknownBinding(_) => {}
        },
        JsAnyBindingPattern::JsArrayBindingPattern(inner_binding) => {
            for ele in inner_binding.elements().into_iter() {
                let ele = ele.ok()?;
                match ele {
                    JsAnyArrayBindingPatternElement::JsAnyBindingPattern(pattern) => {
                        traverse_binding(pattern, set, res);
                    }
                    JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                        binding_rest,
                    ) => {
                        let binding_pattern = binding_rest.pattern().ok()?;
                        traverse_binding(binding_pattern, set, res);
                    }
                    JsAnyArrayBindingPatternElement::JsArrayHole(_) => {}
                    JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(
                        binding_with_default,
                    ) => {
                        let pattern = binding_with_default.pattern().ok()?;
                        traverse_binding(pattern, set, res);
                    }
                }
            }
        }
        JsAnyBindingPattern::JsObjectBindingPattern(pattern) => {
            for prop in pattern.properties().into_iter() {
                let prop = prop.ok()?;
                match prop {
                    JsAnyObjectBindingPatternMember::JsIdentifierBinding(id_binding) => {
                        check_binding(id_binding, set, res);
                    }
                    JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(pattern) => {
						let pattern = pattern.pattern().ok()?;
						traverse_binding(pattern, set, res);
					},
                    JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                        let pattern = rest.binding().ok()?;
                        match pattern {
                            JsAnyBinding::JsIdentifierBinding(binding) => {
                                check_binding(binding, set, res);
                            }
                            JsAnyBinding::JsUnknownBinding(_) => {}
                        }
                    }
                    JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        shorthand_binding,
                    ) => match shorthand_binding.identifier().ok()? {
                        JsAnyBinding::JsIdentifierBinding(id_binding) => {
                            check_binding(id_binding, set, res)
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
fn check_binding(
    id_binding: JsIdentifierBinding,
    set: &mut FxHashSet<String>,
    res: &mut Vec<JsIdentifierBinding>,
) {
    let binding_text = id_binding.text();
    if set.contains(&binding_text) {
        res.push(id_binding);
    } else {
        set.insert(binding_text);
    }
}
