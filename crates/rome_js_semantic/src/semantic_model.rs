use rome_js_syntax::{
    JsAnyRoot, JsIdentifierAssignment, JsIdentifierBinding, JsLanguage, JsReferenceIdentifier,
    JsSyntaxNode, TextRange,
};
use rome_rowan::{AstNode, SyntaxTokenText};
use rust_lapper::{Interval, Lapper};
use std::{collections::HashMap, iter::FusedIterator, sync::Arc};

use crate::{SemanticEvent, SemanticEventExtractor};

/// Marker trait that groups all "AstNode" that are declarations
pub trait IsDeclarationAstNode: AstNode<Language = JsLanguage> {
    #[inline(always)]
    fn node(&self) -> &Self {
        self
    }
}

impl IsDeclarationAstNode for JsIdentifierBinding {}

/// Marker trait that groups all "AstNode" that have declarations
pub trait HasDeclarationAstNode: AstNode<Language = JsLanguage> {
    #[inline(always)]
    fn node(&self) -> &Self {
        self
    }
}

impl HasDeclarationAstNode for JsReferenceIdentifier {}
impl HasDeclarationAstNode for JsIdentifierAssignment {}

struct SemanticModelScopeData {
    parent: Option<usize>,
    children: Vec<usize>,
    bindings: Vec<TextRange>,
    bindings_by_name: HashMap<SyntaxTokenText, usize>,
}

/// Contains all the data of the [SemanticModel] and only lives behind an [Arc].
///
/// That allows any returned struct (like [Scope], [Binding])
/// to outlive the [SemanticModel], and to not include lifetimes.
struct SemanticModelData {
    root: JsAnyRoot,
    scopes: Vec<SemanticModelScopeData>,
    scope_by_range: rust_lapper::Lapper<usize, usize>,
    node_by_range: HashMap<TextRange, JsSyntaxNode>,
    // Maps any range in the code to its declaration
    declared_at_by_range: HashMap<TextRange, TextRange>,
    // Maps a declaration range to the range of its references
    declaration_all_references: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    // Maps a declaration range to the range of its "reads"
    declaration_all_reads: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    // Maps a declaration range to the range of its "writes"
    declaration_all_writes: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
}

impl SemanticModelData {
    fn scope(&self, range: &TextRange) -> usize {
        let scopes = self
            .scope_by_range
            .find(range.start().into(), range.end().into());

        match scopes.last() {
            Some(interval) => interval.val,
            // We always have at least one scope, the global one.
            None => unreachable!(),
        }
    }

    pub fn all_references_iter(
        &self,
        range: &TextRange,
    ) -> std::slice::Iter<'_, (ReferenceType, TextRange)> {
        if let Some(v) = self.declaration_all_references.get(range) {
            v.iter()
        } else {
            [].iter()
        }
    }

    pub fn all_reads_iter(
        &self,
        range: &TextRange,
    ) -> std::slice::Iter<'_, (ReferenceType, TextRange)> {
        if let Some(v) = self.declaration_all_reads.get(range) {
            v.iter()
        } else {
            [].iter()
        }
    }

    pub fn all_writes_iter(
        &self,
        range: &TextRange,
    ) -> std::slice::Iter<'_, (ReferenceType, TextRange)> {
        if let Some(v) = self.declaration_all_writes.get(range) {
            v.iter()
        } else {
            [].iter()
        }
    }
}

impl PartialEq for SemanticModelData {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl Eq for SemanticModelData {}

/// Iterator to navigate upwards in the scope tree
pub struct ScopeAncestorsIter {
    current: Option<Scope>,
}

impl Iterator for ScopeAncestorsIter {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current.take() {
            Some(current) => {
                self.current = current.parent();
                Some(current)
            }
            None => None,
        }
    }
}

impl FusedIterator for ScopeAncestorsIter {}

/// Provides all information regarding a specific scope.
/// Allows navigation to parent and children scope and binding information.
#[derive(Clone)]
pub struct Scope {
    data: Arc<SemanticModelData>,
    id: usize,
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.data == other.data
    }
}

impl Eq for Scope {}

impl std::fmt::Debug for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = &self.data.scopes[self.id];
        f.debug_struct("Scope")
            .field("parent", &data.parent)
            .field("children", &data.children)
            .finish()
    }
}

