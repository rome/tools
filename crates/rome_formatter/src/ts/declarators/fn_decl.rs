use crate::{
	concat_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsFunctionDeclaration;

impl ToFormatElement for JsFunctionDeclaration {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let mut tokens = vec![];

		if let Some(token) = self.async_token() {
			tokens.push(formatter.format_token(&token)?);
			tokens.push(space_token());
		}

		tokens.push(formatter.format_token(&self.function_token()?)?);

		if let Some(token) = self.star_token() {
			tokens.push(formatter.format_token(&token)?);
		}

		tokens.push(space_token());
		tokens.push(formatter.format_node(self.id()?)?);
		tokens.push(formatter.format_node(self.parameters()?)?);
		tokens.push(space_token());
		tokens.push(formatter.format_node(self.body()?)?);

		Ok(concat_elements(tokens))
	}
}
