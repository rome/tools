use std::sync::Arc;

use super::*;
use rome_js_syntax::{
    JsArrowFunctionExpression, JsConstructorClassMember, JsFunctionDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsLanguage, JsMethodClassMember,
    JsSetterClassMember,
};
use rome_rowan::{AstNode, SyntaxNode, SyntaxNodeCast};

/// Marker trait that groups all "AstNode" that have closure
pub trait HasClosureAstNode {
    fn node_text_range(&self) -> TextRange;
}

impl HasClosureAstNode for JsFunctionDeclaration {
    #[inline(always)]
    fn node_text_range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

impl HasClosureAstNode for JsFunctionExpression {
    #[inline(always)]
    fn node_text_range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

impl HasClosureAstNode for JsArrowFunctionExpression {
    #[inline(always)]
    fn node_text_range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

impl HasClosureAstNode for JsConstructorClassMember {
    #[inline(always)]
    fn node_text_range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

impl HasClosureAstNode for JsMethodClassMember {
    #[inline(always)]
    fn node_text_range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

impl HasClosureAstNode for JsGetterClassMember {
    #[inline(always)]
    fn node_text_range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

impl HasClosureAstNode for JsSetterClassMember {
    #[inline(always)]
    fn node_text_range(&self) -> TextRange {
        self.syntax().text_range()
    }
}

pub enum AnyHasClosureNode {
    JsFunctionDeclaration(JsFunctionDeclaration),
    JsFunctionExpression(JsFunctionExpression),
    JsArrowFunctionExpression(JsArrowFunctionExpression),
    JsConstructorClassMember(JsConstructorClassMember),
    JsMethodClassMember(JsMethodClassMember),
    JsGetterClassMember(JsGetterClassMember),
    JsSetterClassMember(JsSetterClassMember),
}

impl AnyHasClosureNode {
    pub fn from_node(node: &SyntaxNode<JsLanguage>) -> Option<AnyHasClosureNode> {
        match node.kind() {
            JsSyntaxKind::JS_FUNCTION_DECLARATION => node
                .clone()
                .cast::<JsFunctionDeclaration>()
                .map(AnyHasClosureNode::JsFunctionDeclaration),
            JsSyntaxKind::JS_FUNCTION_EXPRESSION => node
                .clone()
                .cast::<JsFunctionExpression>()
                .map(AnyHasClosureNode::JsFunctionExpression),
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => node
                .clone()
                .cast::<JsArrowFunctionExpression>()
                .map(AnyHasClosureNode::JsArrowFunctionExpression),
            JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => node
                .clone()
                .cast::<JsConstructorClassMember>()
                .map(AnyHasClosureNode::JsConstructorClassMember),
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER => node
                .clone()
                .cast::<JsMethodClassMember>()
                .map(AnyHasClosureNode::JsMethodClassMember),
            JsSyntaxKind::JS_GETTER_CLASS_MEMBER => node
                .clone()
                .cast::<JsGetterClassMember>()
                .map(AnyHasClosureNode::JsGetterClassMember),
            JsSyntaxKind::JS_SETTER_CLASS_MEMBER => node
                .clone()
                .cast::<JsSetterClassMember>()
                .map(AnyHasClosureNode::JsSetterClassMember),
            _ => None,
        }
    }
}

impl HasClosureAstNode for AnyHasClosureNode {
    #[inline(always)]
    fn node_text_range(&self) -> TextRange {
        match self {
            AnyHasClosureNode::JsFunctionDeclaration(node) => node.syntax().text_range(),
            AnyHasClosureNode::JsFunctionExpression(node) => node.syntax().text_range(),
            AnyHasClosureNode::JsArrowFunctionExpression(node) => node.syntax().text_range(),
            AnyHasClosureNode::JsConstructorClassMember(node) => node.syntax().text_range(),
            AnyHasClosureNode::JsMethodClassMember(node) => node.syntax().text_range(),
            AnyHasClosureNode::JsGetterClassMember(node) => node.syntax().text_range(),
            AnyHasClosureNode::JsSetterClassMember(node) => node.syntax().text_range(),
        }
    }
}

pub struct AllCapturesIter {
    data: Arc<SemanticModelData>,
    closure_range: TextRange,
    scopes: Vec<usize>,
    references: Vec<ScopeReference>,
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
                        range: reference.range,
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
                        self.references
                            .extend(scope.read_references.iter().cloned());
                        self.references
                            .extend(scope.write_references.iter().cloned());
                        self.scopes.extend(scope.children.iter());
                        continue 'references;
                    }
                }
            }

