use std::sync::Arc;

use rome_js_syntax::{JsLanguage, JsFunctionDeclaration, JsArrowFunctionExpression, JsFunctionExpression};
use rome_rowan::AstNode;
use super::*;

/// Marker trait that groups all "AstNode" that have closure
pub trait HasClosureAstNode: AstNode<Language = JsLanguage> {
    #[inline(always)]
    fn node(&self) -> &Self {
        self
    }
}

impl HasClosureAstNode for JsFunctionDeclaration {}
impl HasClosureAstNode for JsFunctionExpression {}
impl HasClosureAstNode for JsArrowFunctionExpression {}

/// Provides all information regarding a specific closure.
pub struct Closure {
    data: Arc<SemanticModelData>,
    scope_id: usize,
    closure_range: TextRange,
}

impl Closure {
    pub(super) fn from_node(data: Arc<SemanticModelData>, node: &impl HasClosureAstNode) -> Closure {
        let node = node.node();
        let closure_range = node.syntax().text_range();
        let scope_id = data.scope(&closure_range);

        Closure {
            data,
            scope_id,
            closure_range,
        }
    }

    pub(super) fn from_scope(data: Arc<SemanticModelData>, scope_id: usize, range: &TextRange) -> Option<Closure> {
        let node = &data.node_by_range[range];
        match node.kind() {
            JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                Some(Closure {
                    data,
                    scope_id,
                    closure_range: range.clone(),
                })
            }
            _ => None
        }
    }

    // Returns all scopes which captures are considered capture of
    // the current closure
    fn capture_scopes(&self) -> Vec<usize> {
        let scope = &self.data.scopes[self.scope_id];

        let mut scopes = VecDeque::from_iter(scope.children.iter().cloned());
        let mut result = vec![self.scope_id];

        while let Some(id) = scopes.pop_front() {
            let scope = &self.data.scopes[id];
            let node = &self.data.node_by_range[&scope.range];
            match node.kind() {
                JsSyntaxKind::JS_FUNCTION_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {}
                _ => {
                    result.push(id);
                    scopes.extend(scope.children.iter());
                }
            }
        }

        result
    }

    // Visit all relevant captures and find references which declarations
    // live outside the scope of the closure
    fn captures(&self) -> HashSet<ScopeReference> {
        let mut captures = HashSet::new();

        for id in self.capture_scopes() {
            for reference in self.data.scopes[id].read_references.iter() {
                let declaration = self.data.declared_at_by_range[&reference.range];
                if self.closure_range.intersect(declaration).is_none() {
                    captures.insert(reference.clone());
                }
            }
        }

        captures
    }

    // Return all [Reference] this closure captures
    pub fn all_captures(&self) -> impl Iterator<Item = Reference> {
        self.captures()
            .drain()
            .map(|x| Reference {
                data: self.data.clone(),
                node: self.data.node_by_range[&x.range].clone(),
                range: x.range,
                ty: x.ty,
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    // Returns all scopes which are immediate children closures of
    // the current closure
    fn children_scopes(&self) -> Vec<usize> {
        let scope = &self.data.scopes[self.scope_id];
        
        let mut scopes = VecDeque::from_iter(scope.children.iter().cloned());
        let mut result = vec![];

        while let Some(id) = scopes.pop_front() {
            let scope = &self.data.scopes[id];
            let node = &self.data.node_by_range[&scope.range];
            match node.kind() {
                JsSyntaxKind::JS_FUNCTION_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                    result.push(id);
                }
                _ => {
                    scopes.extend(scope.children.iter());
                }
            }
        }

        result
    }

    /// Return all immediate children closures of this closure
    pub fn children(&self) -> Vec<Closure> {
        let mut closures = vec![];

        for scope_id in self.children_scopes() {
            let scope = &self.data.scopes[scope_id];
            closures.push(Closure {
                data: self.data.clone(),
                scope_id,
                closure_range: scope.range,
            })
        }

        closures
    }
}