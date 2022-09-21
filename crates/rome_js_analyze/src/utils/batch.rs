use rome_js_syntax::{
    JsAnyConstructorParameter, JsAnyFormalParameter, JsAnyParameter, JsConstructorParameterList,
    JsFormalParameter, JsLanguage, JsParameterList, JsSyntaxKind, JsSyntaxNode,
    JsVariableDeclaration, JsVariableDeclarator, JsVariableDeclaratorList, JsVariableStatement,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutation};

pub trait JsBatchMutation {
    /// Removes the declarator, and:
    /// 1 - removes the statement if the declaration only has one declarator;
    /// 2 - removes commas around the declarator to keep the list valid.
    fn remove_js_variable_declarator(&mut self, declarator: &JsVariableDeclarator) -> bool;

    /// Removes the parameter, and:
    /// 1 - removes commas around the parameter to keep the list valid.
    fn remove_js_formal_parameter(&mut self, parameter: &JsFormalParameter) -> bool;
}

fn remove_js_formal_parameter_from_js_parameter_list(
    batch: &mut BatchMutation<JsLanguage>,
    parameter: &JsFormalParameter,
    list: JsSyntaxNode,
) -> Option<bool> {
    let list = JsParameterList::cast(list)?;
    let mut elements = list.elements();

    // Find the parameter we want to remove
    // remove its trailing comma, if there is one
    let mut previous_element = None;
    for element in elements.by_ref() {
        if let Ok(node) = element.node() {
            match node {
                JsAnyParameter::JsAnyFormalParameter(JsAnyFormalParameter::JsFormalParameter(
                    node,
                )) if node == parameter => {
                    batch.remove_node(node.clone());
                    if let Some(comma) = element.trailing_separator().ok().flatten() {
                        batch.remove_token(comma.clone());
                    }
                    break;
                }
                _ => {}
            }
        }
        previous_element = Some(element);
    }

    // if it is the last parameter of the list
    // removes the comma before this element
    if elements.next().is_none() {
        if let Some(element) = previous_element {
            if let Some(comma) = element.trailing_separator().ok().flatten() {
                batch.remove_token(comma.clone());
            }
        }
    }

    Some(true)
}

fn remove_js_formal_parameter_from_js_constructor_parameter_list(
    batch: &mut BatchMutation<JsLanguage>,
    parameter: &JsFormalParameter,
    list: JsSyntaxNode,
) -> Option<bool> {
    let list = JsConstructorParameterList::cast(list)?;
    let mut elements = list.elements();

    // Find the parameter we want to remove
    // remove its trailing comma, if there is one
    let mut previous_element = None;
    for element in elements.by_ref() {
        if let Ok(node) = element.node() {
            match node {
                JsAnyConstructorParameter::JsAnyFormalParameter(
                    JsAnyFormalParameter::JsFormalParameter(node),
                ) if node == parameter => {
                    batch.remove_node(node.clone());
                    if let Some(comma) = element.trailing_separator().ok().flatten() {
                        batch.remove_token(comma.clone());
                    }
                    break;
                }
                _ => {}
            }
        }
        previous_element = Some(element);
    }

    // if it is the last parameter of the list
    // removes the comma before this element
    if elements.next().is_none() {
        if let Some(element) = previous_element {
            if let Some(comma) = element.trailing_separator().ok().flatten() {
                batch.remove_token(comma.clone());
            }
        }
    }

    Some(true)
}

impl JsBatchMutation for BatchMutation<JsLanguage> {
    fn remove_js_variable_declarator(&mut self, declarator: &JsVariableDeclarator) -> bool {
        declarator
            .parent::<JsVariableDeclaratorList>()
            .and_then(|list| {
                let declaration = list.parent::<JsVariableDeclaration>()?;

                if list.syntax_list().len() == 1 {
                    let statement = declaration.parent::<JsVariableStatement>()?;
                    self.remove_node(statement);
                } else {
                    let mut elements = list.elements();

                    // Find the declarator we want to remove
                    // remove its trailing comma, if there is one
                    let mut previous_element = None;
                    for element in elements.by_ref() {
                        if let Ok(node) = element.node() {
                            if node == declarator {
                                self.remove_node(node.clone());
                                if let Some(comma) = element.trailing_separator().ok().flatten() {
                                    self.remove_token(comma.clone());
                                }
                                break;
                            }
                        }
                        previous_element = Some(element);
                    }

                    // if it is the last declarator of the list
                    // removes the comma before this element
                    let remove_previous_element_comma = match elements.next() {
                        Some(e) if e.node().is_err() => true,
                        None => true,
                        _ => false,
                    };

                    if remove_previous_element_comma {
                        if let Some(element) = previous_element {
                            if let Some(comma) = element.trailing_separator().ok().flatten() {
                                self.remove_token(comma.clone());
                            }
                        }
                    }
                }

                Some(true)
            })
            .unwrap_or(false)
    }

