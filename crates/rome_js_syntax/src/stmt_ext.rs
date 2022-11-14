//! Extended AST node definitions for statements which are unique and special enough to generate code for manually

use crate::{JsForVariableDeclaration, JsVariableDeclaration, T};
use rome_rowan::SyntaxResult;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JsVariableKind {
    Const,
    Let,
    Var,
}

impl JsVariableDeclaration {
    /// Whether the declaration is a const declaration
    pub fn is_const(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Const)
    }

    /// Whether the declaration is a let declaration
    pub fn is_let(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Let)
    }

    /// Whether the declaration is a var declaration
    pub fn is_var(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Var)
    }

    pub fn variable_kind(&self) -> SyntaxResult<JsVariableKind> {
        let token_kind = self.kind().map(|t| t.kind())?;

        Ok(match token_kind {
            T![const] => JsVariableKind::Const,
            T![let] => JsVariableKind::Let,
            T![var] => JsVariableKind::Var,
            _ => unreachable!(),
        })
    }
}

impl JsForVariableDeclaration {
    /// Whether the declaration is a const declaration
    pub fn is_const(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Const)
    }

    /// Whether the declaration is a let declaration
    pub fn is_let(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Let)
    }

    /// Whether the declaration is a var declaration
    pub fn is_var(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Var)
    }

    pub fn variable_kind(&self) -> SyntaxResult<JsVariableKind> {
        let token_kind = self.kind_token().map(|t| t.kind())?;

        Ok(match token_kind {
            T![const] => JsVariableKind::Const,
            T![let] => JsVariableKind::Let,
            T![var] => JsVariableKind::Var,
            _ => unreachable!(),
        })
    }
}

#[cfg(test)]
mod tests {
    use rome_js_factory::syntax::{JsSyntaxKind::*, JsVariableDeclaration};
    use rome_js_factory::JsSyntaxTreeBuilder;
    use rome_rowan::AstNode;

    #[test]
    fn is_var_check() {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        tree_builder.start_node(JS_VARIABLE_DECLARATION);
        tree_builder.token(VAR_KW, "var");
        tree_builder.start_node(JS_VARIABLE_DECLARATOR_LIST);
        tree_builder.start_node(JS_VARIABLE_DECLARATOR);

        tree_builder.start_node(JS_IDENTIFIER_BINDING);
        tree_builder.token(IDENT, "a");
        tree_builder.finish_node();

        tree_builder.finish_node(); // declarator
        tree_builder.finish_node(); // list
        tree_builder.finish_node(); // declaration

        let root = tree_builder.finish();

        let var_decl = JsVariableDeclaration::cast(root).unwrap();

        assert!(var_decl.is_var());
    }
}