impl Scope {
    /// Returns all parents of this scope. Starting with the current
    /// [Scope].
    pub fn ancestors(&self) -> ScopeAncestorsIter {
        ScopeAncestorsIter {
            current: Some(self.clone()),
        }
    }

    /// Return this scope parent.
    pub fn parent(&self) -> Option<Scope> {
        // id will always be a valid scope because
        // it was created by [SemanticModel::scope] method.
        debug_assert!(self.id < self.data.scopes.len());

        let parent = self.data.scopes[self.id].parent?;
        Some(Scope {
            data: self.data.clone(),
            id: parent,
        })
    }

    /// Return all bindings that were bound in this scope. It **does
    /// not** return bindings of parent scopes.
    pub fn bindings(&self) -> ScopeBindingsIter {
        ScopeBindingsIter {
            data: self.data.clone(),
            scope_id: self.id,
            binding_index: 0,
        }
    }

    /// Return a binding by its name, like it appears on code.
    pub fn get_binding(&self, name: impl AsRef<str>) -> Option<Binding> {
        let name = name.as_ref();
        let data = &self.data.scopes[self.id];

        let i = data.bindings_by_name.get(name)?;
        let range = &data.bindings[*i];
        let node = self.data.node_by_range.get(range)?;

        Some(Binding {
            node: node.clone(),
            data: self.data.clone(),
        })
    }
}

/// Provides all information regarding to a specific binding.
pub struct Binding {
    node: JsSyntaxNode,
    data: Arc<SemanticModelData>,
}

impl Binding {
    /// Returns the scope of this binding
    pub fn scope(&self) -> Scope {
        let range = self.node.text_range();
        let id = self.data.scope(&range);
        Scope {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the syntax node associated with the binding.
    pub fn syntax(&self) -> &JsSyntaxNode {
        &self.node
    }

    /// Returns an iterator to all references of this binding.
    pub fn all_references(&self) -> ReferencesIter {
        let range = self.node.text_range();
        ReferencesIter {
            data: self.data.clone(),
            iter: self.data.all_references_iter(&range),
        }
    }

    /// Returns an iterator to all reads references of this binding.
    pub fn all_reads(&self) -> ReferencesIter {
        let range = self.node.text_range();
        ReferencesIter {
            data: self.data.clone(),
            iter: self.data.all_reads_iter(&range),
        }
    }

    /// Returns an iterator to all write references of this binding.
    pub fn all_writes(&self) -> ReferencesIter {
        let range = self.node.text_range();
        ReferencesIter {
            data: self.data.clone(),
            iter: self.data.all_writes_iter(&range),
        }
    }
}

#[derive(Clone, Copy)]
enum ReferenceType {
    Read { hoisted: bool },
    Write { hoisted: bool },
}

/// Provides all information regarding to a specific reference.
pub struct Reference {
    data: Arc<SemanticModelData>,
    node: JsSyntaxNode,
    range: TextRange,
    ty: ReferenceType,
}

impl Reference {
    /// Returns the scope of this reference
    pub fn scope(&self) -> Scope {
        let id = self.data.scope(&self.range);
        Scope {
            data: self.data.clone(),
            id,
        }
    }

    pub fn node(&self) -> &JsSyntaxNode {
        &self.node
    }

    pub fn declaration(&self) -> Option<Binding> {
        let range = self.data.declared_at_by_range.get(&self.range)?;
        let node = self.data.node_by_range.get(range)?.clone();
        Some(Binding {
            data: self.data.clone(),
            node,
        })
    }

    pub fn is_using_hoisted_declaration(&self) -> bool {
        match self.ty {
            ReferenceType::Read { hoisted } => hoisted,
            ReferenceType::Write { hoisted } => hoisted,
        }
    }

    pub fn is_read(&self) -> bool {
        matches!(self.ty, ReferenceType::Read { .. })
    }

    pub fn is_write(&self) -> bool {
        matches!(self.ty, ReferenceType::Write { .. })
    }
}

/// Iterate all references of a particular declaration.
pub struct ReferencesIter<'a> {
    data: Arc<SemanticModelData>,
    iter: std::slice::Iter<'a, (ReferenceType, TextRange)>,
}

impl<'a> Iterator for ReferencesIter<'a> {
    type Item = Reference;

