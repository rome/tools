use std::sync::Arc;

use super::*;
use rome_js_syntax::{
    JsArrowFunctionExpression, JsFunctionDeclaration, JsFunctionExpression, JsLanguage,
};
use rome_rowan::AstNode;

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

pub struct AllCapturesIter {
    data: Arc<SemanticModelData>,
    closure_range: TextRange,
    scopes: Vec<usize>,
    references: Vec<ScopeReference>
}

impl Iterator for AllCapturesIter {
    type Item = Reference;

    fn next(&mut self) -> Option<Self::Item> {
        'references: loop {
            while let Some(reference) = self.references.pop() {
                let declaration = self.data.declared_at_by_range[&reference.range];
                if self.closure_range.intersect(declaration).is_none() {
                    return Some(Reference {
                        data: self.data.clone(),
                        node: self.data.node_by_range[&reference.range].clone(),
                        range: reference.range.clone(),
                        ty: reference.ty,
                    });
                }
            }

            'scopes: while let Some(scope_id) = self.scopes.pop() {
                let scope = &self.data.scopes[scope_id];
                let node = &self.data.node_by_range[&scope.range];

                match node.kind() {
                    JsSyntaxKind::JS_FUNCTION_DECLARATION
                    | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                    | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                        continue 'scopes;
                    }
                    _ => {
                        self.references.clear();
                        self.references.extend(scope.read_references.iter().cloned());
                        self.references.extend(scope.write_references.iter().cloned());
                        self.scopes.extend(scope.children.iter());
                        continue 'references;
                    }
                }
            }
               
            return None;
        }
    }

}

/// Provides all information regarding a specific closure.
pub struct Closure {
    data: Arc<SemanticModelData>,
    scope_id: usize,
    closure_range: TextRange,
}

impl Closure {
    pub(super) fn from_node(
        data: Arc<SemanticModelData>,
        node: &impl HasClosureAstNode,
    ) -> Closure {
        let node = node.node();
        let closure_range = node.syntax().text_range();
        let scope_id = data.scope(&closure_range);

        Closure {
            data,
            scope_id,
            closure_range,
        }
    }

    pub(super) fn from_scope(
        data: Arc<SemanticModelData>,
        scope_id: usize,
        closure_range: &TextRange,
    ) -> Option<Closure> {
        let node = &data.node_by_range[closure_range];
        match node.kind() {
            JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => Some(Closure {
                data,
                scope_id,
                closure_range: *closure_range,
            }),
            _ => None,
        }
    }

    /// Return all [Reference] this closure captures
    pub fn all_captures(&self) -> impl Iterator<Item = Reference> {
        let scope = &self.data.scopes[self.scope_id];

        let mut scopes = Vec::with_capacity(128);
        scopes.extend(scope.children.iter().cloned());

        let mut references =  Vec::with_capacity(128);
        references.extend(scope.read_references.iter().cloned());
        references.extend(scope.write_references.iter().cloned());

        AllCapturesIter {
            data: self.data.clone(),
            closure_range: self.closure_range.clone(),
            scopes,
            references,
        }
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
