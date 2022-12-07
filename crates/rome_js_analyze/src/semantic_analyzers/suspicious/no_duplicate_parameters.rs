use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern, AnyJsFormalParameter,
    AnyJsObjectBindingPatternMember, AnyJsParameter, JsArrowFunctionExpression,
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
    pub(crate) NoDuplicateParameters {
        version: "0.9.0",
        name: "noDuplicateParameters",
        recommended: true,
    }
}

impl Rule for NoDuplicateParameters {
    type Query = Ast<AnyJsFunctionAndMethod>;
    type State = JsIdentifierBinding;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let function = ctx.query();
        let args = match function {
            AnyJsFunctionAndMethod::JsArrowFunctionExpression(func) => {
                func.parameters().ok()?.as_js_parameters()?.clone()
            }
            AnyJsFunctionAndMethod::JsFunctionDeclaration(func) => func.parameters().ok()?,
            AnyJsFunctionAndMethod::JsFunctionExportDefaultDeclaration(func) => {
                func.parameters().ok()?
            }
            AnyJsFunctionAndMethod::JsFunctionExpression(func) => func.parameters().ok()?,
            AnyJsFunctionAndMethod::JsMethodClassMember(member) => member.parameters().ok()?,
            AnyJsFunctionAndMethod::JsMethodObjectMember(member) => member.parameters().ok()?,
        };
        let mut set = FxHashSet::default();
        // Traversing the parameters of the function in preorder and checking for duplicates,
        args.items().into_iter().find_map(|parameter| {
            let parameter = parameter.ok()?;
            traverse_parameter(parameter, &mut set)
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding_syntax_node = state;
        Some(RuleDiagnostic::new(
            rule_category!(),
            binding_syntax_node.syntax().text_trimmed_range(),
            markup! {
                "Duplicate argument name"
            },
        ))
    }
}

/// Traverse the parameter recursively and check if it is duplicated.
/// Return `Some(JsIdentifierBinding)` if it is duplicated.
fn traverse_parameter(
    parameter: AnyJsParameter,
    tracked_bindings: &mut FxHashSet<String>,
) -> Option<JsIdentifierBinding> {
    match parameter {
        AnyJsParameter::AnyJsFormalParameter(p) => match p {
            AnyJsFormalParameter::JsFormalParameter(parameter) => {
                traverse_binding(parameter.binding().ok()?, tracked_bindings)
            }
            AnyJsFormalParameter::JsBogusParameter(_) => None,
        },
        AnyJsParameter::JsRestParameter(rest_parameter) => {
            traverse_binding(rest_parameter.binding().ok()?, tracked_bindings)
        }
        AnyJsParameter::TsThisParameter(_) => None,
    }
}

/// Traverse a [JsAnyBindingPattern] in preorder and check if the name of [JsIdentifierBinding] has seem before.
/// If true then add the [JsIdentifierBinding] to the [duplicated_arguments].
/// If false then add the [JsIdentifierBinding] to the [tracked_bindings], mark it name as seen.
/// If it is not a [JsIdentifierBinding] then recursively call [traverse_binding] on its children.
fn traverse_binding(
    binding: AnyJsBindingPattern,
    tracked_bindings: &mut FxHashSet<String>,
) -> Option<JsIdentifierBinding> {
    match binding {
        AnyJsBindingPattern::AnyJsBinding(inner_binding) => match inner_binding {
            AnyJsBinding::JsIdentifierBinding(id_binding) => {
                if track_binding(&id_binding, tracked_bindings) {
                    return Some(id_binding);
                }
            }
            AnyJsBinding::JsBogusBinding(_) => {}
        },
        AnyJsBindingPattern::JsArrayBindingPattern(inner_binding) => {
            return inner_binding.elements().into_iter().find_map(|element| {
                let element = element.ok()?;
                match element {
                    AnyJsArrayBindingPatternElement::AnyJsBindingPattern(pattern) => {
                        traverse_binding(pattern, tracked_bindings)
                    }
                    AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(
                        binding_rest,
                    ) => {
                        let binding_pattern = binding_rest.pattern().ok()?;
                        traverse_binding(binding_pattern, tracked_bindings)
                    }
                    AnyJsArrayBindingPatternElement::JsArrayHole(_) => None,
                    AnyJsArrayBindingPatternElement::JsBindingPatternWithDefault(
                        binding_with_default,
                    ) => {
                        let pattern = binding_with_default.pattern().ok()?;
                        traverse_binding(pattern, tracked_bindings)
                    }
                }
            })
        }

        AnyJsBindingPattern::JsObjectBindingPattern(pattern) => {
            return pattern.properties().into_iter().find_map(|prop| {
                let prop = prop.ok()?;
                match prop {
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(pattern) => {
                        let pattern = pattern.pattern().ok()?;
                        traverse_binding(pattern, tracked_bindings)
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                        let pattern = rest.binding().ok()?;
                        match pattern {
                            AnyJsBinding::JsIdentifierBinding(binding) => {
                                track_binding(&binding, tracked_bindings).then_some(binding)
                            }
                            AnyJsBinding::JsBogusBinding(_) => None,
                        }
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        shorthand_binding,
                    ) => match shorthand_binding.identifier().ok()? {
                        AnyJsBinding::JsIdentifierBinding(id_binding) => {
                            track_binding(&id_binding, tracked_bindings).then_some(id_binding)
                        }
                        AnyJsBinding::JsBogusBinding(_) => None,
                    },
                    AnyJsObjectBindingPatternMember::JsBogusBinding(_) => None,
                }
            })
        }
    }
    None
}

#[inline]
/// If the name of binding has been seen in set, then we push the `JsIdentifierBinding` into `identifier_vec`.
/// Else we mark the name of binding as seen.
fn track_binding(
    id_binding: &JsIdentifierBinding,
    tracked_bindings: &mut FxHashSet<String>,
) -> bool {
    let binding_text = id_binding.text();
    if tracked_bindings.contains(&binding_text) {
        true
    } else {
        tracked_bindings.insert(binding_text);
        false
    }
}

declare_node_union! {
    /// A union of all possible FunctionLike `JsAstNode` in the JS grammar.
    pub(crate) AnyJsFunctionAndMethod = JsArrowFunctionExpression| JsFunctionDeclaration| JsFunctionExportDefaultDeclaration | JsFunctionExpression | JsMethodClassMember | JsMethodObjectMember
}
