use crate::{concat_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::{ForStmtInit, VarDecl};
use rslint_parser::AstNode;

impl ToFormatElement for VarDecl {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let mut tokens = vec![];

		if let Some(token) = self.const_token() {
			tokens.push(formatter.format_token(&token));
		} else if let Some(token) = self.let_token() {
			tokens.push(formatter.format_token(&token));
		} else if let Some(token) = self.var_token() {
			tokens.push(formatter.format_token(&token));
		} else {
			// TODO: Diagnostic?
			tokens.push(token("var"));
		}
		tokens.push(space_token());

		for declarator in self.declared() {
			tokens.push(formatter.format_node(declarator));
		}

		// don't add a semicolon if the var decl is in the init section of a for statement to avoid
		// terminating the `init` with two semicolons.
		if self.syntax().parent().and_then(ForStmtInit::cast).is_none() {
			tokens.push(token(";"));
		}

		concat_elements(tokens)
	}
}
