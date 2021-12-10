use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsComputedMemberName;

impl ToFormatElement for JsComputedMemberName {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.l_brack_token()?)?,
			formatter.format_node(self.expression()?)?,
			formatter.format_token(&self.r_brack_token()?)?,
		])
	}
}
