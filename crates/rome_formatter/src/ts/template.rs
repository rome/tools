use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::Template;

impl ToFormatElement for Template {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		// TODO implement template string, see parser in rslint_parser/src/ast/expr_ext.rs
		Ok(format_elements![
			formatter.format_token(&self.l_tick_token()?)?,
			token("TODO"),
			formatter.format_token(&self.r_tick_token()?)?
		])
	}
}
