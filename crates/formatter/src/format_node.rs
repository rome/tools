use crate::token::{LineMode, Token, Tokens};
use serde_json::Value;

pub trait FormatValue {
	fn format(&self) -> Token;
}

impl FormatValue for Value {
	fn format(&self) -> Token {
		match self {
			Value::String(string) => Token::string(format!(r#""{}""#, string).as_str()),
			Value::Number(number) => {
				let number = number.as_u64().unwrap();
				Token::from(number)
			}
			Value::Bool(value) => Token::from(value),
			Value::Object(value) => {
				let mut final_tokens: Tokens = vec![];
				final_tokens.push("{".into());
				let mut content = vec![];
				for (key, value) in value {
					let tokens: Tokens = vec![
						"\"".into(),
						key.as_str().into(),
						"\"".into(),
						":".into(),
						Token::Space,
						value.format(),
						",".into(),
						Token::Break,
					];
					content.push(Token::concat(tokens))
				}
				final_tokens.push(Token::indent(Token::from(content)));
				final_tokens.push("}".into());
				Token::from(final_tokens)
			}
			_ => unimplemented!("Implement rest"),
		}
	}
}
