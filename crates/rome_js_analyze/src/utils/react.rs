use std::collections::{HashMap, HashSet};

use rome_js_semantic::{Capture, ClosureExtensions, SemanticModel};
use rome_js_syntax::{
    JsAnyExpression, JsArrayBindingPattern, JsArrayBindingPatternElementList, JsCallExpression,
    JsIdentifierBinding, JsVariableDeclarator, TextRange,
};
use rome_rowan::AstNode;
use serde::{Deserialize, Serialize};

pub(crate) struct ReactCallWithDependencyResult {
    pub(crate) function_name_range: TextRange,
    pub(crate) closure_node: Option<JsAnyExpression>,
    pub(crate) dependencies_node: Option<JsAnyExpression>,
}

impl ReactCallWithDependencyResult {
    pub fn all_captures(&self, model: &SemanticModel) -> Vec<Capture> {
        self.closure_node
            .as_ref()
            .and_then(|node| {
                Some(
                    node.as_js_arrow_function_expression()?
                        .closure(model)
                        .all_captures()
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default()
    }

    pub fn all_dependencies(&self) -> Vec<JsAnyExpression> {
        self.dependencies_node
            .as_ref()
            .and_then(|x| {
                Some(
                    x.as_js_array_expression()?
                        .elements()
                        .into_iter()
                        .filter_map(|x| x.ok()?.as_js_any_expression().cloned())
                        .collect::<Vec<_>>(),
                )
            })
            .unwrap_or_default()
    }
}

fn get_call_expression(call: &JsCallExpression, i: usize) -> Option<JsAnyExpression> {
    let args = call.arguments().ok()?;
    let mut args = args.args().into_iter();
    args.nth(i)?.ok()?.as_js_any_expression().cloned()
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ReactHookClosureDependenciesPosition {
    closure: usize,
    dependencies: usize,
}

impl From<(usize, usize)> for ReactHookClosureDependenciesPosition {
    fn from((closure, dependencies): (usize, usize)) -> Self {
        Self {
            closure,
            dependencies,
        }
    }
}

pub(crate) fn react_hook_with_dependency(
    call: &JsCallExpression,
    hooks: &HashMap<String, ReactHookClosureDependenciesPosition>,
) -> Option<ReactCallWithDependencyResult> {
    let name = call
        .callee()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .value_token()
        .ok()?;
    let function_name_range = name.text_trimmed_range();
    let name = name.text_trimmed();

    let hook = hooks.get(name)?;
    let closure_node = get_call_expression(call, hook.closure);
    let dependencies_node = get_call_expression(call, hook.dependencies);

    Some(ReactCallWithDependencyResult {
        function_name_range,
        closure_node,
        dependencies_node,
    })
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReactHookStable {
    hook_name: String,
    index: Option<usize>,
}

impl ReactHookStable {
    pub fn new(hook_name: &str, index: Option<usize>) -> Self {
        Self {
            hook_name: hook_name.into(),
            index,
        }
    }
}

pub fn is_stable_binding(
    binding: &JsIdentifierBinding,
    stables: &HashSet<ReactHookStable>,
) -> bool {
    fn array_binding_declarator_index(
        binding: &JsIdentifierBinding,
    ) -> Option<(JsVariableDeclarator, Option<usize>)> {
        let index = binding.syntax().index() / 2;
        let declarator = binding
            .parent::<JsArrayBindingPatternElementList>()?
            .parent::<JsArrayBindingPattern>()?
            .parent::<JsVariableDeclarator>()?;
        Some((declarator, Some(index)))
    }

    fn assignment_declarator(
        binding: &JsIdentifierBinding,
    ) -> Option<(JsVariableDeclarator, Option<usize>)> {
        let declarator = binding.parent::<JsVariableDeclarator>()?;
        Some((declarator, None))
    }

    array_binding_declarator_index(binding)
        .or_else(|| assignment_declarator(binding))
        .and_then(|(declarator, index)| {
            let hook_name = declarator
                .initializer()?
                .expression()
                .ok()?
                .as_js_call_expression()?
                .callee()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok()?
                .value_token()
                .ok()?
                .token_text();

            let stable = ReactHookStable {
                hook_name: hook_name.to_string(),
                index,
            };

            Some(stables.contains(&stable))
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod test {
    use super::*;
    use rome_js_parser::FileId;
    use rome_js_syntax::SourceType;
    use rome_rowan::SyntaxNodeCast;

    #[test]
    pub fn ok_react_stable_captures() {
        let r = rome_js_parser::parse(
            "const ref = useRef();",
            FileId::zero(),
            SourceType::js_module(),
        );
        let node = r
            .syntax()
            .descendants()
            .filter(|x| x.text_trimmed() == "ref")
            .last()
            .unwrap();
        let set_name = node.cast::<JsIdentifierBinding>().unwrap();

        let stables = HashSet::from_iter([
            ReactHookStable::new("useRef", None),
            ReactHookStable::new("useState", Some(1)),
        ]);

        assert!(is_stable_binding(&set_name, &stables));
    }
}
