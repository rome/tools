//! A series of AST utilities to work with the React library

use rome_js_semantic::SemanticModel;
use rome_js_syntax::{
    JsAnyCallArgument, JsAnyExpression, JsArrayExpression, JsCallExpression, JsIdentifierBinding,
    JsImport, JsObjectExpression, JsPropertyObjectMember, JsxMemberName, JsxReferenceIdentifier,
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
    pub(crate) children: Option<JsAnyExpression>,
}

impl ReactCreateElementCall {
    /// It scans the current props and returns the property that matches the passed name
    pub(crate) fn find_prop_by_name(&self, prop_name: &str) -> Option<JsPropertyObjectMember> {
        self.props.as_ref().and_then(|props| {
            let members = props.members();
            members.into_iter().find_map(|member| {
                let member = member.ok()?;
                let property = member.as_js_property_object_member()?;
                let property_name = property.name().ok()?;

                let property_name = property_name.as_js_literal_member_name()?;
                if property_name.name().ok()? == prop_name {
                    Some(property.clone())
                } else {
                    None
                }
            })
        })
    }
}

impl ReactCreateElementCall {
    /// It scans the current props and returns the property that matches the passed name
    pub(crate) fn find_prop_by_name(&self, prop_name: &str) -> Option<JsPropertyObjectMember> {
        self.props.as_ref().and_then(|props| {
            let members = props.members();
            members.into_iter().find_map(|member| {
                let member = member.ok()?;
                let property = member.as_js_property_object_member()?;
                let property_name = property.name().ok()?;

                let property_name = property_name.as_js_literal_member_name()?;
                if property_name.name().ok()? == prop_name {
                    Some(property.clone())
                } else {
                    None
                }
            })
        })
    }
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
            let third_argument = iter
                .next()
                .and_then(|argument| argument.ok())
                .and_then(|argument| argument.as_js_any_expression().cloned());

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

/// Checks if the node `JsxMemberName` is a react fragment.
///
/// e.g. `<React.Fragment>` is a fragment, but no `<React.StrictMode>`.
///
/// In case the `React` is a valid reference, the function checks if it is exported from the
/// `"react"` library
pub(crate) fn jsx_member_name_is_react_fragment(
    member_name: &JsxMemberName,
    model: &SemanticModel,
) -> Option<bool> {
    let object = member_name.object().ok()?;
    let member = member_name.member().ok()?;
    let object = object.as_jsx_reference_identifier()?;
    let mut maybe_react_fragment = object.value_token().ok()?.text_trimmed() == "React"
        && member.value_token().ok()?.text_trimmed() == "Fragment";
    if let Some(reference) = model.declaration(object) {
        if let Some(js_import) = reference
            .syntax()
            .ancestors()
            .find_map(|ancestor| JsImport::cast_ref(&ancestor))
        {
            let source_is_react = js_import.source_is("react").ok()?;
            maybe_react_fragment =
                source_is_react && member.value_token().ok()?.text_trimmed() == "Fragment";
        } else {
            // `React.Fragment` is a binding but it doesn't come from the "react" package
            maybe_react_fragment = false;
        }
    }

    Some(maybe_react_fragment)
}

/// Checks if the node `JsxReferenceIdentifier` is a react fragment.
///
/// e.g. `<Fragment>` is a fragment
///
/// In case the `Fragment` is a valid reference, the function checks if it is exported from the
/// `"react"` library
pub(crate) fn jsx_reference_identifier_is_fragment(
    name: &JsxReferenceIdentifier,
    model: &SemanticModel,
) -> Option<bool> {
    let value_token = name.value_token().ok()?;
    let mut maybe_react_fragment = value_token.text_trimmed() == "Fragment";
    if let Some(reference) = model.declaration(name) {
        if let Some(js_import) = reference
            .syntax()
            .ancestors()
            .find_map(|ancestor| JsImport::cast_ref(&ancestor))
        {
            let source_is_react = js_import.source_is("react").ok()?;
            maybe_react_fragment = source_is_react;
        } else {
            // `Fragment` is a binding g but it doesn't come from the "react" package
            maybe_react_fragment = false;
        }
    }

    Some(maybe_react_fragment)
}
