use crate::{format_tokens, FormatToken, FormatValue, ListToken};
use rslint_parser::ast::ParameterList;

impl FormatValue for ParameterList {
	fn format(&self) -> FormatToken {
		let mut tokens = vec![];
		if let Some(paren) = self.l_paren_token() {
			tokens.push(format_tokens!(paren.text().as_str()))
		}

		let param_tokens: Vec<_> = self
			.parameters()
			.map(|param| match param {
				rslint_parser::ast::Pattern::SinglePattern(single_pattern) => {
					single_pattern.format()
				}
				rslint_parser::ast::Pattern::RestPattern(_) => todo!(),
				rslint_parser::ast::Pattern::AssignPattern(_) => todo!(),
				rslint_parser::ast::Pattern::ObjectPattern(_) => todo!(),
				rslint_parser::ast::Pattern::ArrayPattern(_) => todo!(),
				rslint_parser::ast::Pattern::ExprPattern(_) => todo!(),
			})
			.collect();

		tokens.push(format_tokens!(ListToken::join(
			format_tokens!(",", FormatToken::Space),
			param_tokens,
		)));

		if let Some(paren) = self.r_paren_token() {
			tokens.push(format_tokens!(paren.text().as_str()));
		}

		FormatToken::concat(tokens)
	}
}
