use crate::format_token::{IndentToken, LineToken};
use crate::{
	format_token::{ConcatTokens, FormatToken},
	FormatValue,
};
use serde_json::Value;

impl FormatValue for Value {
	fn format(&self) -> FormatToken {
		match self {
			Value::String(string) => FormatToken::string(format!("\"{}\"", string).as_str()),
			Value::Number(number) => {
				let number = number.as_f64().unwrap();
				FormatToken::f64(number)
			}
			Value::Bool(value) => FormatToken::from(value),
			Value::Object(value) => {
				let mut content = ConcatTokens::new();
				for (key, value) in value {
					content = content
						.push_token(format!("\"{}\":", key).as_str())
						.push_token(FormatToken::Space)
						.push_token(value.format())
						.push_token(",")
						.push_token(LineToken::soft_or_space());
				}
				ConcatTokens::new()
					.push_token("{")
					.push_token(LineToken::soft())
					.push_token(IndentToken::new(content.format_tokens()))
					.push_token(LineToken::soft())
					.push_token("}")
					.format_tokens()
			}
			_ => unimplemented!("Implement rest"),
		}
	}
}

pub fn json_to_tokens(content: &str) -> FormatToken {
	let json: Value = serde_json::from_str(content).unwrap();

	json.format()
}

#[cfg(test)]
mod test {
	use crate::{format_token::ConcatTokens, FormatToken};

	use super::json_to_tokens;
	use crate::format_token::{IndentToken, LineToken};

	#[test]
	fn tokenize_number() {
		let result = json_to_tokens("6.45");

		assert_eq!(FormatToken::string("6.45"), result);
	}

	#[test]
	fn tokenize_string() {
		let result = json_to_tokens(r#""foo""#);

		assert_eq!(FormatToken::string(r#""foo""#), result);
	}

	#[test]
	fn tokenize_boolean_false() {
		let result = json_to_tokens("false");

		assert_eq!(FormatToken::string("false"), result);
	}

	#[test]
	fn tokenize_boolean_true() {
		let result = json_to_tokens("true");

		assert_eq!(FormatToken::string("true"), result);
	}

	#[test]
	fn tokenize_object() {
		let input = r#"{ "foo": "bar", "num": 5 }"#;
		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(LineToken::soft())
			.push_token(IndentToken::new(
				ConcatTokens::new()
					// 'foo'
					.push_token("\"foo\":")
					.push_token(FormatToken::Space)
					.push_token("\"bar\"")
					.push_token(",")
					.push_token(LineToken::soft_or_space())
					// 'num'
					.push_token("\"num\":")
					.push_token(FormatToken::Space)
					.push_token(FormatToken::string("5"))
					.push_token(",")
					.push_token(LineToken::soft_or_space())
					.format_tokens(),
			))
			.push_token(LineToken::soft())
			.push_token("}")
			.format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}
}
