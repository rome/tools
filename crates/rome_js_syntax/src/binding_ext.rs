use crate::{
    JsArrowFunctionExpression, JsBogusNamedImportSpecifier, JsBogusParameter, JsCatchDeclaration,
    JsClassDeclaration, JsClassExportDefaultDeclaration, JsClassExpression,
    JsConstructorClassMember, JsConstructorParameterList, JsConstructorParameters,
    JsDefaultImportSpecifier, JsFormalParameter, JsFunctionDeclaration,
    JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsIdentifierBinding,
    JsImportDefaultClause, JsImportNamespaceClause, JsMethodClassMember, JsMethodObjectMember,
    JsNamedImportSpecifier, JsNamespaceImportSpecifier, JsParameterList, JsParameters,
    JsRestParameter, JsSetterClassMember, JsSetterObjectMember, JsShorthandNamedImportSpecifier,
    JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator, TsCallSignatureTypeMember,
    TsConstructSignatureTypeMember, TsConstructorSignatureClassMember, TsConstructorType,
    TsDeclareFunctionDeclaration, TsDeclareFunctionExportDefaultDeclaration, TsEnumDeclaration,
    TsFunctionType, TsIdentifierBinding, TsImportEqualsDeclaration, TsIndexSignatureClassMember,
    TsIndexSignatureParameter, TsInterfaceDeclaration, TsMethodSignatureClassMember,
    TsMethodSignatureTypeMember, TsModuleDeclaration, TsPropertyParameter,
    TsSetterSignatureClassMember, TsSetterSignatureTypeMember, TsTypeAliasDeclaration,
};
use rome_rowan::{declare_node_union, AstNode};

declare_node_union! {
    pub JsAnyBindingDeclaration =
        // variable
            JsVariableDeclarator
        // parameters
            | JsFormalParameter | JsRestParameter | JsBogusParameter
            | TsIndexSignatureParameter | TsPropertyParameter
        // functions
            | JsFunctionDeclaration | JsFunctionExpression
            | TsDeclareFunctionDeclaration
        // classes, objects, interface, type, enum, module
            | JsClassDeclaration | JsClassExpression
            | TsInterfaceDeclaration | TsTypeAliasDeclaration | TsEnumDeclaration | TsModuleDeclaration
        // import
            | JsImportDefaultClause | JsImportNamespaceClause | JsShorthandNamedImportSpecifier
                | JsNamedImportSpecifier | JsBogusNamedImportSpecifier | JsDefaultImportSpecifier
                | JsNamespaceImportSpecifier
            | TsImportEqualsDeclaration
        // export
            | JsClassExportDefaultDeclaration | JsFunctionExportDefaultDeclaration
            | TsDeclareFunctionExportDefaultDeclaration
        // try/catch
            | JsCatchDeclaration
}

declare_node_union! {
    pub JsAnyIdentifierBinding = JsIdentifierBinding | TsIdentifierBinding
}

fn declaration(node: &JsSyntaxNode) -> Option<JsAnyBindingDeclaration> {
    use JsSyntaxKind::*;
    let parent = node.parent()?;
    let possible_declarator = parent.ancestors().find(|x| {
        !matches!(
            x.kind(),
            JS_BINDING_PATTERN_WITH_DEFAULT
                | JS_OBJECT_BINDING_PATTERN
                | JS_OBJECT_BINDING_PATTERN_REST
                | JS_OBJECT_BINDING_PATTERN_PROPERTY
                | JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
                | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
                | JS_ARRAY_BINDING_PATTERN
                | JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
                | JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
        )
    })?;

    JsAnyBindingDeclaration::cast(possible_declarator)
}

fn is_under_pattern_binding(node: &JsSyntaxNode) -> Option<bool> {
    use JsSyntaxKind::*;
    Some(matches!(
        node.parent()?.kind(),
        JS_BINDING_PATTERN_WITH_DEFAULT
            | JS_OBJECT_BINDING_PATTERN
            | JS_OBJECT_BINDING_PATTERN_REST
            | JS_OBJECT_BINDING_PATTERN_PROPERTY
            | JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
            | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
            | JS_ARRAY_BINDING_PATTERN
            | JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
            | JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
    ))
}

fn is_under_array_pattern_binding(node: &JsSyntaxNode) -> Option<bool> {
    use JsSyntaxKind::*;
    let parent = node.parent()?;
    match parent.kind() {
        JS_ARRAY_BINDING_PATTERN
        | JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
        | JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => Some(true),
        JS_BINDING_PATTERN_WITH_DEFAULT => is_under_array_pattern_binding(&parent),
        _ => Some(false),
    }
}

