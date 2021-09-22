use crate::{
	format_tokens::{FormatTokens, Tokens},
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
				let mut final_tokens: Tokens = vec!["{".into()];
				let mut content = vec![];
				for (key, value) in value {
					content.push(FormatTokens::concat([
						"\"",
						key.as_str(),
						"\"",
						":",
						FormatTokens::Space,
						value.format(),
						",",
					]));
				}
				final_tokens.push(FormatTokens::indent(FormatTokens::from(content)));
				final_tokens.push("}".into());
				FormatTokens::from(final_tokens)
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
	use crate::FormatTokens;

	use super::json_to_tokens;

	#[test]
	fn tokenize_numbers() {
		let input = r#"{ "foo": 6 }"#;
		let expected = FormatTokens::concat([
			"{".into(),
			FormatTokens::indent(FormatTokens::from(vec![
				"\"".into(),
				"foo".into(),
				"\"".into(),
				":".into(),
				FormatTokens::Space,
				6.into(),
				",".into(),
			])),
			"}".into(),
		]);

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_strings() {
		let input = r#"{ "foo": "bar" }"#;
		let expected = FormatTokens::concat([
			"{".into(),
			FormatTokens::indent(FormatTokens::from(vec![
				"\"".into(),
				"foo".into(),
				"\"".into(),
				":".into(),
				FormatTokens::Space,
				FormatTokens::concat(vec!["\"".into(), "bar".into(), "\"".into()]),
				",".into(),
			])),
			"}".into(),
		]);

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_boolean_false() {
		let input = r#"{ "foo": false }"#;
		let expected = FormatTokens::concat([
			"{".into(),
			FormatTokens::indent(FormatTokens::from(vec![
				"\"".into(),
				"foo".into(),
				"\"".into(),
				":".into(),
				FormatTokens::Space,
				false.into(),
				",".into(),
			])),
			"}".into(),
		]);

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_boolean_true() {
		let input = r#"{ "foo": true }"#;
		let expected = FormatTokens::concat([
			"{".into(),
			FormatTokens::indent(FormatTokens::from(vec![
				"\"".into(),
				"foo".into(),
				"\"".into(),
				":".into(),
				FormatTokens::Space,
				true.into(),
				",".into(),
			])),
			"}".into(),
		]);

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}
}
