use crate::{
	format_tokens::{ConcatTokens, FormatTokens, Tokens},
	FormatValue,
};
use serde_json::Value;

impl FormatValue for Value {
	fn format(&self) -> FormatTokens {
		match self {
			Value::String(string) => {
				FormatTokens::from(vec!["\"".into(), string.as_str().into(), "\"".into()])
			}
			Value::Number(number) => {
				let number = number.as_u64().unwrap();
				FormatTokens::from(number)
			}
			Value::Bool(value) => FormatTokens::from(value),
			Value::Object(value) => {
				let mut content = vec![];
				for (key, value) in value {
					content.push(
						ConcatTokens::new()
							.push_token("\"")
							.push_token(key.as_str())
							.push_token("\"")
							.push_token(":")
							.push_token(FormatTokens::Space)
							.push_token(value.format())
							.push_token(",")
							// .push_token(FormatTokens::hardline())
							.to_format_tokens(),
					);
				}
				let tokens = ConcatTokens::new()
					.push_token("{")
					.push_token(FormatTokens::indent(FormatTokens::from(content)))
					.push_token("}")
					.to_format_tokens();

				FormatTokens::from(tokens)
			}
			_ => unimplemented!("Implement rest"),
		}
	}
}

pub fn json_to_tokens(content: &str) -> FormatTokens {
	let json: Value = serde_json::from_str(content).unwrap();

	json.format()
}

#[cfg(test)]
mod test {
	use crate::{format_tokens::ConcatTokens, FormatTokens};

	use super::json_to_tokens;

	#[test]
	fn tokenize_numbers() {
		let input = r#"{ "foo": 6 }"#;
		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(FormatTokens::indent(
				ConcatTokens::new()
					.push_token("\"")
					.push_token("foo")
					.push_token("\"")
					.push_token(":")
					.push_token(FormatTokens::Space)
					.push_token(6)
					.push_token(",")
					.to_format_tokens(),
			))
			.push_token("}")
			.to_format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_strings() {
		let input = r#"{ "foo": "bar" }"#;
		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(FormatTokens::indent(
				ConcatTokens::new()
					.push_token("\"")
					.push_token("foo")
					.push_token("\"")
					.push_token(":")
					.push_token(FormatTokens::Space)
					.push_token(
						ConcatTokens::with_capacity(3)
							.push_token("\"")
							.push_token("bar")
							.push_token("\"")
							.to_format_tokens(),
					)
					.push_token(",")
					.to_format_tokens(),
			))
			.push_token("}")
			.to_format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_boolean_false() {
		let input = r#"{ "foo": false }"#;
		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(FormatTokens::indent(
				ConcatTokens::new()
					.push_token("\"")
					.push_token("foo")
					.push_token("\"")
					.push_token(":")
					.push_token(FormatTokens::Space)
					.push_token(false)
					.push_token(",")
					.to_format_tokens(),
			))
			.push_token("}")
			.to_format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_boolean_true() {
		let input = r#"{ "foo": true }"#;

		let expected = ConcatTokens::new()
			.push_token("{")
			.push_token(FormatTokens::indent(
				ConcatTokens::new()
					.push_token("\"")
					.push_token("foo")
					.push_token("\"")
					.push_token(":")
					.push_token(FormatTokens::Space)
					.push_token(true)
					.push_token(",")
					.to_format_tokens(),
			))
			.push_token("}")
			.to_format_tokens();

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}
}
