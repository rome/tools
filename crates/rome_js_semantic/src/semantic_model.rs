use rome_js_syntax::{
    JsAnyRoot, JsIdentifierAssignment, JsIdentifierBinding, JsLanguage, JsReferenceIdentifier,
    JsSyntaxNode, TextRange, TextSize, TsIdentifierBinding,
};
use rome_rowan::{AstNode, SyntaxTokenText};
use rust_lapper::{Interval, Lapper};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    iter::FusedIterator,
    sync::Arc,
};

use crate::{SemanticEvent, SemanticEventExtractor};

/// Marker trait that groups all "AstNode" that are declarations
pub trait IsDeclarationAstNode: AstNode<Language = JsLanguage> {
    #[inline(always)]
    fn node(&self) -> &Self {
        self
    }
}

impl IsDeclarationAstNode for JsIdentifierBinding {}
impl IsDeclarationAstNode for TsIdentifierBinding {}

/// Marker trait that groups all "AstNode" that have declarations
pub trait HasDeclarationAstNode: AstNode<Language = JsLanguage> {
    #[inline(always)]
    fn node(&self) -> &Self {
        self
    }
}

impl HasDeclarationAstNode for JsReferenceIdentifier {}
impl HasDeclarationAstNode for JsIdentifierAssignment {}

/// Marker trait that groups all "AstNode" that can be exported
pub trait IsExportedCanBeQueried: AstNode<Language = JsLanguage> {
    type Result;
    fn is_exported(&self, model: &SemanticModel) -> Self::Result;
}

impl IsExportedCanBeQueried for JsIdentifierBinding {
    type Result = bool;

    fn is_exported(&self, model: &SemanticModel) -> Self::Result {
        let range = self.syntax().text_range();
        model.data.is_exported(range)
    }
}

impl IsExportedCanBeQueried for TsIdentifierBinding {
    type Result = bool;

    fn is_exported(&self, model: &SemanticModel) -> Self::Result {
        let range = self.syntax().text_range();
        model.data.is_exported(range)
    }
}

impl<T: HasDeclarationAstNode> IsExportedCanBeQueried for T {
    type Result = Option<bool>;

    fn is_exported(&self, model: &SemanticModel) -> Self::Result {
        let range = self.declaration(model)?.syntax().text_range();
        Some(model.data.is_exported(range))
    }
}

#[derive(Debug)]
struct SemanticModelScopeData {
    range: TextRange,
    parent: Option<usize>,
    children: Vec<usize>,
    bindings: Vec<TextRange>,
    bindings_by_name: HashMap<SyntaxTokenText, usize>,
}

/// Contains all the data of the [SemanticModel] and only lives behind an [Arc].
///
/// That allows any returned struct (like [Scope], [Binding])
/// to outlive the [SemanticModel], and to not include lifetimes.
#[derive(Debug)]
struct SemanticModelData {
    root: JsAnyRoot,
    scopes: Vec<SemanticModelScopeData>,
    scope_by_range: rust_lapper::Lapper<usize, usize>,
    // Maps the start of a node range to a scope id
    scope_hoisted_to_by_range: HashMap<TextSize, usize>,
    node_by_range: HashMap<TextRange, JsSyntaxNode>,
    // Maps any range in the code to its declaration
    declared_at_by_range: HashMap<TextRange, TextRange>,
    // Maps a declaration range to the range of its references
    declaration_all_references: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    // Maps a declaration range to the range of its "reads"
    declaration_all_reads: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    // Maps a declaration range to the range of its "writes"
    declaration_all_writes: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    // All bindings that were exported
    exported: HashSet<TextRange>,
}

impl SemanticModelData {
    fn scope(&self, range: &TextRange) -> usize {
        let scopes = self
            .scope_by_range
            .find(range.start().into(), range.end().into());

        // We always want the most tight scope
        match scopes.map(|x| x.val).max() {
            Some(val) => val,
            // We always have at least one scope, the global one.
            None => unreachable!("Expected global scope not present"),
        }
    }

