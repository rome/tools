use crate::{
	concat_elements, format_elements, join_elements, space_token, token, ts::format_syntax_token,
	FormatElement, ToFormatElement,
};
use rslint_parser::ast::{ParameterList, Pattern};

impl ToFormatElement for ParameterList {
	fn to_format_element(&self) -> FormatElement {
		let mut tokens = vec![];
		if let Some(paren) = self.l_paren_token() {
			tokens.push(format_syntax_token(paren))
		}

		let param_tokens: Vec<_> = self
			.parameters()
			.map(|param| match param {
				Pattern::SinglePattern(single_pattern) => single_pattern.to_format_element(),
				Pattern::RestPattern(_) => todo!(),
				Pattern::AssignPattern(assign_pattern) => assign_pattern.to_format_element(),
				Pattern::ObjectPattern(_) => todo!(),
				Pattern::ArrayPattern(_) => todo!(),
				Pattern::ExprPattern(_) => todo!(),
			})
			.collect();

		tokens.push(format_elements!(join_elements(
			format_elements!(token(","), space_token()),
			param_tokens,
		)));

		if let Some(paren) = self.r_paren_token() {
			tokens.push(format_syntax_token(paren));
		}

		concat_elements(tokens)
	}
}