    fn next(&mut self) -> Option<Self::Item> {
        let (ty, range) = self.iter.next()?;
        let node = self.data.node_by_range.get(range)?;
        Some(Reference {
            data: self.data.clone(),
            node: node.clone(),
            range: *range,
            ty: *ty,
        })
    }
}

impl<'a> ExactSizeIterator for ReferencesIter<'a> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a> FusedIterator for ReferencesIter<'a> {}

/// Iterate all bindings that were bound in a given scope. It **does
/// not** return bindings of parent scopes.
pub struct ScopeBindingsIter {
    data: Arc<SemanticModelData>,
    scope_id: usize,
    binding_index: usize,
}

impl Iterator for ScopeBindingsIter {
    type Item = Binding;

    fn next(&mut self) -> Option<Self::Item> {
        // scope_id will always be a valid scope because
        // it was created by [Scope::bindings] method.
        debug_assert!(self.scope_id < self.data.scopes.len());

        let range = self.data.scopes[self.scope_id]
            .bindings
            .get(self.binding_index)?;
        let node = self.data.node_by_range.get(range)?;

        self.binding_index += 1;

        Some(Binding {
            node: node.clone(),
            data: self.data.clone(),
        })
    }
}

impl ExactSizeIterator for ScopeBindingsIter {
    fn len(&self) -> usize {
        // scope_id will always be a valid scope because
        // it was created by [Scope::bindings] method.
        debug_assert!(self.scope_id < self.data.scopes.len());

        self.data.scopes[self.scope_id].bindings.len()
    }
}

impl FusedIterator for ScopeBindingsIter {}

/// The façade for all semantic information.
/// - Scope: [scope]
/// - Declrations: [declaration]
///
/// See [SemanticModelData] for more information about the internals.
#[derive(Clone)]
pub struct SemanticModel {
    data: Arc<SemanticModelData>,
}

impl SemanticModel {
    fn new(data: SemanticModelData) -> Self {
        Self {
            data: Arc::new(data),
        }
    }

    /// Return the [Scope] which the syntax is part of.
    /// Can also be called from [AstNode]::scope extension method.
    ///
    /// ```rust
    /// use rome_rowan::{AstNode, SyntaxNodeCast};
    /// use rome_js_syntax::{SourceType, JsReferenceIdentifier};
    /// use rome_js_semantic::{semantic_model, SemanticScopeExtensions};
    ///
    /// let r = rome_js_parser::parse("function f(){let a = arguments[0]; let b = a + 1;}", 0, SourceType::js_module());
    /// let model = semantic_model(&r.tree());
    ///
    /// let arguments_reference = r
    ///     .syntax()
    ///     .descendants()
    ///     .filter_map(|x| x.cast::<JsReferenceIdentifier>())
    ///     .find(|x| x.text() == "arguments")
    ///     .unwrap();
    ///
    /// let block_scope = model.scope(&arguments_reference.syntax());
    /// // or
    /// let block_scope = arguments_reference.scope(&model);
    /// ```
    pub fn scope(&self, node: &JsSyntaxNode) -> Scope {
        let range = node.text_range();
        let scopes = self
            .data
            .scope_by_range
            .find(range.start().into(), range.end().into());

        match scopes.last() {
            Some(interval) => Scope {
                data: self.data.clone(),
                id: interval.val,
            },
            // We always have at least one scope, the global one.
            None => unreachable!(),
        }
    }

    /// Return the [Declaration] of a reference.
    /// Can also be called from "declaration" extension method.
    ///
    /// ```rust
    /// use rome_rowan::{AstNode, SyntaxNodeCast};
    /// use rome_js_syntax::{SourceType, JsReferenceIdentifier};
    /// use rome_js_semantic::{semantic_model, DeclarationExtensions};
    ///
    /// let r = rome_js_parser::parse("function f(){let a = arguments[0]; let b = a + 1;}", 0, SourceType::js_module());
    /// let model = semantic_model(&r.tree());
    ///
    /// let arguments_reference = r
    ///     .syntax()
    ///     .descendants()
    ///     .filter_map(|x| x.cast::<JsReferenceIdentifier>())
    ///     .find(|x| x.text() == "arguments")
    ///     .unwrap();
    ///
    /// let arguments_declaration = model.declaration(&arguments_reference);
    /// // or
    /// let arguments_declaration = arguments_reference.declaration(&model);
    /// ```
    pub fn declaration(&self, reference: &impl HasDeclarationAstNode) -> Option<Binding> {
        let reference = reference.node();
        let range = reference.syntax().text_range();
        let declaration_range = self.data.declared_at_by_range.get(&range)?;
        let node = self.data.node_by_range.get(declaration_range)?.clone();
        Some(Binding {
            node,
            data: self.data.clone(),
        })
    }

