use rome_js_semantic::SemanticModel;
use rome_js_syntax::JsSyntaxKind::JS_IMPORT;
use rome_js_syntax::{JsIdentifierBinding, JsIdentifierExpression, JsStaticMemberExpression};
use rome_rowan::{declare_node_union, AstNode};

declare_node_union! {
    pub(crate) PossibleCreateElement = JsStaticMemberExpression | JsIdentifierExpression
}

/// Checks if the current node is a possible `createElement` call.
///
/// There are two cases:
///
/// First case
/// ```js
/// React.createElement()
/// ```
/// We check if the node is a static member expression with the specific members. Also, if `React`
/// has been imported in the current scope, we make sure that the binding `React` has been imported
/// from the `"react"` module.
///
/// Second case
///
/// ```js
/// createElement()
/// ```
///
/// The logic of this second case is very similar to the previous one, simply the node that we have
/// to inspect is different.
pub(crate) fn is_react_create_element(
    node: PossibleCreateElement,
    model: &SemanticModel,
) -> Option<bool> {
    let result = match node {
        PossibleCreateElement::JsStaticMemberExpression(node) => {
            let object = node.object().ok()?;
            let member = node.member().ok()?;
            let member = member.as_js_name()?;
            let identifier = object.as_js_identifier_expression()?.name().ok()?;

            let maybe_from_react = identifier.syntax().text_trimmed() == "React"
                && member.syntax().text_trimmed() == "createElement";

            if maybe_from_react {
                let identifier_binding = model.declaration(&identifier);
                if let Some(binding_identifier) = identifier_binding {
                    let binding_identifier =
                        JsIdentifierBinding::cast_ref(binding_identifier.syntax())?;
                    for ancestor in binding_identifier.syntax().ancestors() {
                        if ancestor.kind() == JS_IMPORT {
                            return Some(
                                binding_identifier.syntax().text_trimmed()
                                    == identifier.syntax().text_trimmed(),
                            );
                        }
                    }
                }
            }
            maybe_from_react
        }
        PossibleCreateElement::JsIdentifierExpression(identifier) => {
            let name = identifier.name().ok()?;
            let maybe_from_react = identifier.syntax().text_trimmed() == "createElement";
            if maybe_from_react {
                let identifier_binding = model.declaration(&name);
                if let Some(identifier_binding) = identifier_binding {
                    let binding_identifier =
                        JsIdentifierBinding::cast_ref(identifier_binding.syntax())?;
                    for ancestor in binding_identifier.syntax().ancestors() {
                        if ancestor.kind() == JS_IMPORT {
                            return Some(
                                binding_identifier.syntax().text_trimmed()
                                    == identifier.syntax().text_trimmed(),
                            );
                        }
                    }
                }
            }

            maybe_from_react
        }
    };

    Some(result)
}
