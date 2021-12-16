//! Extended AST node definitions for statements which are unique and special enough to generate code for manually

use crate::{ast::*, SyntaxKind::*, SyntaxNodeExt, T};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JsVariableKind {
	Const,
	Let,
	Var,
}

impl ForStmt {
	pub fn second_semi_token(&self) -> SyntaxResult<SyntaxToken> {
		self.syntax
			.children_with_tokens()
			.filter_map(|e| e.into_token())
			.filter(|t| t.kind() == T![;])
			.nth(1)
			.ok_or_else(|| SyntaxError::MissingRequiredChild(self.syntax().clone()))
	}
}

impl JsVariableDeclarations {
	/// Whether the declaration is a const declaration
	pub fn is_const(&self) -> bool {
		self.variable_kind() == Ok(JsVariableKind::Const)
	}

	/// Whether the declaration is a let declaration
	pub fn is_let(&self) -> bool {
		self.variable_kind() == Ok(JsVariableKind::Let)
	}

	/// Whether the declaration is a let declaration
	pub fn is_var(&self) -> bool {
		self.variable_kind() == Ok(JsVariableKind::Const)
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

impl Specifier {
	pub fn alias(&self) -> Option<JsName> {
		self.syntax().children().nth(1).and_then(|x| x.try_to())
	}
}

impl JsAnySwitchClause {
	pub fn into_case(self) -> Option<JsCaseClause> {
		if let JsAnySwitchClause::JsCaseClause(clause) = self {
			Some(clause)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn var_decl_let_token() {
		let parsed = parse_text("/* */let a = 5;", 0).tree();
		let var_decl = parsed
			.statements()
			.iter()
			.find_map(|stmt| ast::JsVariableStatement::cast(stmt.syntax().clone()));

		assert!(var_decl.is_some());
	}
}

impl TsEnumMember {
	pub fn string_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, JS_STRING_LITERAL)
	}
}
