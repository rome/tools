use crate::format_token::{GroupToken, LineToken};
use crate::{format_token::FormatToken, TokenizeValue};
use serde_json::Value;

impl TokenizeValue for Value {
	fn tokenize(&self) -> FormatToken {
		match self {
			Value::String(string) => {
				FormatToken::string(format!("\"{}\"", escape_string(string)).as_str())
			}
			Value::Number(number) => {
				let number = number.as_f64().unwrap();
				FormatToken::f64(number)
			}
			Value::Bool(value) => FormatToken::from(value),
			Value::Object(value) => {
				let separator = FormatToken::concat(vec![
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
				]);

				let properties_list: Vec<FormatToken> = value
					.iter()
					.map(|(key, value)| {
						FormatToken::concat(vec![
							FormatToken::string(format!("\"{}\":", escape_string(key)).as_str()),
							FormatToken::Space,
							value.tokenize(),
						])
					})
					.collect();

				let properties = vec![
					FormatToken::Line(LineToken::soft()),
					FormatToken::join(separator, properties_list),
				];

				FormatToken::Group(GroupToken::new(vec![
					FormatToken::string("{"),
					FormatToken::indent(properties),
					FormatToken::Line(LineToken::soft()),
					FormatToken::string("}"),
				]))
			}
			Value::Null => FormatToken::string("null"),
			Value::Array(array) => {
				let separator = FormatToken::concat(vec![
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
				]);

				let elements = vec![
					FormatToken::Line(LineToken::soft()),
					FormatToken::join(separator, array.iter().map(|element| element.tokenize())),
				];

				FormatToken::Group(GroupToken::new(vec![
					FormatToken::string("["),
					FormatToken::indent(elements),
					FormatToken::Line(LineToken::soft()),
					FormatToken::string("]"),
				]))
			}
		}
	}
}

fn escape_string(string: &str) -> String {
	string
		.replace("\\", "\\\\")
		.replace('"', "\\\"")
		.replace('\r', "\\r")
		.replace('\t', "\\t")
		.replace('\n', "\\n")
}

pub fn json_to_tokens(content: &str) -> FormatToken {
	let json: Value = serde_json::from_str(content).expect("cannot convert json to tokens");

	FormatToken::from(ListToken::concat(vec![
		json.tokenize(),
		FormatToken::from(LineToken::hard()),
	]))
}

#[cfg(test)]
mod test {
	use crate::{FormatToken, ListToken};

	use super::json_to_tokens;
	use crate::format_token::{GroupToken, LineToken};

	#[test]
	fn tokenize_number() {
		let result = json_to_tokens("6.45");

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string("6.45"),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_string() {
		let result = json_to_tokens(r#""foo""#);

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string(r#""foo""#),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_boolean_false() {
		let result = json_to_tokens("false");

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string("false"),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_boolean_true() {
		let result = json_to_tokens("true");

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string("true"),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_boolean_null() {
		let result = json_to_tokens("null");

		assert_eq!(
			FormatToken::List(ListToken::concat(vec![
				FormatToken::string("null"),
				FormatToken::Line(LineToken::hard())
			])),
			result
		);
	}

	#[test]
	fn tokenize_object() {
		let input = r#"{ "foo": "bar", "num": 5 }"#;
		let expected = FormatToken::List(ListToken::concat(vec![
			FormatToken::Group(GroupToken::new(vec![
				FormatToken::string("{"),
				FormatToken::indent(FormatToken::concat(vec![
					FormatToken::Line(LineToken::soft()),
					FormatToken::string("\"foo\":"),
					FormatToken::Space,
					FormatToken::string("\"bar\""),
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
					FormatToken::string("\"num\":"),
					FormatToken::Space,
					FormatToken::string("5"),
				])),
				FormatToken::Line(LineToken::soft()),
				FormatToken::string("}"),
			])),
			FormatToken::Line(LineToken::hard()),
		]));

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_array() {
		let input = r#"[ "foo", "bar", 5 ]"#;
		let expected = FormatToken::List(ListToken::concat(vec![
			FormatToken::Group(GroupToken::new(vec![
				FormatToken::string("["),
				FormatToken::indent(FormatToken::concat(vec![
					FormatToken::Line(LineToken::soft()),
					FormatToken::string("\"foo\""),
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
					FormatToken::string("\"bar\""),
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
					FormatToken::string("5"),
				])),
				FormatToken::Line(LineToken::soft()),
				FormatToken::string("]"),
			])),
			FormatToken::Line(LineToken::hard()),
		]));

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}
}
