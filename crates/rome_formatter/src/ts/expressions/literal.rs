use crate::{FormatElement, FormatResult, Formatter, ToFormatElement, token_from_string, token};
use rslint_parser::{AstNode, ast::{
	JsAnyLiteral, JsBigIntLiteral, JsBooleanLiteral, JsNullLiteral, JsNumberLiteral,
	JsStringLiteral,
}};

impl ToFormatElement for JsStringLiteral {
	fn to_format_element(&self, _: &Formatter) -> FormatResult<FormatElement> {
		let value_token = self.value_token()?;
		let quoted = value_token.text_trimmed();

		// uses single quotes
		if quoted.starts_with('\'') {
			let s = &quoted[1..quoted.len() - 1];
			let s = format!("\"{}\"", s);
			Ok(token_from_string(s))
		} else {
			Ok(token(quoted))
		}
	}
}

impl ToFormatElement for JsBooleanLiteral {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.value_token()?)
	}
}

impl ToFormatElement for JsNullLiteral {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.value_token()?)
	}
}

impl ToFormatElement for JsNumberLiteral {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.value_token()?)
	}
}

impl ToFormatElement for JsBigIntLiteral {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.value_token()?)
	}
}

impl ToFormatElement for JsAnyLiteral {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyLiteral::JsBooleanLiteral(boolean) => boolean.to_format_element(formatter),
			JsAnyLiteral::JsStringLiteral(string_literal) => {
				string_literal.to_format_element(formatter)
			}
			JsAnyLiteral::JsNumberLiteral(number) => number.to_format_element(formatter),
			JsAnyLiteral::JsBigIntLiteral(big_int) => big_int.to_format_element(formatter),
			JsAnyLiteral::JsNullLiteral(null_literal) => null_literal.to_format_element(formatter),
			JsAnyLiteral::JsRegexLiteral(_) => todo!(),
		}
	}
}