fn is_under_object_pattern_binding(node: &JsSyntaxNode) -> Option<bool> {
    use JsSyntaxKind::*;
    let parent = node.parent()?;
    match parent.kind() {
        JS_OBJECT_BINDING_PATTERN
        | JS_OBJECT_BINDING_PATTERN_REST
        | JS_OBJECT_BINDING_PATTERN_PROPERTY
        | JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
        | JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => Some(true),
        JS_BINDING_PATTERN_WITH_DEFAULT => is_under_object_pattern_binding(&parent),
        _ => Some(false),
    }
}

impl JsAnyIdentifierBinding {
    pub fn declaration(&self) -> Option<JsAnyBindingDeclaration> {
        let node = match self {
            JsAnyIdentifierBinding::JsIdentifierBinding(binding) => &binding.syntax,
            JsAnyIdentifierBinding::TsIdentifierBinding(binding) => &binding.syntax,
        };
        declaration(node)
    }

    pub fn is_under_pattern_binding(&self) -> Option<bool> {
        is_under_pattern_binding(self.syntax())
    }

    pub fn is_under_array_pattern_binding(&self) -> Option<bool> {
        is_under_array_pattern_binding(self.syntax())
    }

    pub fn is_under_object_pattern_binding(&self) -> Option<bool> {
        is_under_object_pattern_binding(self.syntax())
    }
}

impl JsIdentifierBinding {
    /// Navigate upward until the declaration of this binding bypassing all nodes
    /// related to pattern binding.
    pub fn declaration(&self) -> Option<JsAnyBindingDeclaration> {
        declaration(&self.syntax)
    }

    pub fn is_under_pattern_binding(&self) -> Option<bool> {
        is_under_pattern_binding(self.syntax())
    }

    pub fn is_under_array_pattern_binding(&self) -> Option<bool> {
        is_under_array_pattern_binding(self.syntax())
    }

    pub fn is_under_object_pattern_binding(&self) -> Option<bool> {
        is_under_object_pattern_binding(self.syntax())
    }
}

impl TsIdentifierBinding {
    pub fn declaration(&self) -> Option<JsAnyBindingDeclaration> {
        declaration(&self.syntax)
    }

    pub fn is_under_pattern_binding(&self) -> Option<bool> {
        is_under_pattern_binding(self.syntax())
    }

    pub fn is_under_array_pattern_binding(&self) -> Option<bool> {
        is_under_array_pattern_binding(self.syntax())
    }

    pub fn is_under_object_pattern_binding(&self) -> Option<bool> {
        is_under_object_pattern_binding(self.syntax())
    }
}

declare_node_union! {
    pub JsAnyParameterParentFunction =
        JsFunctionDeclaration
        | JsFunctionExpression
        | JsArrowFunctionExpression
        | JsFunctionExportDefaultDeclaration

        | JsConstructorClassMember
        | JsMethodClassMember
        | JsSetterClassMember

        | JsMethodObjectMember
        | JsSetterObjectMember

        | TsFunctionType
        | TsConstructorType

        | TsDeclareFunctionDeclaration
        | TsDeclareFunctionExportDefaultDeclaration

        | TsConstructorSignatureClassMember
        | TsMethodSignatureClassMember
        | TsSetterSignatureClassMember
        | TsIndexSignatureClassMember

        | TsConstructSignatureTypeMember
        | TsMethodSignatureTypeMember
        | TsSetterSignatureTypeMember
        | TsCallSignatureTypeMember
}

fn parent_function(node: &JsSyntaxNode) -> Option<JsAnyParameterParentFunction> {
    let parent = node.parent()?;

    match parent.kind() {
        JsSyntaxKind::JS_PARAMETER_LIST => {
            // SAFETY: kind check above
            let parameters = JsParameterList::unwrap_cast(parent).parent::<JsParameters>()?;
            let parent = parameters.syntax.parent()?;
            JsAnyParameterParentFunction::cast(parent)
        }
        JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST => {
            // SAFETY: kind check above
            let parameters = JsConstructorParameterList::unwrap_cast(parent)
                .parent::<JsConstructorParameters>()?;
            let parent = parameters.syntax().parent()?;
            JsAnyParameterParentFunction::cast(parent)
        }
        _ => JsAnyParameterParentFunction::cast(parent),
    }
}

impl JsFormalParameter {
    pub fn parent_function(&self) -> Option<JsAnyParameterParentFunction> {
        parent_function(&self.syntax)
    }
}

impl JsRestParameter {
    pub fn parent_function(&self) -> Option<JsAnyParameterParentFunction> {
        parent_function(&self.syntax)
    }
}

impl TsPropertyParameter {
    pub fn parent_function(&self) -> Option<JsAnyParameterParentFunction> {
        parent_function(&self.syntax)
    }
}
