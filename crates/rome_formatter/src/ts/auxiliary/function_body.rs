use rslint_parser::ast::JsFunctionBody;

use crate::ts::statements::format_statements;
use crate::{
	block_indent, format_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsFunctionBody {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.l_curly_token()?)?,
			block_indent(format_statements(self.statements(), formatter)),
			formatter.format_token(&self.r_curly_token()?)?
		])
	}
}
