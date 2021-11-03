use crate::{
	concat_elements, space_token, token, FormatElement, FormatError, Formatter, ToFormatElement,
};
use rslint_parser::ast::{AstNode, ForStmtInit, VarDecl};

impl ToFormatElement for VarDecl {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let mut tokens = vec![];

		//  TODO review
		// if let Some(token) = self.const_token() {
		// 	tokens.push(formatter.format_token(&token)?);
		// } else if let Some(token) = self.let_token() {
		// 	tokens.push(formatter.format_token(&token)?);
		// } else if let Some(token) = self.var_token() {
		// 	tokens.push(formatter.format_token(&token)?);
		// } else {
		// 	return None;
		// }

		tokens.push(space_token());

		for declarator in self.declared() {
			tokens.push(formatter.format_node(declarator)?);
		}

		// don't add a semicolon if the var decl is in the init section of a for statement to avoid
		// terminating the `init` with two semicolons.
		if self.syntax().parent().and_then(ForStmtInit::cast).is_none() {
			tokens.push(token(";"));
		}

		Ok(concat_elements(tokens))
	}
}
