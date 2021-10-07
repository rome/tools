use rslint_parser::ast::{ArrowExpr, ArrowExprParams};

use crate::{format_tokens, FormatToken, FormatValue};

impl FormatValue for ArrowExpr {
	fn format(&self) -> FormatToken {
		let mut tokens: Vec<FormatToken> = vec![];

		if let Some(async_token) = self.async_token() {
			tokens.push(format_tokens!(
				async_token.text().as_str(),
				FormatToken::Space
			));
		}

		if let Some(arrow_expression_params) = self.params() {
			match arrow_expression_params {
				ArrowExprParams::Name(name) => {
					tokens.push(format_tokens!("("));
					tokens.push(name.format());
					tokens.push(format_tokens!(")"));
				}
				ArrowExprParams::ParameterList(params) => tokens.push(params.format()),
			}
		}

		tokens.push(format_tokens!(FormatToken::Space));
		if let Some(arrow) = self.fat_arrow_token() {
			tokens.push(format_tokens!(arrow.text().as_str()));
		} else {
			tokens.push(format_tokens!("=>"));
		}
		tokens.push(format_tokens!(FormatToken::Space));

		let body = self.body();

		if let Some(body) = body {
			match body {
				rslint_parser::ast::ExprOrBlock::Expr(expression) => {
					tokens.push(expression.format());
				}
				rslint_parser::ast::ExprOrBlock::Block(block) => tokens.push(block.format()),
			}
		}

		FormatToken::concat(tokens)
	}
}
