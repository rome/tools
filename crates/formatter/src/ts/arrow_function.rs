use rslint_parser::ast::{ArrowExpr, ArrowExprParams, ExprOrBlock};

use crate::{
	concat_elements, format_elements, space_token, token, ts::format_syntax_token, FormatElement,
	ToFormatElement,
};

impl ToFormatElement for ArrowExpr {
	fn to_format_element(&self) -> FormatElement {
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
					tokens.push(name.to_format_element());
					tokens.push(token(")"));
				}
				ArrowExprParams::ParameterList(params) => tokens.push(params.to_format_element()),
			}
		}

		tokens.push(space_token());
		if let Some(arrow) = self.fat_arrow_token() {
			tokens.push(format_syntax_token(arrow));
		}

		tokens.push(space_token());

		let body = self.body();

		if let Some(body) = body {
			match body {
				ExprOrBlock::Expr(expression) => {
					tokens.push(expression.to_format_element());
				}
				ExprOrBlock::Block(block) => tokens.push(block.to_format_element()),
			}
		}

		concat_elements(tokens)
	}
}
