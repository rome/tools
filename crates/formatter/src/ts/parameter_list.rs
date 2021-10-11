use crate::{
	concat_elements, format_elements, join_elements, space_token, syntax_token, token,
	FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::ParameterList;

impl ToFormatElement for ParameterList {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let mut elements = vec![];
		if let Some(paren) = self.l_paren_token() {
			elements.push(syntax_token(&paren))
		}

		let param_tokens = self.parameters().map(|param| formatter.format_node(param));

		elements.push(format_elements!(join_elements(
			format_elements!(token(","), space_token()),
			param_tokens,
		)));

		if let Some(paren) = self.r_paren_token() {
			elements.push(syntax_token(&paren));
		}

		concat_elements(elements)
	}
}
