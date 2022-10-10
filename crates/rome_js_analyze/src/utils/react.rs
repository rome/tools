use std::collections::HashSet;

use rome_js_syntax::{JsIdentifierBinding, JsVariableDeclarator, JsArrayBindingPatternElementList, JsArrayBindingPattern};

#[derive(PartialEq, Eq, Hash)]
pub struct ReactHookStable  {
    hook_name: String,
    index: Option<usize>
}

impl ReactHookStable {
    pub fn new(hook_name: &str, index: Option<usize>) -> Self { Self { hook_name: hook_name.into(), index } }
}

fn is_react_hook_state(binding: &JsIdentifierBinding, stables: &HashSet<ReactHookStable>) -> bool {
    fn array_binding_declarator_index(binding: &JsIdentifierBinding) -> Option<(JsVariableDeclarator, Option<usize>)> {
        let index = binding.syntax().index() / 2;
        let declarator = binding.parent::<JsArrayBindingPatternElementList>()?
            .parent::<JsArrayBindingPattern>()?
            .parent::<JsVariableDeclarator>()?;
        Some((declarator, Some(index)))
    }

    fn assignment_declarator(binding: &JsIdentifierBinding) -> Option<(JsVariableDeclarator, Option<usize>)> {
        let declarator = binding.parent::<JsVariableDeclarator>()?;
        Some((declarator, None))
    }
   
    array_binding_declarator_index(binding)
        .or_else(|| assignment_declarator(binding))
        .and_then(|(declarator, index)| {
            let hook_name = declarator.initializer()?
                .expression().ok()?
                .as_js_call_expression()?
                .callee().ok()?
                .as_js_identifier_expression()?
                .name().ok()?
                .value_token().ok()?
                .token_text();
            
            let stable = ReactHookStable {
                hook_name: hook_name.to_string(),
                index: index
            };
            
            Some(stables.contains(&stable))
    }).unwrap_or(false)
}


#[test]
pub fn ok_react_stable_captures() {
    let r = rome_js_parser::parse("const ref = useRef();", FileId::zero(), SourceType::js_module());
    let node = r
        .syntax()
        .descendants()
        .filter(|x| x.text_trimmed() == "ref")
        .last()
        .unwrap();
    let setName = node.cast::<JsIdentifierBinding>().unwrap();

    let stables = HashSet::from_iter([
        ReactHookStable::new("useRef", None),
        ReactHookStable::new("useState", Some(1))
    ]);

    assert!(is_react_hook_state(&setName, &stables));
}
