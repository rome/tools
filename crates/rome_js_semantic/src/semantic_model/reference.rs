use rome_js_syntax::{AnyJsIdentifierUsage, JsCallExpression};

use super::*;
use std::sync::Arc;

/// Provides all information regarding to a specific reference.
#[derive(Debug)]
pub struct Reference {
    pub(crate) data: Arc<SemanticModelData>,
    pub(crate) index: ReferenceIndex,
}

impl Reference {
    pub(crate) fn find_next(&self) -> Option<Reference> {
        let reference = self.data.next_reference(self.index)?;
        Some(Reference {
            data: self.data.clone(),
            index: reference.index,
        })
    }

    pub(crate) fn find_next_read(&self) -> Option<Reference> {
        let mut index = self.index;

        while let Some(reference) = self.data.next_reference(index) {
            if reference.is_read() {
                return Some(Reference {
                    data: self.data.clone(),
                    index: reference.index,
                });
            } else {
                index = reference.index;
            }
        }

        None
    }

    pub(crate) fn find_next_write(&self) -> Option<Reference> {
        let mut index = self.index;

        while let Some(reference) = self.data.next_reference(index) {
            if reference.is_write() {
                return Some(Reference {
                    data: self.data.clone(),
                    index: reference.index,
                });
            } else {
                index = reference.index;
            }
        }

        None
    }

    /// Returns the range of this reference
    pub fn range(&self) -> &TextRange {
        let reference = self.data.reference(self.index);
        &reference.range
    }

    /// Returns the scope of this reference
    pub fn scope(&self) -> Scope {
        let id = self.data.scope(self.range());
        Scope {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the node of this reference
    pub fn syntax(&self) -> &JsSyntaxNode {
        &self.data.node_by_range[self.range()]
    }

    /// Returns the binding of this reference
    pub fn binding(&self) -> Option<Binding> {
        Some(Binding {
            data: self.data.clone(),
            index: self.index.binding(),
        })
    }

    /// Returns if the declaration of this reference is hoisted or not
    pub fn is_using_hoisted_declaration(&self) -> bool {
        let reference = &self.data.reference(self.index);
        match reference.ty {
            SemanticModelReferenceType::Read { hoisted } => hoisted,
            SemanticModelReferenceType::Write { hoisted } => hoisted,
        }
    }

    /// Returns if this reference is just reading its binding
    pub fn is_read(&self) -> bool {
        let reference = self.data.reference(self.index);
        matches!(reference.ty, SemanticModelReferenceType::Read { .. })
    }

    /// Returns if this reference is writing its binding
    pub fn is_write(&self) -> bool {
        let reference = self.data.reference(self.index);
        matches!(reference.ty, SemanticModelReferenceType::Write { .. })
    }
}

/// Provides all information regarding to a specific function of method call.
#[derive(Debug)]
pub struct Call {
    pub(crate) data: Arc<SemanticModelData>,
    pub(crate) index: ReferenceIndex,
}

impl Call {
    /// Returns the range of this reference
    pub fn range(&self) -> &TextRange {
        let reference = self.data.reference(self.index);
        &reference.range
    }

    /// Returns the node of this reference
    pub fn syntax(&self) -> &JsSyntaxNode {
        &self.data.node_by_range[self.range()]
    }

    /// Returns the typed AST node of this reference
    pub fn tree(&self) -> JsCallExpression {
        let node = self.syntax();
        let call = node.ancestors().find(|x| {
            !matches!(
                x.kind(),
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER | JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
            )
        });
        debug_assert!(matches!(&call,
            Some(call) if call.kind() == JsSyntaxKind::JS_CALL_EXPRESSION
        ));
        JsCallExpression::unwrap_cast(call.unwrap())
    }
}

#[derive(Debug)]
pub struct SemanticModelUnresolvedReference {
    pub(crate) range: TextRange,
}

#[derive(Debug)]
pub struct UnresolvedReference {
    pub(crate) data: Arc<SemanticModelData>,
    pub(crate) id: usize,
}

impl UnresolvedReference {
    pub fn syntax(&self) -> &JsSyntaxNode {
        let reference = &self.data.unresolved_references[self.id];
        &self.data.node_by_range[&reference.range]
    }

    pub fn tree(&self) -> AnyJsIdentifierUsage {
        AnyJsIdentifierUsage::unwrap_cast(self.syntax().clone())
    }

    pub fn range(&self) -> &TextRange {
        let reference = &self.data.unresolved_references[self.id];
        &reference.range
    }
}

/// Marker trait that groups all "AstNode" that have declarations
pub trait HasDeclarationAstNode: AstNode<Language = JsLanguage> {
    #[inline(always)]
    fn node(&self) -> &Self {
        self
    }
}

impl HasDeclarationAstNode for JsReferenceIdentifier {}
impl HasDeclarationAstNode for JsIdentifierAssignment {}
impl HasDeclarationAstNode for JsxReferenceIdentifier {}

/// Extension method to allow any node that is a declaration to easily
/// get all of its references.
pub trait ReferencesExtensions {
    fn all_references(&self, model: &SemanticModel) -> AllBindingReferencesIter
    where
        Self: IsBindingAstNode,
    {
        model.as_binding(self).all_references()
    }

    fn all_reads(&self, model: &SemanticModel) -> AllBindingReadReferencesIter
    where
        Self: IsBindingAstNode,
    {
        model.as_binding(self).all_reads()
    }

    fn all_writes(&self, model: &SemanticModel) -> AllBindingWriteReferencesIter
    where
        Self: IsBindingAstNode,
    {
        model.as_binding(self).all_writes()
    }
}

impl<T: IsBindingAstNode> ReferencesExtensions for T {}