    fn scope_hoisted_to(&self, range: &TextRange) -> Option<usize> {
        self.scope_hoisted_to_by_range.get(&range.start()).cloned()
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

    pub fn is_exported(&self, range: TextRange) -> bool {
        self.exported.contains(&range)
    }
}

impl PartialEq for SemanticModelData {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

impl Eq for SemanticModelData {}

/// Provides all information regarding a specific scope.
/// Allows navigation to parent and children scope and binding information.
#[derive(Clone, Debug)]
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

impl Scope {
    /// Returns all parents of this scope. Starting with the current
    /// [Scope].
    pub fn ancestors(&self) -> impl Iterator<Item = Scope> {
        std::iter::successors(Some(self.clone()), |scope| scope.parent())
    }

    /// Returns this scope parent.
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

    /// Returns all bindings that were bound in this scope. It **does
    /// not** returns bindings of parent scopes.
    pub fn bindings(&self) -> ScopeBindingsIter {
        ScopeBindingsIter {
            data: self.data.clone(),
            scope_id: self.id,
            binding_index: 0,
        }
    }

    /// Returns a binding by its name, like it appears on code.  It **does
    /// not** returns bindings of parent scopes.
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

    /// Checks if the current scope is one of the ancestor of "other". Given
    /// that [ancestors] return "self" as the first scope,
    /// this function returns true for:
    ///
    /// ```rust,ignore
    /// assert!(scope.is_ancestor_of(scope));
    /// ```
    pub fn is_ancestor_of(&self, other: &Scope) -> bool {
        other.ancestors().any(|s| s == *self)
    }

    pub fn range(&self) -> &TextRange {
        &self.data.scopes[self.id].range
    }

    pub fn syntax(&self) -> &JsSyntaxNode {
        &self.data.node_by_range[self.range()]
    }
}

/// Provides all information regarding to a specific binding.
#[derive(Debug)]
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

#[derive(Clone, Copy, Debug)]
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
/// not** Returns bindings of parent scopes.
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

    /// Iterate all scopes
    pub fn scopes(&self) -> impl Iterator<Item = Scope> + '_ {
        self.data.scopes.iter().enumerate().map(|(id, _)| Scope {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns the global scope of the model
    pub fn global_scope(&self) -> Scope {
        Scope {
            data: self.data.clone(),
            id: 0,
        }
    }

    /// Returns the [Scope] which the syntax is part of.
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
        let id = self.data.scope(&range);
        Scope {
            data: self.data.clone(),
            id,
        }
    }

    /// Returns the [Scope] which the specified syntax node was hoisted to, if any.
    /// Can also be called from [AstNode]::scope_hoisted_to extension method.
    pub fn scope_hoisted_to(&self, node: &JsSyntaxNode) -> Option<Scope> {
        let range = node.text_range();
        let id = self.data.scope_hoisted_to(&range)?;
        Some(Scope {
            data: self.data.clone(),
            id,
        })
    }

    /// Returns the [Declaration] of a reference.
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

    /// Returns a list with all [Reference] of the specified declaration.
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

    /// Returns a list with all read [Reference] of the specified declaration.
    /// Can also be called from "all_reads" extension method.
    pub fn all_reads<'a>(&'a self, declaration: &impl IsDeclarationAstNode) -> ReferencesIter<'a> {
        let node = declaration.node();
        let range = node.syntax().text_range();
        ReferencesIter {
            data: self.data.clone(),
            iter: self.data.all_reads_iter(&range),
        }
    }

    /// Returns a list with all write [Reference] of the specified declaration.
    /// Can also be called from "all_writes" extension method.
    pub fn all_writes<'a>(&'a self, declaration: &impl IsDeclarationAstNode) -> ReferencesIter<'a> {
        let node = declaration.node();
        let range = node.syntax().text_range();
        ReferencesIter {
            data: self.data.clone(),
            iter: self.data.all_writes_iter(&range),
        }
    }

    /// Returns if the node is exported or is a reference to a binding
    /// that is exported.
    ///
    /// When a binding is specified this method returns a bool.
    ///
    /// When a reference is specified this method returns Option<bool>,
    /// because there is no guarantee that the corresponding declaration exists.
    pub fn is_exported<T>(&self, node: &T) -> T::Result
    where
        T: IsExportedCanBeQueried,
    {
        node.is_exported(self)
    }
}

