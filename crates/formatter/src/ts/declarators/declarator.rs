use crate::{
	concat_elements, space_token, token, ts::format_syntax_token, FormatToken, FormatValue,
};
use rslint_parser::ast::{Declarator, Pattern};

impl FormatValue for Declarator {
	fn format(&self) -> FormatToken {
		let mut tokens = vec![];

		if let Some(pattern) = self.pattern() {
			let token = match pattern {
				Pattern::SinglePattern(single_pattern) => single_pattern.format(),
				Pattern::RestPattern(_) => todo!(),
				Pattern::AssignPattern(_) => todo!(),
				Pattern::ObjectPattern(_) => todo!(),
				Pattern::ArrayPattern(_) => todo!(),
				Pattern::ExprPattern(_) => todo!(),
			};

			tokens.push(token);
		}
		if let Some(equal) = self.eq_token() {
			tokens.push(space_token());
			tokens.push(format_syntax_token(equal));
			tokens.push(space_token());
		}

		if let Some(expression) = self.value() {
			tokens.push(expression.format());
		}
		tokens.push(token(";"));

		concat_elements(tokens)
	}
}
