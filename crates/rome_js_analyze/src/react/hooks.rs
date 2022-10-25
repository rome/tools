use std::collections::{HashMap, HashSet};

use rome_js_semantic::{Capture, ClosureExtensions, SemanticModel};
use rome_js_syntax::{
    JsAnyExpression, JsArrayBindingPattern, JsArrayBindingPatternElementList, JsCallExpression,
    JsIdentifierBinding, JsVariableDeclarator, TextRange,
};
use rome_rowan::AstNode;
use serde::{Deserialize, Serialize};

/// Return result of [react_hook_with_dependency].
pub(crate) struct ReactCallWithDependencyResult {
    pub(crate) function_name_range: TextRange,
    pub(crate) closure_node: Option<JsAnyExpression>,
    pub(crate) dependencies_node: Option<JsAnyExpression>,
}

impl ReactCallWithDependencyResult {
    /// Returns all [Reference] captured by the closure argument of the React hook.  
    /// See [react_hook_with_dependency].
    pub fn all_captures(&self, model: &SemanticModel) -> impl Iterator<Item = Capture> {
        self.closure_node
            .as_ref()
            .and_then(|node| node.as_js_arrow_function_expression())
            .map(|arrow_function| {
                let closure = arrow_function.closure(model);
                let range = *closure.closure_range();
                closure
                    .descendents()
                    .flat_map(|closure| closure.all_captures())
                    .filter(move |capture| capture.declaration_range().intersect(range).is_none())
            })
            .into_iter()
            .flatten()
    }

    /// Returns all dependencies of a React hook.  
    /// See [react_hook_with_dependency]
    pub fn all_dependencies(&self) -> impl Iterator<Item = JsAnyExpression> {
        self.dependencies_node
            .as_ref()
            .and_then(|x| Some(x.as_js_array_expression()?.elements().into_iter()))
            .into_iter()
            .flatten()
            .filter_map(|x| x.ok()?.as_js_any_expression().cloned())
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ReactHookConfiguration {
    closure_index: usize,
    dependencies_index: usize,
}

impl From<(usize, usize)> for ReactHookConfiguration {
    fn from((closure, dependencies): (usize, usize)) -> Self {
        Self {
            closure_index: closure,
            dependencies_index: dependencies,
        }
    }
}

/// Returns the [TextRange] of the hook name; the node of the
/// expression of the argument that correspond to the closure of
/// the hook; and the node of the dependency list of the hook.
///
/// Example:
/// ```js
/// useEffect(() => {}, []);
///                     ^^ <- dependencies_node
///           ^^^^^^^^ <- closure_node
/// ^^^^^^^^^ <- function_name_range
/// ```
///
/// This function will use the parameter "hooks" with the configuration
/// of all function that are considered hooks. See [ReactHookConfiguration].
pub(crate) fn react_hook_with_dependency(
    call: &JsCallExpression,
    hooks: &HashMap<String, ReactHookConfiguration>,
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

    let mut indices = [hook.closure_index, hook.dependencies_index];
    indices.sort();
    let [closure_node, dependencies_node] = call.get_arguments_by_index(indices);

    Some(ReactCallWithDependencyResult {
        function_name_range,
        closure_node: closure_node.and_then(|x| x.as_js_any_expression().cloned()),
        dependencies_node: dependencies_node.and_then(|x| x.as_js_any_expression().cloned()),
    })
}

/// Specifies which, if any, of the returns of a React hook are stable.    
/// See [is_binding_react_stable].
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StableReactHookConfiguration {
    /// Name of the React hook
    hook_name: String,
    /// Index of the position of the stable return, [None] if
    /// none returns are stable
    index: Option<usize>,
}

impl StableReactHookConfiguration {
    pub fn new(hook_name: &str, index: Option<usize>) -> Self {
        Self {
            hook_name: hook_name.into(),
            index,
        }
    }
}

/// Checks if the binding is bound to a stable React hook
/// return value. Stable returns do not need to be specified
/// as dependencies.
///
/// Example:
/// ```js
/// let [name, setName] = useState("");
/// useEffect(() => {
///     // name is NOT stable, so it must be specified as dependency
///     console.log(name);
///     // setName IS stable, so it must not be specified as dependency
///     console.log(setName("a"));
/// }, [name]);
/// ```
pub fn is_binding_react_stable(
    binding: &JsIdentifierBinding,
    stable_config: &HashSet<StableReactHookConfiguration>,
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

            let stable = StableReactHookConfiguration {
                hook_name: hook_name.to_string(),
                index,
            };

            Some(stable_config.contains(&stable))
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

        let config = HashSet::from_iter([
            StableReactHookConfiguration::new("useRef", None),
            StableReactHookConfiguration::new("useState", Some(1)),
        ]);

        assert!(is_binding_react_stable(&set_name, &config));
    }
}
