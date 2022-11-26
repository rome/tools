use rome_js_semantic::{ReferencesExtensions, SemanticModel};
use rome_js_syntax::{
    JsIdentifierAssignment, JsIdentifierBinding, JsLanguage, JsReferenceIdentifier, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::{AstNode, BatchMutation, SyntaxNodeCast, TriviaPiece};
use serde::{Deserialize, Serialize};

pub trait RenamableNode {
    fn binding(&self, model: &SemanticModel) -> Option<JsSyntaxNode>;
}

impl RenamableNode for JsIdentifierBinding {
    fn binding(&self, _: &SemanticModel) -> Option<JsSyntaxNode> {
        Some(self.syntax().clone())
    }
}

impl RenamableNode for JsReferenceIdentifier {
    fn binding(&self, model: &SemanticModel) -> Option<JsSyntaxNode> {
        Some(model.binding(self)?.syntax().clone())
    }
}

impl RenamableNode for JsIdentifierAssignment {
    fn binding(&self, model: &SemanticModel) -> Option<JsSyntaxNode> {
        Some(model.binding(self)?.syntax().clone())
    }
}

pub enum JsAnyRenamableDeclaration {
    JsIdentifierBinding(JsIdentifierBinding),
    JsReferenceIdentifier(JsReferenceIdentifier),
    JsIdentifierAssignment(JsIdentifierAssignment),
}

impl RenamableNode for JsAnyRenamableDeclaration {
    fn binding(&self, model: &SemanticModel) -> Option<JsSyntaxNode> {
        match self {
            JsAnyRenamableDeclaration::JsIdentifierBinding(node) => {
                RenamableNode::binding(node, model)
            }
            JsAnyRenamableDeclaration::JsReferenceIdentifier(node) => {
                RenamableNode::binding(node, model)
            }
            JsAnyRenamableDeclaration::JsIdentifierAssignment(node) => {
                RenamableNode::binding(node, model)
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum RenameError {
    CannotFindDeclaration,
    CannotBeRenamed {
        original_name: String,
        new_name: String,
    },
}

impl TryFrom<JsSyntaxNode> for JsAnyRenamableDeclaration {
    type Error = RenameError;

    fn try_from(node: JsSyntaxNode) -> Result<Self, Self::Error> {
        match node.kind() {
            JsSyntaxKind::JS_IDENTIFIER_BINDING => node
                .cast::<JsIdentifierBinding>()
                .map(JsAnyRenamableDeclaration::JsIdentifierBinding)
                .ok_or(Self::Error::CannotFindDeclaration),
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER => node
                .cast::<JsReferenceIdentifier>()
                .map(JsAnyRenamableDeclaration::JsReferenceIdentifier)
                .ok_or(Self::Error::CannotFindDeclaration),
            JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => node
                .cast::<JsIdentifierAssignment>()
                .map(JsAnyRenamableDeclaration::JsIdentifierAssignment)
                .ok_or(Self::Error::CannotFindDeclaration),
            _ => Err(Self::Error::CannotFindDeclaration),
        }
    }
}

pub trait RenameSymbolExtensions {
    /// Rename the binding and all its references to "new_name".
    fn rename_node_declaration(
        &mut self,
        model: &SemanticModel,
        node: impl RenamableNode,
        new_name: &str,
    ) -> bool;

    /// Rename a symbol using the new name from the candidates iterator
    /// until the first success.
    ///
    /// A usual use case is to append a suffix to a variable name.
    ///
    /// ```ignore
    /// let new_name = "new_name";
    /// let candidates = (2..).map(|i| format!("{}{}", new_name, i).into());
    /// let candidates = once(Cow::from(new_name)).chain(candidates);
    /// batch.try_rename_node_declaration_until_success(model, node, candidates);
    /// ```
    fn rename_node_declaration_with_retry<S, I>(
        &mut self,
        model: &SemanticModel,
        node: impl RenamableNode + Clone,
        candidates: I,
    ) -> bool
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        for candidate in candidates {
            if self.rename_node_declaration(model, node.clone(), candidate.as_ref()) {
                return true;
            }
        }

        false
    }

    /// Rename the binding and all its references to "new_name".
    fn rename_any_renamable_node(
        &mut self,
        model: &SemanticModel,
        node: JsAnyRenamableDeclaration,
        new_name: &str,
    ) -> bool {
        self.rename_node_declaration(model, node, new_name)
    }
}

fn token_with_new_text(token: &JsSyntaxToken, new_text: &str) -> JsSyntaxToken {
    let new_text = format!(
        "{}{}{}",
        token.leading_trivia().text(),
        new_text,
        token.trailing_trivia().text()
    );

    let leading = token
        .leading_trivia()
        .pieces()
        .map(|item| TriviaPiece::new(item.kind(), item.text_len()))
        .collect::<Vec<_>>();
    let trailing = token
        .trailing_trivia()
        .pieces()
        .map(|item| TriviaPiece::new(item.kind(), item.text_len()))
        .collect::<Vec<_>>();

    JsSyntaxToken::new_detached(JsSyntaxKind::IDENT, new_text.as_str(), leading, trailing)
}

impl RenameSymbolExtensions for BatchMutation<JsLanguage> {
    /// Rename the binding and all its references to "new_name".
    /// If we can´t rename the binding, the [BatchMutation] is never changes and it is left
    /// intact.
    fn rename_node_declaration(
        &mut self,
        model: &SemanticModel,
        node: impl RenamableNode,
        new_name: &str,
    ) -> bool {
        let prev_binding = match node
            .binding(model)
            .and_then(|node| node.cast::<JsIdentifierBinding>())
        {
            Some(prev_binding) => prev_binding,
            None => return false,
        };

        // We can rename a binding if there is no conflicts in the current scope.
        // We can shadow parent scopes, so we don´t check them.

        let syntax = prev_binding.syntax();
        let scope = model
            .scope_hoisted_to(syntax)
            .unwrap_or_else(|| model.scope(syntax));
        if scope.get_binding(new_name).is_some() {
            return false;
        }

        let name_token = match prev_binding.name_token() {
            Ok(name_token) => name_token,
            Err(_) => {
                return false;
            }
        };

        // We can rename references, if there is no conflicts in any scope
        // until the root.

        let all_references: Vec<_> = prev_binding.all_references(model).collect();
        let mut changes = Vec::with_capacity(all_references.len());

        for reference in all_references {
            let scope = reference.scope();
            if scope
                .ancestors()
                .find_map(|scope| scope.get_binding(new_name))
                .is_some()
            {
                return false;
            }

            let prev_token = match reference.syntax().kind() {
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER => reference
                    .syntax()
                    .clone()
                    .cast::<JsReferenceIdentifier>()
                    .and_then(|node| node.value_token().ok()),
                JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => reference
                    .syntax()
                    .clone()
                    .cast::<JsIdentifierAssignment>()
                    .and_then(|node| node.name_token().ok()),
                _ => None,
            };

            if let Some(prev_token) = prev_token {
                let next_token = token_with_new_text(&prev_token, new_name);
                changes.push((prev_token, next_token));
            }
        }

        // Now it is safe to push changes to the batch mutation
        // Rename binding

        let next_name_token = token_with_new_text(&name_token, new_name);
        let next_binding = prev_binding.clone().with_name_token(next_name_token);
        self.replace_node(prev_binding, next_binding);

        // Rename all references

        for (prev_token, next_token) in changes {
            self.replace_token(prev_token, next_token);
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_rename_nok, assert_rename_ok};

    assert_rename_ok! {
        ok_rename_declaration,
            "let a;",
            "let b;",
        ok_rename_declaration_inner_scope,
            "let b; if (true) { let a; }",
            "let b; if (true) { let b; }",
        ok_rename_read_reference,
            "let a; a + 1;",
            "let b; b + 1;",
        ok_rename_read_before_initit,
            "function f() { console.log(a); let a; }",
            "function f() { console.log(b); let b; }",
        ok_rename_write_reference,
            "let a; a = 1;",
            "let b; b = 1;",
        ok_rename_write_before_init,
            "function f() { a = 1; let a; }",
            "function f() { b = 1; let b; }",
        ok_rename_trivia_is_kept,
            "let /*1*/a/*2*/; /*3*/a/*4*/ = 1; /*5*/a/*6*/ + 1",
            "let /*1*/b/*2*/; /*3*/b/*4*/ = 1; /*5*/b/*6*/ + 1",
        ok_rename_function_same_name,
            "function a() { function b() {console.log(2)}; console.log(1); b(); } a();",
            "function b() { function b() {console.log(2)}; console.log(1); b(); } b();",
    }

    assert_rename_nok! {
        nok_rename_declaration_conflict_before, "let b; let a;",
        nok_rename_declaration_conflict_after, "let a; let b;",
        nok_rename_read_reference, "let a; if (true) { let b; a + 1 }",
        nok_rename_read_reference_conflict_hoisting_same_scope, "let a; if (true) { a + 1; var b; }",
        nok_rename_read_reference_conflict_hoisting_outer_scope, "let a; if (true) { a + 1; } var b;",
        nok_rename_write_reference, "let a; if (true) { let b; a = 1 }",
        nok_rename_read_reference_parent_scope_conflict, "function f() { let b; if(true) { console.log(a); } } var a;",
        nok_rename_function_conflict, "function a() {} function b() {}",
    }
}
