use rome_js_syntax::{JsAnyRoot, JsLanguage, JsReferenceIdentifier, JsSyntaxNode, TextRange};
use rome_rowan::{AstNode, SyntaxTokenText};
use rust_lapper::{Interval, Lapper};
use std::{collections::HashMap, iter::FusedIterator, sync::Arc};

use crate::{SemanticEvent, SemanticEventExtractor};

struct SemanticModelScopeData {
    parent: Option<usize>,
    children: Vec<usize>,
    bindings: Vec<TextRange>,
    bindings_by_name: HashMap<SyntaxTokenText, TextRange>,
}

/// Contains all the data of the [SemanticModel] and only lives behind an [Arc].
///
/// That allows any returned struct (like [Scope], [Binding])
/// to outlive the [SemanticModel], and to not include lifetimes.
struct SemanticModelData {
    scopes: Vec<SemanticModelScopeData>,
    scope_by_range: rust_lapper::Lapper<usize, usize>,
    node_by_range: HashMap<TextRange, JsSyntaxNode>,
    declarations_by_range: HashMap<TextRange, TextRange>,
}

/// Provides all information regarding a specific scope.
/// Allows navigation to parent and children scope and binding information.
pub struct Scope {
    data: Arc<SemanticModelData>,
    id: usize,
}

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
    /// Return this scope parent.
    pub fn parent(&self) -> Option<Scope> {
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
    pub fn get_binding(&self, name: &str) -> Option<Binding> {
        let data = &self.data.scopes[self.id];

        let range = data.bindings_by_name.get(name)?;
        let node = self.data.node_by_range.get(range)?;

        Some(Binding { node: node.clone() })
    }
}

/// Provides all information regarding to a specific binding.
pub struct Binding {
    node: JsSyntaxNode,
}

impl Binding {
    /// Returns the syntax node associated with the binding.
    pub fn syntax(&self) -> &JsSyntaxNode {
        &self.node
    }
}

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

        Some(Binding { node: node.clone() })
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
    /// Can also be called from [JsReferenceIdentifier]::declaration extension method.
    ///
    /// ```rust
    /// use rome_rowan::{AstNode, SyntaxNodeCast};
    /// use rome_js_syntax::{SourceType, JsReferenceIdentifier};
    /// use rome_js_semantic::{semantic_model, JsReferenceIdentifierExtensions};
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
    pub fn declaration(&self, reference: &JsReferenceIdentifier) -> Option<Binding> {
        let declaration_range = self
            .data
            .declarations_by_range
            .get(&reference.syntax().text_range())?;
        let node = self.data.node_by_range.get(declaration_range)?.clone();
        Some(Binding { node })
    }
}

// Extensions

pub trait SemanticScopeExtensions {
    fn scope(&self, model: &SemanticModel) -> Scope;
}

impl<T: AstNode<Language = JsLanguage>> SemanticScopeExtensions for T {
    fn scope(&self, model: &SemanticModel) -> Scope {
        model.scope(self.syntax())
    }
}

pub trait JsReferenceIdentifierExtensions {
    fn declaration(&self, model: &SemanticModel) -> Option<Binding>;
}

impl JsReferenceIdentifierExtensions for JsReferenceIdentifier {
    fn declaration(&self, model: &SemanticModel) -> Option<Binding> {
        model.declaration(self)
    }
}

/// Builds the [SemanticModel] consuming [SemanticEvent] and [SyntaxNode].
/// For a good example on how to use it see [semantic_model].
///
/// [SemanticModelBuilder] consumes all the [SemanticEvents] and build all the
/// data necessary to build a [SemanticModelData], that is allocated with an [Arc]
/// and stored inside the [SemanticModel].
#[derive(Default)]
pub struct SemanticModelBuilder {
    scope_stack: Vec<usize>,
    scopes: Vec<SemanticModelScopeData>,
    scope_by_range: Vec<Interval<usize, usize>>,
    node_by_range: HashMap<TextRange, JsSyntaxNode>,
    declarations_by_range: HashMap<TextRange, TextRange>,
}

impl SemanticModelBuilder {
    pub fn push_node(&mut self, node: &JsSyntaxNode) {
        self.node_by_range.insert(node.text_range(), node.clone());
    }

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

                scope.bindings_by_name.insert(name, range);
            }
            Read {
                range,
                declaration_at: Some(declaration_at),
            } => {
                self.declarations_by_range.insert(range, declaration_at);
            }
            HoistedRead {
                range,
                declaration_at,
            } => {
                self.declarations_by_range.insert(range, declaration_at);
            }
            _ => {}
        }
    }

    pub fn build(self) -> SemanticModel {
        let data = SemanticModelData {
            scopes: self.scopes,
            scope_by_range: Lapper::new(self.scope_by_range),
            node_by_range: self.node_by_range,
            declarations_by_range: self.declarations_by_range,
        };
        SemanticModel::new(data)
    }
}

/// Build the complete [SemanticModel] of a parsed file.
/// For a push based model to build the [SemanticModel], see [SemanticModelBuilder].
pub fn semantic_model(root: &JsAnyRoot) -> SemanticModel {
    let mut extractor = SemanticEventExtractor::default();
    let mut sink = SemanticModelBuilder::default();

    let root = root.syntax();
    for node in root.preorder() {
        match node {
            rome_js_syntax::WalkEvent::Enter(node) => {
                sink.push_node(&node);
                extractor.enter(&node);
            }
            rome_js_syntax::WalkEvent::Leave(node) => extractor.leave(&node),
        }
    }

    while let Some(e) = extractor.pop() {
        sink.push_event(e);
    }

    sink.build()
}

#[cfg(test)]
mod test {
    use super::*;
    use rome_js_syntax::{JsReferenceIdentifier, SourceType};
    use rome_rowan::SyntaxNodeCast;

    #[test]
    pub fn ok_semantic_model_events_sink() {
        let r = rome_js_parser::parse(
            "function f(){let a = arguments[0]; let b = a + 1;}",
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

        // Scope Hierarchy

        let block_scope = arguments_reference.scope(&model);
        let func_scope = block_scope.parent().unwrap();
        let global_scope = func_scope.parent().unwrap();

        assert!(global_scope.parent().is_none());

        // Bindings

        let mut bindings = block_scope.bindings();

        let binding0 = bindings.next().unwrap();
        assert_eq!("a", binding0.syntax().text_trimmed());

        let binding1 = bindings.next().unwrap();
        assert_eq!("b", binding1.syntax().text_trimmed());

        assert!(bindings.next().is_none());

        // Binding by name

        let binding = block_scope.get_binding("arguments");
        assert!(binding.is_none());

        let binding = block_scope.get_binding("a").unwrap();
        assert_eq!("a", binding.syntax().text_trimmed());

        // Declarations

        let arguments_declaration = arguments_reference.declaration(&model);
        assert!(arguments_declaration.is_none());

        let a_reference = r
            .syntax()
            .descendants()
            .filter_map(|x| x.cast::<JsReferenceIdentifier>())
            .find(|x| x.text() == "a")
            .unwrap();

        let a_declaration = a_reference.declaration(&model).unwrap();
        assert_eq!("a", a_declaration.syntax().text_trimmed());
    }
}
