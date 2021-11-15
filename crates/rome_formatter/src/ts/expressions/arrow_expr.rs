use rslint_parser::ast::{
	JsAnyArrowFunctionBody, JsAnyArrowFunctionParameters, JsArrowFunctionExpression,
};

use crate::{
	concat_elements, format_elements, space_token, token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};

impl ToFormatElement for JsArrowFunctionExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let mut tokens: Vec<FormatElement> = vec![];

		if let Some(async_token) = self.async_token() {
			tokens.push(format_elements!(
				formatter.format_token(&async_token)?,
				space_token()
			));
		}

		if let Some(params) = self.parameter_list() {
			match params {
				JsAnyArrowFunctionParameters::JsIdentifierBinding(name) => {
					tokens.push(token("("));
					tokens.push(formatter.format_node(name)?);
					tokens.push(token(")"));
				}
				JsAnyArrowFunctionParameters::JsParameterList(params) => {
					tokens.push(formatter.format_node(params)?)
				}
			}
		}

		tokens.push(space_token());
		tokens.push(formatter.format_token(&self.fat_arrow_token()?)?);
		tokens.push(space_token());
		if let Some(body) = self.body() {
			tokens.push(formatter.format_node(body)?);
		}

		Ok(concat_elements(tokens))
	}
}

impl ToFormatElement for JsAnyArrowFunctionBody {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyArrowFunctionBody::JsFunctionBody(body) => body.to_format_element(formatter),
			JsAnyArrowFunctionBody::JsAnyExpression(expr) => expr.to_format_element(formatter),
		}
	}
}
