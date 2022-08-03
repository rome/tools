use rome_js_syntax::{
    JsAnyFormalParameter, JsAnyParameter, JsAnyRoot, JsFormalParameter, JsLanguage,
    JsParameterList, JsVariableDeclaration, JsVariableDeclarator, JsVariableDeclaratorList,
    JsVariableStatement,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutation};

pub trait JsBatchMutation {
    /// Removes the declarator, and:
    /// 1 - removes the statement if the declaration only has one declarator;
    /// 2 - removes commas around the declarator to keep the list valid.
    fn remove_js_variable_declarator(&mut self, declarator: &JsVariableDeclarator);

    /// Removes the parameter, and:
    /// 1 - removes commas around the parameter to keep the list valid.
    fn remove_js_formal_parameter(&mut self, parameter: &JsFormalParameter);
}

impl JsBatchMutation for BatchMutation<JsLanguage, JsAnyRoot> {
    fn remove_js_variable_declarator(&mut self, declarator: &JsVariableDeclarator) {
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

                Some(())
            });
    }

    fn remove_js_formal_parameter(&mut self, parameter: &JsFormalParameter) {
        parameter.parent::<JsParameterList>().map(|list| {
            let mut elements = list.elements();

            // Find the parameter we want to remove
            // remove its trailing comma, if there is one
            let mut previous_element = None;
            for element in elements.by_ref() {
                if let Ok(node) = element.node() {
                    match node {
                        JsAnyParameter::JsAnyFormalParameter(
                            JsAnyFormalParameter::JsFormalParameter(node),
                        ) if node == parameter => {
                            self.remove_node(node.clone());
                            if let Some(comma) = element.trailing_separator().ok().flatten() {
                                self.remove_token(comma.clone());
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
                        self.remove_token(comma.clone());
                    }
                }
            }

            Some(())
        });
    }
}

#[cfg(test)]
mod tests {
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

    // Remove JsFormalParameter
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
}
