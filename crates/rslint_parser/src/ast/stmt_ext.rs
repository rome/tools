//! Extended AST node definitions for statements which are unique and special enough to generate code for manually

use crate::{ast::*, T};

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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn var_decl_let_token() {
        let parsed = parse_script("/* */let a = 5;", 0).tree();
        let var_decl = parsed
            .statements()
            .iter()
            .find_map(|stmt| ast::JsVariableStatement::cast(stmt.syntax().clone()));

        assert!(var_decl.is_some());
    }

    #[test]
    fn is_var_check() {
        let root = parse_script("var a = 5;", 0).syntax();
        let var_decl = root
            .descendants()
            .find_map(ast::JsVariableDeclaration::cast);

        assert!(var_decl.unwrap().is_var());
    }
}
