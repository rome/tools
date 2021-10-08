use rslint_parser::ast::{ArrowExpr, ArrowExprParams};

use crate::{
	concat_elements, format_elements, space_token, syntax_token, token, FormatContext,
	FormatElement, ToFormatElement,
};

impl ToFormatElement for ArrowExpr {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		let mut tokens: Vec<FormatElement> = vec![];

		if let Some(async_token) = self.async_token() {
			tokens.push(format_elements!(syntax_token(&async_token), space_token()));
		}

		if let Some(arrow_expression_params) = self.params() {
			match arrow_expression_params {
				ArrowExprParams::Name(name) => {
					tokens.push(token("("));
					tokens.push(context.format_node(name));
					tokens.push(token(")"));
				}
				ArrowExprParams::ParameterList(params) => tokens.push(context.format_node(params)),
			}
		}

		tokens.push(space_token());
		if let Some(arrow) = self.fat_arrow_token() {
			tokens.push(syntax_token(&arrow));
		}

		tokens.push(space_token());

		let body = self.body();

		if let Some(body) = body {
			tokens.push(context.format_node(body));
		}

		concat_elements(tokens)
	}
}
