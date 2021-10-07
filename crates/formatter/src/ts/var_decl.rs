use crate::{
	concat_elements, space_token, token, ts::format_syntax_token, FormatElement, ToFormatElement,
};
use rslint_parser::ast::VarDecl;

impl ToFormatElement for VarDecl {
	fn to_format_element(&self) -> FormatElement {
		let mut tokens = vec![];

		if let Some(token) = self.const_token() {
			tokens.push(format_syntax_token(token));
		} else if let Some(token) = self.let_token() {
			tokens.push(format_syntax_token(token));
		} else if let Some(token) = self.var_token() {
			tokens.push(format_syntax_token(token));
		} else {
			// TODO: Diagnostic?
			tokens.push(token("var"));
		}
		tokens.push(space_token());

		for declarator in self.declared() {
			tokens.push(declarator.to_format_element());
		}

		concat_elements(tokens)
	}
}
