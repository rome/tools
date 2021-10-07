use rslint_parser::ast::{ArrowExpr, ArrowExprParams};

use crate::{
	concat_elements, format_elements, space_token, token, ts::format_syntax_token, FormatElement,
	FormatValue,
};

impl FormatValue for ArrowExpr {
	fn format(&self) -> FormatElement {
		let mut tokens: Vec<FormatElement> = vec![];

		if let Some(async_token) = self.async_token() {
			tokens.push(format_elements!(
				format_syntax_token(async_token),
				space_token()
			));
		}

		if let Some(arrow_expression_params) = self.params() {
			match arrow_expression_params {
				ArrowExprParams::Name(name) => {
					tokens.push(token("("));
					tokens.push(name.format());
					tokens.push(token(")"));
				}
				ArrowExprParams::ParameterList(params) => tokens.push(params.format()),
			}
		}

		tokens.push(space_token());
		if let Some(arrow) = self.fat_arrow_token() {
			tokens.push(format_syntax_token(arrow));
		} else {
			tokens.push(token("=>"));
		}
		tokens.push(space_token());

		let body = self.body();

		if let Some(body) = body {
			match body {
				rslint_parser::ast::ExprOrBlock::Expr(expression) => {
					tokens.push(expression.format());
				}
				rslint_parser::ast::ExprOrBlock::Block(block) => tokens.push(block.format()),
			}
		}

		concat_elements(tokens)
	}
}