            return None;
        }
    }
}

pub struct ChildrenIter {
    data: Arc<SemanticModelData>,
    closure_range: TextRange,
    scopes: Vec<usize>,
}

impl Iterator for ChildrenIter {
    type Item = Closure;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(scope_id) = self.scopes.pop() {
            let scope = &self.data.scopes[scope_id];
            let node = &self.data.node_by_range[&scope.range];
            match node.kind() {
                JsSyntaxKind::JS_FUNCTION_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                    return Some(Closure {
                        data: self.data.clone(),
                        scope_id,
                        closure_range: self.closure_range,
                    });
                }
                _ => {
                    self.scopes.extend(scope.children.iter());
                }
            }
        }

        None
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
        let closure_range = node.node_text_range();
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

        let mut references = Vec::with_capacity(128);
        references.extend(scope.read_references.iter().cloned());
        references.extend(scope.write_references.iter().cloned());

        AllCapturesIter {
            data: self.data.clone(),
            closure_range: self.closure_range,
            scopes,
            references,
        }
    }

    /// Return all immediate children closures of this closure
    pub fn children(&self) -> impl Iterator<Item = Closure> {
        let scope = &self.data.scopes[self.scope_id];

        let mut scopes = Vec::with_capacity(128);
        scopes.extend(scope.children.iter().cloned());

        ChildrenIter {
            data: self.data.clone(),
            closure_range: self.closure_range,
            scopes,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rome_diagnostics::file::FileId;
    use rome_js_syntax::{JsArrowFunctionExpression, JsSyntaxKind, SourceType};
    use rome_rowan::SyntaxNodeCast;

    fn assert_closure(code: &str, name: &str, captures: &[&str]) {
        let r = rome_js_parser::parse(code, FileId::zero(), SourceType::tsx());
        let model = semantic_model(&r.tree());

        let closure = if name != "ARROWFUNCTION" {
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.text_trimmed() == name)
                .last()
                .unwrap();
            let node = node
                .parent()
                .and_then(|node| AnyHasClosureNode::from_node(&node))
                .unwrap();
            model.closure(&node)
        } else {
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
                .last()
                .unwrap()
                .cast::<JsArrowFunctionExpression>()
                .unwrap();
            model.closure(&node)
        };

        let expected_captures: BTreeSet<String> = captures.iter().map(|x| x.to_string()).collect();

        let all_captures: BTreeSet<String> = closure
            .all_captures()
            .map(|x| x.node().text_trimmed().to_string())
            .collect();

        let intersection = expected_captures.intersection(&all_captures);
        let intersection_count = intersection.count();

        assert_eq!(intersection_count, expected_captures.len());
        assert_eq!(intersection_count, all_captures.len());
    }

    fn get_closure_children(code: &str, name: &str) -> Vec<Closure> {
        let r = rome_js_parser::parse(code, FileId::zero(), SourceType::tsx());
        let model = semantic_model(&r.tree());

        let closure = if name != "ARROWFUNCTION" {
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.text_trimmed() == name)
                .last()
                .unwrap();
            let node = node
                .parent()
                .and_then(|node| AnyHasClosureNode::from_node(&node))
                .unwrap();
            model.closure(&node)
        } else {
            let node = r
                .syntax()
                .descendants()
                .filter(|x| x.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
                .last()
                .unwrap()
                .cast::<JsArrowFunctionExpression>()
                .unwrap();
            model.closure(&node)
        };

        closure.children().collect()
    }

    #[test]
    pub fn ok_semantic_model_closure() {
        assert_closure("function f() {}", "f", &[]);

        let two_captures = "let a, b; function f(c) {console.log(a, b, c)}";
        assert_closure(two_captures, "f", &["a", "b"]);
        assert_eq!(get_closure_children(two_captures, "f").len(), 0);

        let inner_function = "let a, b;
        function f(c) {
            console.log(a);
            function g() {
                console.log(b, c);
            }
        }";
        assert_closure(inner_function, "f", &["a"]);
        assert_closure(inner_function, "g", &["b", "c"]);
        assert_eq!(get_closure_children(inner_function, "f").len(), 1);
        assert_eq!(get_closure_children(inner_function, "g").len(), 0);

        let arrow_function = "let a, b;
        function f(c) {
            console.log(a);
            c.map(x => x + b + c);
        }";
        assert_closure(arrow_function, "f", &["a"]);
        assert_closure(arrow_function, "ARROWFUNCTION", &["b", "c"]);
        assert_eq!(get_closure_children(arrow_function, "f").len(), 1);
        assert_eq!(
            get_closure_children(arrow_function, "ARROWFUNCTION").len(),
            0
        );

        let writes = "let a;
        function f(c) {
            a = 1
        }";
        assert_closure(writes, "f", &["a"]);

        let class_callables = "let a;
        class A {
            constructor() { console.log(a); }
            f() { console.log(a); }

            get getValue() { console.log(a); }
            set setValue(v) { console.log(a); }
        }";
        assert_closure(class_callables, "constructor", &["a"]);
        assert_closure(class_callables, "f", &["a"]);
        assert_closure(class_callables, "getValue", &["a"]);
        assert_closure(class_callables, "setValue", &["a"]);
    }
}