    /// Return a list with all [Reference] of a declaration.
    /// Can also be called from "all_references" extension method.
    ///
    /// ```rust
    /// use rome_rowan::{AstNode, SyntaxNodeCast};
    /// use rome_js_syntax::{SourceType, JsIdentifierBinding};
    /// use rome_js_semantic::{semantic_model, AllReferencesExtensions};
    ///
    /// let r = rome_js_parser::parse("function f(){let a = arguments[0]; let b = a + 1;}", 0, SourceType::js_module());
    /// let model = semantic_model(&r.tree());
    ///
    /// let a_binding = r
    ///     .syntax()
    ///     .descendants()
    ///     .filter_map(JsIdentifierBinding::cast)
    ///     .find(|x| x.text() == "a")
    ///     .unwrap();
    ///
    /// let all_references = model.all_references(&a_binding);
    /// // or
    /// let all_references = a_binding.all_references(&model);
    /// ```
    pub fn all_references<'a>(
        &'a self,
        declaration: &impl IsDeclarationAstNode,
    ) -> ReferencesIter<'a> {
        let node = declaration.node();
        let range = node.syntax().text_range();
        ReferencesIter {
            data: self.data.clone(),
            iter: self.data.all_references_iter(&range),
        }
    }

    pub fn all_reads<'a>(&'a self, declaration: &impl IsDeclarationAstNode) -> ReferencesIter<'a> {
        let node = declaration.node();
        let range = node.syntax().text_range();
        ReferencesIter {
            data: self.data.clone(),
            iter: self.data.all_reads_iter(&range),
        }
    }

    pub fn all_writes<'a>(&'a self, declaration: &impl IsDeclarationAstNode) -> ReferencesIter<'a> {
        let node = declaration.node();
        let range = node.syntax().text_range();
        ReferencesIter {
            data: self.data.clone(),
            iter: self.data.all_writes_iter(&range),
        }
    }
}

// Extensions

/// Extension method to allow [AstNode] to easily
/// get its [Scope].
pub trait SemanticScopeExtensions {
    /// Return the [Scope] which this object is part of.
    /// See [scope](semantic_model::SemanticModel::scope)
    fn scope(&self, model: &SemanticModel) -> Scope;
}

impl<T: AstNode<Language = JsLanguage>> SemanticScopeExtensions for T {
    fn scope(&self, model: &SemanticModel) -> Scope {
        model.scope(self.syntax())
    }
}

/// Extension method to allow any node that have a declaration to easily
/// get its declaration.
pub trait DeclarationExtensions {
    /// Return the [Binding] that declared the symbol this reference references.
    fn declaration(&self, model: &SemanticModel) -> Option<Binding>
    where
        Self: HasDeclarationAstNode,
    {
        model.declaration(self)
    }
}

impl<T: HasDeclarationAstNode> DeclarationExtensions for T {}

/// Extension method to allow any node that is a declaration to easily
/// get all of its references.
pub trait AllReferencesExtensions {
    fn all_references<'a>(&self, model: &'a SemanticModel) -> ReferencesIter<'a>
    where
        Self: IsDeclarationAstNode,
    {
        model.all_references(self)
    }

    fn all_reads<'a>(&self, model: &'a SemanticModel) -> ReferencesIter<'a>
    where
        Self: IsDeclarationAstNode,
    {
        model.all_reads(self)
    }

    fn all_writes<'a>(&self, model: &'a SemanticModel) -> ReferencesIter<'a>
    where
        Self: IsDeclarationAstNode,
    {
        model.all_writes(self)
    }
}

impl<T: IsDeclarationAstNode> AllReferencesExtensions for T {}