// Extensions

/// Extension method to allow [AstNode] to easily
/// get its [Scope].
pub trait SemanticScopeExtensions {
    /// Returns the [Scope] which this object is part of.
    /// See [scope](semantic_model::SemanticModel::scope)
    fn scope(&self, model: &SemanticModel) -> Scope;

    /// Returns the [Scope] which this object was hosted to, if any.
    /// See [scope](semantic_model::SemanticModel::scope_hoisted_to)
    fn scope_hoisted_to(&self, model: &SemanticModel) -> Option<Scope>;
}

impl<T: AstNode<Language = JsLanguage>> SemanticScopeExtensions for T {
    fn scope(&self, model: &SemanticModel) -> Scope {
        model.scope(self.syntax())
    }

    fn scope_hoisted_to(&self, model: &SemanticModel) -> Option<Scope> {
        model.scope_hoisted_to(self.syntax())
    }
}

/// Extension method to allow any node that have a declaration to easily
/// get its declaration.
pub trait DeclarationExtensions {
    /// Returns the [Binding] that declared the symbol this reference references.
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
    scopes: Vec<SemanticModelScopeData>,
    scope_range_by_start: HashMap<TextSize, BTreeSet<Interval<usize, usize>>>,
    scope_hoisted_to_by_range: HashMap<TextSize, usize>,
    node_by_range: HashMap<TextRange, JsSyntaxNode>,
    declarations_by_range: HashMap<TextRange, TextRange>,
    declaration_all_references: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    declaration_all_reads: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    declaration_all_writes: HashMap<TextRange, Vec<(ReferenceType, TextRange)>>,
    exported: HashSet<TextRange>,
}

impl SemanticModelBuilder {
    pub fn new(root: JsAnyRoot) -> Self {
        Self {
            root,
            scopes: vec![],
            scope_range_by_start: HashMap::new(),
            scope_hoisted_to_by_range: HashMap::new(),
            node_by_range: HashMap::new(),
            declarations_by_range: HashMap::new(),
            declaration_all_references: HashMap::new(),
            declaration_all_reads: HashMap::new(),
            declaration_all_writes: HashMap::new(),
            exported: HashSet::new(),
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
            ScopeStarted {
                range,
                parent_scope_id,
                scope_id,
            } => {
                // Scopes will be raised in order
                debug_assert!(scope_id == self.scopes.len());

                self.scopes.push(SemanticModelScopeData {
                    range,
                    parent: parent_scope_id,
                    children: vec![],
                    bindings: vec![],
                    bindings_by_name: HashMap::new(),
                });

                if let Some(parent_scope_id) = parent_scope_id {
                    self.scopes[parent_scope_id].children.push(scope_id);
                }

                let start = range.start();
                self.scope_range_by_start
                    .entry(start)
                    .or_default()
                    .insert(Interval {
                        start: start.into(),
                        stop: range.end().into(),
                        val: scope_id,
                    });
            }
            ScopeEnded { .. } => {}
            DeclarationFound {
                name,
                range,
                scope_id,
                hoisted_scope_id,
                ..
            } => {
                let binding_scope_id = hoisted_scope_id.unwrap_or(scope_id);

                // SAFETY: this scope id is guaranteed to exist because they were generated by the
                // event extractor
                debug_assert!(binding_scope_id < self.scopes.len());
                let scope = self.scopes.get_mut(binding_scope_id).unwrap();

                scope.bindings.push(range);
                scope
                    .bindings_by_name
                    .insert(name, scope.bindings.len() - 1);

                if let Some(hoisted_scope_id) = hoisted_scope_id {
                    self.scope_hoisted_to_by_range
                        .insert(range.start(), hoisted_scope_id);
                }
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
            Exported { range } => {
                self.exported.insert(range);
            }
        }
    }

    #[inline]
    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            root: self.root,
            scopes: self.scopes,
            scope_by_range: Lapper::new(
                self.scope_range_by_start
                    .iter()
                    .flat_map(|(_, scopes)| scopes.iter())
                    .cloned()
                    .collect(),
            ),
            scope_hoisted_to_by_range: self.scope_hoisted_to_by_range,
            node_by_range: self.node_by_range,
            declared_at_by_range: self.declarations_by_range,
            declaration_all_references: self.declaration_all_references,
            declaration_all_reads: self.declaration_all_reads,
            declaration_all_writes: self.declaration_all_writes,
            exported: self.exported,
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
    use rome_js_syntax::{JsReferenceIdentifier, JsSyntaxKind, SourceType, TsIdentifierBinding};
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
        assert_eq!(global_scope, model.global_scope());
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

        // function scope must have zero bindings
        // "f" was actually hoisted to the global scope
        let mut bindings = func_scope.bindings();
        assert!(bindings.next().is_none());
        assert!(global_scope.get_binding("f").is_some());

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

    #[test]
    pub fn ok_semantic_model_function_scope() {
        let r = rome_js_parser::parse(
            "function f() {} function g() {}",
            0,
            SourceType::js_module(),
        );
        let model = semantic_model(&r.tree());

        let function_f = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsIdentifierBinding>())
            .find(|x| x.text() == "f")
            .unwrap();

        let function_g = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsIdentifierBinding>())
            .find(|x| x.text() == "g")
            .unwrap();

