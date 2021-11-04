use crate::{
	format_elements, space_token, token, FormatElement, FormatError, Formatter, ToFormatElement,
};
use rslint_parser::ast::ThrowStmt;

impl ToFormatElement for ThrowStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let throw_token = formatter.format_token(&self.throw_token()?)?;
		let exception = formatter.format_node(self.exception()?)?;
		Ok(format_elements![
			throw_token,
			space_token(),
			exception,
			token(";")
		])
	}
}
