use crate::{
	concat_elements, space_token, token, FormatElement, FormatError, Formatter, ToFormatElement,
};
use rslint_parser::ast::ReturnStmt;

impl ToFormatElement for ReturnStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let mut tokens = vec![token("return")];

		tokens.push(space_token());
		tokens.push(formatter.format_node(self.value()?)?);

		tokens.push(token(";"));

		Ok(concat_elements(tokens))
	}
}
