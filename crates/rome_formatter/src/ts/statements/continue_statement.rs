use crate::{
	empty_element, format_elements, space_token, token, FormatElement, FormatError, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::ContinueStmt;

impl ToFormatElement for ContinueStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		// NOTE: rslint parser (upstream) is currently broken https://github.com/rslint/rslint/issues/126
		let ident = if let Ok(name_ref) = self.name_ref() {
			format_elements![space_token(), formatter.format_node(name_ref)?]
		} else {
			empty_element()
		};
		let continue_token = formatter.format_token(&self.continue_token()?)?;
		Ok(format_elements![continue_token, ident, token(";")])
	}
}
