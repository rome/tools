use crate::{assert_rename_nok, assert_rename_ok};
use rome_js_semantic::{AllReferencesExtensions, SemanticModel};
use rome_js_syntax::{
    JsIdentifierAssignment, JsIdentifierBinding, JsLanguage, JsReferenceIdentifier, JsSyntaxKind,
    JsSyntaxToken,
};
use rome_rowan::{AstNode, BatchMutation, SyntaxNodeCast, TriviaPiece};

pub trait RenameSymbolExtensions {
    fn rename(
        &mut self,
        model: &SemanticModel,
        binding: JsIdentifierBinding,
        new_name: &str,
    ) -> bool;
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

impl<N: AstNode<Language = JsLanguage>> RenameSymbolExtensions for BatchMutation<JsLanguage, N> {
    /// Rename the binding and all its references to the new_name.
    /// If we can´t rename the binding, the [BatchMutation] is never changes and it is left
    /// intact.
    fn rename(
        &mut self,
        model: &SemanticModel,
        prev_binding: JsIdentifierBinding,
        new_name: &str,
    ) -> bool {
        // We can rename a binding if there is no conflicts in the current scope.
        // We can shadow parent scopes, so we don´t check them.

        let scope = model.scope(prev_binding.syntax());
        if scope.get_binding(new_name).is_some() {
            return false;
        }

        let name_token = match prev_binding.name_token() {
            Ok(name_token) => name_token,
            Err(_) => return false,
        };

        // We can rename references, if there is no conflicts in any scope
        // until the root.

        let all_references: Vec<_> = prev_binding.all_references(model).collect();
        let mut changes = Vec::with_capacity(all_references.len());

        for reference in all_references.iter() {
            let scope = reference.scope();
            if scope
                .ancestors()
                .filter_map(|scope| scope.get_binding(new_name))
                .next()
                .is_some()
            {
                return false;
            }

            let prev_token = match reference.node().kind() {
                JsSyntaxKind::JS_REFERENCE_IDENTIFIER => reference
                    .node()
                    .clone()
                    .cast::<JsReferenceIdentifier>()
                    .and_then(|node| node.value_token().ok()),
                JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => reference
                    .node()
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

        // Now it is safe to pish changes to the batch mutation
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

assert_rename_ok! {
    ok_rename_declaration, "let a;", "let b;",
    ok_rename_declaration_inner_scope, "let b; if (true) { let a; }", "let b; if (true) { let b; }",
    ok_rename_read_reference, "let a; a + 1;", "let b; b + 1;",
    ok_rename_read_before_initit, "function f() { console.log(a); let a; }", "function f() { console.log(b); let b; }",
    ok_rename_write_reference, "let a; a = 1;", "let b; b = 1;",
    ok_rename_write_before_init, "function f() { a = 1; let a; }", "function f() { b = 1; let b; }",
    ok_rename_trivia_is_kept, "let /*1*/a/*2*/; /*3*/a/*4*/ = 1; /*5*/a/*6*/ + 1", "let /*1*/b/*2*/; /*3*/b/*4*/ = 1; /*5*/b/*6*/ + 1",
}

assert_rename_nok! {
    nok_rename_declaration_conflict_before, "let b; let a;",
    nok_rename_declaration_conflict_after, "let a; let b;",
    nok_rename_read_reference, "let a; if (true) { let b; a + 1 }",
    nok_rename_read_reference_conflict_hoisting_same_scope, "let a; if (true) { a + 1; var b; }",
    nok_rename_read_reference_conflict_hoisting_outer_scope, "let a; if (true) { a + 1; } var b;",
    nok_rename_write_reference, "let a; if (true) { let b; a = 1 }",
    nok_rename_read_reference_parent_scope_conflict, "function f() { let b; if(true) { console.log(a); } } var a;",
}
