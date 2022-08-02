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
                    let is_last = elements.next().is_none();
                    if is_last {
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
