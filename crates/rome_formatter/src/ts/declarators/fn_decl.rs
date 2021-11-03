use crate::{concat_elements, space_token, FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::FnDecl;

impl ToFormatElement for FnDecl {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
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
		tokens.push(formatter.format_node(self.name()?)?);
		tokens.push(formatter.format_node(self.parameters()?)?);
		tokens.push(space_token());
		tokens.push(formatter.format_node(self.body()?)?);

		Ok(concat_elements(tokens))
	}
}