    fn remove_js_formal_parameter(&mut self, parameter: &JsFormalParameter) -> bool {
        parameter
            .syntax()
            .parent()
            .and_then(|parent| match parent.kind() {
                JsSyntaxKind::JS_PARAMETER_LIST => {
                    remove_js_formal_parameter_from_js_parameter_list(self, parameter, parent)
                }
                JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST => {
                    remove_js_formal_parameter_from_js_constructor_parameter_list(
                        self, parameter, parent,
                    )
                }
                _ => None,
            })
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use rome_js_factory::make;
    use rome_js_parser::parse;
    use rome_js_syntax::{JsAnyExpression, JsLogicalExpression, SourceType, T};
    use rome_rowan::{BatchMutationExt, SyntaxNodeCast};

    use crate::assert_remove_ok;

    // Remove JsVariableDeclarator
    assert_remove_ok! {
        ok_remove_variable_declarator_single,
            "let a;",
            "",
        ok_remove_variable_declarator_fist,
            "let a, b;",
            "let b;",
        ok_remove_variable_declarator_second,
            "let b, a;",
            "let b;",
        ok_remove_variable_declarator_second_trailling_comma,
            "let b, a,;",
            "let b;",
        ok_remove_variable_declarator_middle,
            "let b, a, c;",
            "let b, c;",
    }

    // Remove JsFormalParameter from functions
    assert_remove_ok! {
        ok_remove_formal_parameter_single,
            "function f(a) {}",
            "function f() {}",
        ok_remove_formal_parameter_first,
            "function f(a, b) {}",
            "function f(b) {}",
        ok_remove_formal_parameter_second,
            "function f(b, a) {}",
            "function f(b) {}",
        ok_remove_formal_parameter_second_trailing_comma,
            "function f(b, a,) {}",
            "function f(b) {}",
        ok_remove_formal_parameter_middle,
            "function f(b, a, c) {}",
            "function f(b, c) {}",
    }

    // Remove JsFormalParameter from class constructors
    assert_remove_ok! {
        ok_remove_formal_parameter_from_class_constructor_single,
            "class A { constructor(a) {} }",
            "class A { constructor() {} }",
        ok_remove_formal_parameter_from_class_constructor_first,
            "class A { constructor(a, b) {} }",
            "class A { constructor(b) {} }",
        ok_remove_formal_parameter_from_class_constructor_second,
            "class A { constructor(b, a) {} }",
            "class A { constructor(b) {} }",
        ok_remove_formal_parameter_from_class_constructor_second_trailing_comma,
            "class A { constructor(b, a,) {} }",
            "class A { constructor(b) {} }",
        ok_remove_formal_parameter_from_class_constructor_middle,
            "class A { constructor(b, a, c) {} }",
            "class A { constructor(b, c) {} }",
    }

    #[test]
    pub fn ok_batch_mutation_multiple_changes_same_node() {
        let r = parse(
            "if (/*A*/a && a.b && a.b.c/*B*/) {}",
            0.into(),
            SourceType::js_module(),
        );

        let exprs: Vec<JsLogicalExpression> = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsLogicalExpression>())
            .collect();

        let mut batch = r.tree().begin();

        let right = exprs[0].right().ok().unwrap();
        let right = right.as_js_static_member_expression().unwrap().clone();

        batch.replace_node(
            JsAnyExpression::JsLogicalExpression(exprs[0].clone()),
            JsAnyExpression::JsStaticMemberExpression(right.clone()),
        );

        batch.replace_token(right.operator_token().unwrap(), make::token(T![?.]));

        let obj = right.object().unwrap();
        batch.replace_token(
            obj.as_js_static_member_expression()
                .unwrap()
                .operator_token()
                .unwrap(),
            make::token(T![?.]),
        );

        let (root, changes) = batch.run();
        dbg!(changes);
        let after = root.to_string();
        assert_eq!("if (/*A*/a?.b?.c/*B*/) {}", after.as_str());
    }
}