/// Builds the [SemanticModel] consuming [SemanticEvent] and [SyntaxNode].
/// For a good example on how to use it see [semantic_model].
///
/// [SemanticModelBuilder] consumes all the [SemanticEvents] and build all the
/// data necessary to build a [SemanticModelData], that is allocated with an [Arc]
/// and stored inside the [SemanticModel].
pub struct SemanticModelBuilder {
    root: JsAnyRoot,
    scope_stack: Vec<usize>,
    scopes: Vec<SemanticModelScopeData>,
    scope_by_range: Vec<Interval<usize, usize>>,
    node_by_range: HashMap<TextRange, JsSyntaxNode>,
    declarations_by_range: HashMap<TextRange, TextRange>,
    declaration_all_references: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    declaration_all_reads: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    declaration_all_writes: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
}

impl SemanticModelBuilder {
    pub fn new(root: JsAnyRoot) -> Self {
        Self {
            root,
            scope_stack: vec![],
            scopes: vec![],
            scope_by_range: vec![],
            node_by_range: HashMap::new(),
            declarations_by_range: HashMap::new(),
            declaration_all_references: HashMap::new(),
            declaration_all_reads: HashMap::new(),
            declaration_all_writes: HashMap::new(),
        }
    }

    #[inline]
    pub fn push_node(&mut self, node: &JsSyntaxNode) {
        self.node_by_range.insert(node.text_range(), node.clone());
    }

    #[inline]
    pub fn push_event(&mut self, e: SemanticEvent) {
        use SemanticEvent::*;
        match e {
            ScopeStarted { range } => {
                let new_scope_id = self.scopes.len();

                let current_scope_id = match self.scope_stack.last() {
                    Some(&i) => {
                        self.scopes[i].children.push(new_scope_id);
                        Some(i)
                    }
                    None => None,
                };

                self.scopes.push(SemanticModelScopeData {
                    parent: current_scope_id,
                    children: vec![],
                    bindings: vec![],
                    bindings_by_name: HashMap::new(),
                });
                self.scope_by_range.push(Interval {
                    start: range.start().into(),
                    stop: range.end().into(),
                    val: new_scope_id,
                });
                self.scope_stack.push(new_scope_id);
            }
            ScopeEnded { .. } => {
                self.scope_stack.pop();
            }
            DeclarationFound { name, range, .. } => {
                // We must always have one scope, at least, the global one
                debug_assert!(!self.scope_stack.is_empty());

                let scope = &mut self.scopes[*self.scope_stack.last().unwrap()];
                scope.bindings.push(range);

                scope
                    .bindings_by_name
                    .insert(name, scope.bindings.len() - 1);
            }
            Read {
                range,
                declared_at: declaration_at,
            } => {
                self.declarations_by_range.insert(range, declaration_at);
                self.declaration_all_references
                    .entry(declaration_at)
                    .or_default()
                    .push((ReferenceType::Read { hoisted: false }, range));
                self.declaration_all_reads
                    .entry(declaration_at)
                    .or_default()
                    .push((ReferenceType::Read { hoisted: false }, range));
            }
            HoistedRead {
                range,
                declared_at: declaration_at,
            } => {
                self.declarations_by_range.insert(range, declaration_at);
                self.declaration_all_references
                    .entry(declaration_at)
                    .or_default()
                    .push((ReferenceType::Read { hoisted: true }, range));
                self.declaration_all_reads
                    .entry(declaration_at)
                    .or_default()
                    .push((ReferenceType::Read { hoisted: true }, range));
            }
            Write {
                range,
                declared_at: declaration_at,
            } => {
                self.declarations_by_range.insert(range, declaration_at);
                self.declaration_all_references
                    .entry(declaration_at)
                    .or_default()
                    .push((ReferenceType::Write { hoisted: false }, range));
                self.declaration_all_writes
                    .entry(declaration_at)
                    .or_default()
                    .push((ReferenceType::Write { hoisted: false }, range));
            }
            HoistedWrite {
                range,
                declared_at: declaration_at,
            } => {
                self.declarations_by_range.insert(range, declaration_at);
                self.declaration_all_references
                    .entry(declaration_at)
                    .or_default()
                    .push((ReferenceType::Write { hoisted: true }, range));
                self.declaration_all_writes
                    .entry(declaration_at)
                    .or_default()
                    .push((ReferenceType::Write { hoisted: true }, range));
            }
            UnresolvedReference { .. } => {}
        }
    }

    #[inline]
    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            scopes: self.scopes,
            scope_by_range: Lapper::new(self.scope_by_range),
            node_by_range: self.node_by_range,
            declared_at_by_range: self.declarations_by_range,
            declaration_all_references: self.declaration_all_references,
            declaration_all_reads: self.declaration_all_reads,
            declaration_all_writes: self.declaration_all_writes,
        };
        SemanticModel::new(data)
    }
}

