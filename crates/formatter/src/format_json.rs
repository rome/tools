use crate::format_token::{GroupToken, LineToken};
use crate::{format_token::FormatToken, FormatValue};
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
				let separator = FormatToken::concat(vec![
					FormatToken::string(","),
					FormatToken::Line(LineToken::soft_or_space()),
				]);

				let properties_list: Vec<FormatToken> = value
					.iter()
					.map(|(key, value)| {
						FormatToken::concat(vec![
							FormatToken::string(format!("\"{}\":", key).as_str()),
							FormatToken::Space,
							value.format(),
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
			Value::Array(_) => todo!("Implement array"),
		}
	}
}

pub fn json_to_tokens(content: &str) -> FormatToken {
	let json: Value = serde_json::from_str(content).expect("cannot convert json to tokens");

	json.format()
}

#[cfg(test)]
mod test {
	use crate::FormatToken;

	use super::json_to_tokens;
	use crate::format_token::{GroupToken, LineToken};

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
	fn tokenize_boolean_null() {
		let result = json_to_tokens("null");

		assert_eq!(FormatToken::string("null"), result);
	}

	#[test]
	fn tokenize_object() {
		let input = r#"{ "foo": "bar", "num": 5 }"#;
		let expected = FormatToken::Group(GroupToken::new(vec![
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
		]));

		let result = json_to_tokens(input);

		assert_eq!(expected, result);
	}
}