        // "f" and "g" tokens are not in the same scope, because
        // the keyword "function" starts a new scope
        // but they are both hoisted to the same scope
        assert_ne!(function_f.scope(&model), function_g.scope(&model));
        assert_eq!(
            function_f.scope_hoisted_to(&model),
            function_g.scope_hoisted_to(&model)
        );

        // they are hoisted to the global scope
        let global_scope = model.global_scope();
        assert_eq!(function_f.scope_hoisted_to(&model).unwrap(), global_scope);
        assert_eq!(function_g.scope_hoisted_to(&model).unwrap(), global_scope);

        // And we can find their binding inside the global scope
        assert!(global_scope.get_binding("g").is_some());
        assert!(global_scope.get_binding("f").is_some());
    }

    /// Finds the last time a token named "name" is used and see if its node is marked as exported
    fn assert_is_exported(is_exported: bool, name: &str, code: &str) {
        let r = rome_js_parser::parse(code, 0, SourceType::tsx());
        let model = semantic_model(&r.tree());

        let node = r
            .syntax()
            .descendants()
            .filter(|x| x.text_trimmed() == name)
            .last()
            .unwrap();

        match node.kind() {
            JsSyntaxKind::JS_IDENTIFIER_BINDING => {
                let binding = JsIdentifierBinding::cast(node).unwrap();
                // These do the same thing, but with different APIs
                assert!(
                    is_exported == model.is_exported(&binding),
                    "at \"{}\"",
                    code
                );
                assert!(
                    is_exported == binding.is_exported(&model),
                    "at \"{}\"",
                    code
                );
            }
            JsSyntaxKind::TS_IDENTIFIER_BINDING => {
                let binding = TsIdentifierBinding::cast(node).unwrap();
                // These do the same thing, but with different APIs
                assert!(
                    is_exported == model.is_exported(&binding),
                    "at \"{}\"",
                    code
                );
                assert!(
                    is_exported == binding.is_exported(&model),
                    "at \"{}\"",
                    code
                );
            }
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                let reference = JsReferenceIdentifier::cast(node).unwrap();
                // These do the same thing, but with different APIs
                assert!(
                    is_exported == model.is_exported(&reference).unwrap(),
                    "at \"{}\"",
                    code
                );
                assert!(
                    is_exported == reference.is_exported(&model).unwrap(),
                    "at \"{}\"",
                    code
                );
            }
            x => {
                panic!("This node cannot be exported! {:?}", x);
            }
        };
    }

    #[test]
    pub fn ok_semantic_model_is_exported() {
        // Variables
        assert_is_exported(false, "A", "const A = 1");
        assert_is_exported(true, "A", "export const A = 1");
        assert_is_exported(true, "A", "const A = 1; export default A");
        assert_is_exported(true, "A", "const A = 1; export {A}");
        assert_is_exported(true, "A", "const A = 1; module.exports = A;");
        assert_is_exported(true, "A", "const A = 1; module.exports = {A};");
        assert_is_exported(true, "A", "const A = 1; exports = A;");
        assert_is_exported(true, "A", "const A = 1; exports.A = A;");

        // Functions
        assert_is_exported(false, "f", "function f() {}");
        assert_is_exported(true, "f", "export function f() {}");
        assert_is_exported(true, "f", "export default function f() {}");
        assert_is_exported(true, "f", "function f() {} export default f");
        assert_is_exported(true, "f", "function f() {} export {f}");
        assert_is_exported(true, "f", "function f() {} export {f as g}");
        assert_is_exported(true, "f", "module.exports = function f() {}");
        assert_is_exported(true, "f", "exports = function f() {}");
        assert_is_exported(true, "f", "exports.f = function f() {}");
        assert_is_exported(true, "f", "function f() {} module.exports = f");
        assert_is_exported(true, "f", "function f() {} module.exports = {f}");
        assert_is_exported(true, "f", "function f() {} exports = f");
        assert_is_exported(true, "f", "function f() {} exports.f = f");

        // Classess
        assert_is_exported(false, "A", "class A{}");
        assert_is_exported(true, "A", "export class A{}");
        assert_is_exported(true, "A", "export default class A{}");
        assert_is_exported(true, "A", "class A{} export default A");
        assert_is_exported(true, "A", "class A{} export {A}");
        assert_is_exported(true, "A", "class A{} export {A as B}");
        assert_is_exported(true, "A", "module.exports = class A{}");
        assert_is_exported(true, "A", "exports = class A{}");
        assert_is_exported(true, "A", "class A{} module.exports = A");
        assert_is_exported(true, "A", "class A{} exports = A");
        assert_is_exported(true, "A", "class A{} exports.A = A");

        // Interfaces
        assert_is_exported(false, "A", "interface A{}");
        assert_is_exported(true, "A", "export interface A{}");
        assert_is_exported(true, "A", "export default interface A{}");
        assert_is_exported(true, "A", "interface A{} export default A");
        assert_is_exported(true, "A", "interface A{} export {A}");
        assert_is_exported(true, "A", "interface A{} export {A as B}");
        assert_is_exported(true, "A", "interface A{} module.exports = A");
        assert_is_exported(true, "A", "interface A{} exports = A");
        assert_is_exported(true, "A", "interface A{} exports.A = A");

        // Type Aliases
        assert_is_exported(false, "A", "type A = number;");
        assert_is_exported(true, "A", "export type A = number;");
        assert_is_exported(true, "A", "type A = number; export default A");
        assert_is_exported(true, "A", "type A = number; export {A}");
        assert_is_exported(true, "A", "type A = number; export {A as B}");
        assert_is_exported(true, "A", "type A = number; module.exports = A");
        assert_is_exported(true, "A", "type A = number; exports = A");
        assert_is_exported(true, "A", "type A = number; exports.A = A");

        // Enums
        assert_is_exported(false, "A", "enum A {};");
        assert_is_exported(true, "A", "export enum A {};");
        assert_is_exported(true, "A", "enum A {}; export default A");
        assert_is_exported(true, "A", "enum A {}; export {A}");
        assert_is_exported(true, "A", "enum A {}; export {A as B}");
        assert_is_exported(true, "A", "enum A {}; module.exports = A");
        assert_is_exported(true, "A", "enum A {}; exports = A");
        assert_is_exported(true, "A", "enum A {}; exports.A = A");
    }
}