/// Build the complete [SemanticModel] of a parsed file.
/// For a push based model to build the [SemanticModel], see [SemanticModelBuilder].
pub fn semantic_model(root: &JsAnyRoot) -> SemanticModel {
    let mut extractor = SemanticEventExtractor::default();
    let mut builder = SemanticModelBuilder::new(root.clone());

    let root = root.syntax();
    for node in root.preorder() {
        match node {
            rome_js_syntax::WalkEvent::Enter(node) => {
                builder.push_node(&node);
                extractor.enter(&node);
            }
            rome_js_syntax::WalkEvent::Leave(node) => extractor.leave(&node),
        }
    }

    while let Some(e) = extractor.pop() {
        builder.push_event(e);
    }

    builder.build()
}

#[cfg(test)]
mod test {
    use super::*;
    use rome_js_syntax::{JsReferenceIdentifier, SourceType};
    use rome_rowan::SyntaxNodeCast;

    #[test]
    pub fn ok_semantic_model() {
        let r = rome_js_parser::parse(
            "function f(){let a = arguments[0]; let b = a + 1; b = 2; console.log(b)}",
            0,
            SourceType::js_module(),
        );
        let model = semantic_model(&r.tree());

        let arguments_reference = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsReferenceIdentifier>())
            .find(|x| x.text() == "arguments")
            .unwrap();

        let b_from_b_equals_2 = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsIdentifierAssignment>())
            .find(|x| x.text() == "b")
            .unwrap();

        // Scope hierarchy  navigation

        let block_scope = arguments_reference.scope(&model);
        let func_scope = block_scope.parent().unwrap();
        let global_scope = func_scope.parent().unwrap();

        assert!(global_scope.parent().is_none());
        assert_eq!(block_scope.ancestors().count(), 3);

        // Scope equality

        assert_eq!(block_scope, block_scope);
        assert_eq!(func_scope, func_scope);
        assert_eq!(global_scope, global_scope);

        assert_ne!(block_scope, func_scope);
        assert_ne!(block_scope, global_scope);

        // Bindings

        // block scope must have two bindings: a and b
        let bindings = block_scope.bindings().collect::<Vec<_>>();
        match bindings.as_slice() {
            [a, b] => {
                assert_eq!("a", a.syntax().text_trimmed());
                assert_eq!("b", b.syntax().text_trimmed());
            }
            _ => {
                panic!("wrong number of bindings");
            }
        }

        // function scope must have one binding: f
        let bindings = func_scope.bindings().collect::<Vec<_>>();
        match bindings.as_slice() {
            [f] => {
                assert_eq!("f", f.syntax().text_trimmed());
            }
            _ => {
                panic!("wrong number of bindings");
            }
        }

        // Binding by name

        let binding = block_scope.get_binding("arguments");
        assert!(binding.is_none());

        let binding = block_scope.get_binding("a").unwrap();
        assert_eq!("a", binding.syntax().text_trimmed());

        // Declaration (from Read reference)

        let arguments_declaration = arguments_reference.declaration(&model);
        assert!(arguments_declaration.is_none());

        let a_from_a_plus_1 = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsReferenceIdentifier>())
            .find(|x| x.text() == "a")
            .unwrap();

        let a_declaration = a_from_a_plus_1.declaration(&model).unwrap();
        assert_eq!("a", a_declaration.syntax().text_trimmed());

        // Declarations (from Write reference)

        let b_declaration = b_from_b_equals_2.declaration(&model).unwrap();
        assert_eq!("b", b_declaration.syntax().text_trimmed());

        // All references

        assert_eq!(1, a_declaration.all_references().count());
        assert_eq!(1, a_declaration.all_reads().count());
        assert!(a_declaration.all_reads().all(|r| r.is_read()));
        assert!(a_declaration.all_writes().all(|r| r.is_write()));

        assert_eq!(2, b_declaration.all_references().count());
        assert_eq!(1, b_declaration.all_reads().count());
        assert_eq!(1, b_declaration.all_writes().count());
        assert!(b_declaration.all_reads().all(|r| r.is_read()));
        assert!(b_declaration.all_writes().all(|r| r.is_write()));
    }
}
