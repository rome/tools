use crate::{
	concat_elements, space_token, token, FormatElement, FormatError, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::{AstNode, ForStmtInit, VarDecl};

impl ToFormatElement for VarDecl {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let mut tokens = vec![];

		if self.is_const() {
			tokens.push(formatter.format_token(&self.const_token()?)?);
		} else if self.is_var() {
			tokens.push(formatter.format_token(&self.var_token()?)?);
		} else if self.is_let() {
			// TODO: #1725 remove this custom code once #1745 is merged
			tokens.push(formatter.format_token(&self.let_token().unwrap())?);
		} else {
			return Err(FormatError::MissingRequiredChild);
		}

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
