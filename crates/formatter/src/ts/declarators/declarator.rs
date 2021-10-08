use crate::{
	concat_elements, space_token, syntax_token, token, FormatContext, FormatElement,
	ToFormatElement,
};
use rslint_parser::ast::Declarator;

impl ToFormatElement for Declarator {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		let mut tokens = vec![];

		if let Some(pattern) = self.pattern() {
			tokens.push(context.format_node(pattern));
		}
		if let Some(equal) = self.eq_token() {
			tokens.push(space_token());
			tokens.push(syntax_token(&equal));
			tokens.push(space_token());
		}

		if let Some(expression) = self.value() {
			tokens.push(context.format_node(expression));
		}
		tokens.push(token(";"));

		concat_elements(tokens)
	}
}
