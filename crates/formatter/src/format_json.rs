use crate::format_token::{IndentToken, LineToken};
use crate::{
	format_token::{ConcatTokens, FormatToken},
	FormatValue,
};
use serde_json::Value;

impl FormatValue for Value {
	fn format(&self) -> FormatToken {
		match self {
			Value::String(string) => {
				FormatToken::from(vec!["\"".into(), string.as_str().into(), "\"".into()])
			}
			Value::Number(number) => {
				let number = number.as_f64().unwrap();
				FormatToken::from(number)
			}
			Value::Bool(value) => FormatToken::from(value),
			Value::Object(value) => {
				let mut content = vec![];
				for (key, value) in value {
					content.push(
						ConcatTokens::new()
							.push_token("\"")
							.push_token(key.as_str())
							.push_token("\"")
							.push_token(":")
							.push_token(FormatToken::Space)
							.push_token(value.format())
							.push_token(",")
							.push_token(LineToken::soft_or_space())
							.format_tokens(),
					);
				}
				ConcatTokens::new()
					.push_token("{")
					.push_token(LineToken::soft())
					.push_token(IndentToken::new(content))
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
	use crate::format_token::IndentToken;

	#[test]
	fn tokenize_numbers() {
		let input = r#"{ "foo": 6 }"#;
		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(IndentToken::new(
				ConcatTokens::new()
					.push_token("\"")
					.push_token("foo")
					.push_token("\"")
					.push_token(":")
					.push_token(FormatToken::Space)
					.push_token(6)
					.push_token(",")
					.format_tokens(),
			))
			.push_token("}")
			.format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_strings() {
		let input = r#"{ "foo": "bar" }"#;
		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(IndentToken::new(
				ConcatTokens::new()
					.push_token("\"")
					.push_token("foo")
					.push_token("\"")
					.push_token(":")
					.push_token(FormatToken::Space)
					.push_token(
						ConcatTokens::new()
							.push_token("\"")
							.push_token("bar")
							.push_token("\"")
							.format_tokens(),
					)
					.push_token(",")
					.format_tokens(),
			))
			.push_token("}")
			.format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_boolean_false() {
		let input = r#"{ "foo": false }"#;
		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(IndentToken::new(
				ConcatTokens::new()
					.push_token("\"")
					.push_token("foo")
					.push_token("\"")
					.push_token(":")
					.push_token(FormatToken::Space)
					.push_token(false)
					.push_token(",")
					.format_tokens(),
			))
			.push_token("}")
			.format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_boolean_true() {
		let input = r#"{ "foo": true }"#;

		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(IndentToken::new(
				ConcatTokens::new()
					.push_token("\"")
					.push_token("foo")
					.push_token("\"")
					.push_token(":")
					.push_token(FormatToken::Space)
					.push_token(true)
					.push_token(",")
					.format_tokens(),
			))
			.push_token("}")
			.format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}
}
