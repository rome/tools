use rome_js_semantic::SemanticModel;
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsArrayExpression, JsCallExpression, JsIdentifierBinding,
    JsImport, JsObjectExpression,
};
use rome_rowan::{AstNode, AstSeparatedList};

/// A convenient data structure that returns the three arguments of the [React.createElement] call
///
///[React.createElement]: https://reactjs.org/docs/react-api.html#createelement
pub(crate) struct ReactCreateElementCall {
    /// The type of the react element
    pub(crate) element_type: JsAnyCallArgument,
    /// Optional props
    pub(crate) props: Option<JsObjectExpression>,
    /// Optional children
    #[allow(dead_code)]
    pub(crate) children: Option<JsArrayExpression>,
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
    call_expression: &JsCallExpression,
    model: &SemanticModel,
) -> Option<ReactCreateElementCall> {
    let callee = call_expression.callee().ok()?;
    let is_react_create_element = match callee {
        JsAnyExpression::JsStaticMemberExpression(node) => {
            let object = node.object().ok()?;
            let member = node.member().ok()?;
            let member = member.as_js_name()?;
            let identifier = object.as_js_identifier_expression()?.name().ok()?;

            let mut maybe_from_react = identifier.syntax().text_trimmed() == "React"
                && member.syntax().text_trimmed() == "createElement";

            if let Some(binding_identifier) = model.declaration(&identifier) {
                let binding_identifier =
                    JsIdentifierBinding::cast_ref(binding_identifier.syntax())?;
                if let Some(js_import) = binding_identifier
                    .syntax()
                    .ancestors()
                    .find_map(|ancestor| JsImport::cast_ref(&ancestor))
                {
                    maybe_from_react = js_import.source_is("react").ok()?;
                }
            }
            maybe_from_react
        }
        JsAnyExpression::JsIdentifierExpression(identifier) => {
            let name = identifier.name().ok()?;
            let mut maybe_react = identifier.syntax().text_trimmed() == "createElement";
            if let Some(identifier_binding) = model.declaration(&name) {
                let binding_identifier =
                    JsIdentifierBinding::cast_ref(identifier_binding.syntax())?;
                if let Some(js_import) = binding_identifier
                    .syntax()
                    .ancestors()
                    .find_map(|ancestor| JsImport::cast_ref(&ancestor))
                {
                    maybe_react = js_import.source_is("react").ok()?;
                }
            }
            maybe_react
        }
        _ => return None,
    };

    if is_react_create_element {
        let arguments = call_expression.arguments().ok()?.args();
        // React.createElement() should not be processed
        if !arguments.is_empty() {
            let mut iter = arguments.into_iter();
            // SAFETY: protected by the `is_empty` check
            let first_argument = iter.next().unwrap().ok()?;
            let second_argument =
                iter.next()
                    .and_then(|argument| argument.ok())
                    .and_then(|argument| {
                        argument
                            .as_js_any_expression()?
                            .as_js_object_expression()
                            .cloned()
                    });
            let third_argument =
                iter.next()
                    .and_then(|argument| argument.ok())
                    .and_then(|argument| {
                        argument
                            .as_js_any_expression()?
                            .as_js_array_expression()
                            .cloned()
                    });

            Some(ReactCreateElementCall {
                element_type: first_argument,
                props: second_argument,
                children: third_argument,
            })
        } else {
            None
        }
    } else {
        None
    }
}
