use crate::format_token::{GroupToken, LineToken};
use crate::{format_token::FormatToken, format_tokens};
use rslint_parser::ast::{
	ArrayExpr, GroupingExpr, Literal, LiteralProp, ObjectExpr, ObjectProp, UnaryExpr,
};
use rslint_parser::{parse_text, AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

fn tokenize_token(token: SyntaxToken) -> FormatToken {
	match token.kind() {
		SyntaxKind::NULL_KW => FormatToken::string("null"),
		SyntaxKind::TRUE_KW => FormatToken::string("true"),
		SyntaxKind::FALSE_KW => FormatToken::string("false"),
		SyntaxKind::STRING => FormatToken::string(token.text().as_str()),
		SyntaxKind::NUMBER => FormatToken::string(token.text().as_str()),
		SyntaxKind::MINUS => FormatToken::string("-"),
		_ => panic!("Unsupported JSON token {:?}", token),
	}
}

fn tokenize_node(node: SyntaxNode) -> FormatToken {
	match node.kind() {
		SyntaxKind::LITERAL => {
			let literal = Literal::cast(node).unwrap();
			tokenize_token(literal.token())
		}
		SyntaxKind::UNARY_EXPR => {
			let expr = UnaryExpr::cast(node).unwrap();
			format_tokens![
				tokenize_token(expr.op_token().unwrap()),
				tokenize_node(expr.expr().unwrap().syntax().clone())
			]
		}

		SyntaxKind::LITERAL_PROP => {
			let prop = LiteralProp::cast(node).unwrap();
			format_tokens![
				tokenize_node(prop.key().unwrap().syntax().clone()),
				":",
				FormatToken::Space,
				tokenize_node(prop.value().unwrap().syntax().clone()),
			]
		}

		SyntaxKind::OBJECT_EXPR => {
			let object = ObjectExpr::cast(node).unwrap();

			let separator = format_tokens![",", LineToken::soft_or_space(),];

			let properties_list: Vec<FormatToken> = object
				.props()
				.map(|prop| match prop {
					ObjectProp::LiteralProp(prop) => {
						format_tokens!(tokenize_node(prop.syntax().clone()))
					}
					_ => panic!("Unsupported prop type {:?}", prop),
				})
				.collect();

			let properties = format_tokens![
				LineToken::soft(),
				FormatToken::join(separator, properties_list),
			];

			FormatToken::Group(GroupToken::new(format_tokens![
				"{",
				FormatToken::indent(properties),
				LineToken::soft(),
				"}",
			]))
		}
		SyntaxKind::ARRAY_EXPR => {
			let array = ArrayExpr::cast(node).unwrap();

			let separator = format_tokens![",", LineToken::soft_or_space(),];

			let elements = format_tokens![
				LineToken::soft(),
				FormatToken::join(
					separator,
					array
						.elements()
						.map(|element| tokenize_node(element.syntax().clone())),
				),
			];

			FormatToken::Group(GroupToken::new(format_tokens![
				"[",
				FormatToken::indent(elements),
				LineToken::soft(),
				"]",
			]))
		}
		_ => panic!("Unsupported JSON kind: {:?}", node.kind()),
	}
}

pub fn tokenize_json(content: &str) -> FormatToken {
	let script = parse_text(format!("({})", content).as_str(), 0);

	// Unwrap the grouping to get to the JSON content. The grouping is only used as a trick to parse JSON
	let json_content = GroupingExpr::cast(
		script
			.syntax()
			.descendants()
			.find(|e| e.kind() == SyntaxKind::GROUPING_EXPR)
			.unwrap(),
	)
	.and_then(|grouping| grouping.inner())
	.unwrap();

	let tokenized_content = tokenize_node(json_content.syntax().clone());
	format_tokens!(tokenized_content, LineToken::hard())
}

#[cfg(test)]
mod test {
	use crate::format_tokens;

	use super::tokenize_json;
	use crate::format_token::{GroupToken, LineToken};

	#[test]
	fn tokenize_number() {
		let result = tokenize_json("6.45");

		assert_eq!(format_tokens!["6.45", LineToken::hard()], result);
	}

	#[test]
	fn tokenize_string() {
		let result = tokenize_json(r#""foo""#);

		assert_eq!(format_tokens![r#""foo""#, LineToken::hard()], result);
	}

	#[test]
	fn tokenize_boolean_false() {
		let result = tokenize_json("false");

		assert_eq!(format_tokens!["false", LineToken::hard()], result);
	}

	#[test]
	fn tokenize_boolean_true() {
		let result = tokenize_json("true");

		assert_eq!(format_tokens!["true", LineToken::hard()], result);
	}

	#[test]
	fn tokenize_boolean_null() {
		let result = tokenize_json("null");

		assert_eq!(format_tokens!["null", LineToken::hard()], result);
	}

	#[test]
	fn tokenize_object() {
		let input = r#"{ "foo": "bar", "num": 5 }"#;
		let expected = format_tokens![
			GroupToken::new(format_tokens![
				"{",
				FormatToken::indent(format_tokens![
					LineToken::soft(),
					"\"foo\"",
					":",
					FormatToken::Space,
					"\"bar\"",
					",",
					LineToken::soft_or_space(),
					"\"num\"",
					":",
					FormatToken::Space,
					"5",
				]),
				LineToken::soft(),
				"}",
			]),
			LineToken::hard(),
		];

		let result = tokenize_json(input);

		assert_eq!(expected, result);
	}

	#[test]
	fn tokenize_array() {
		let input = r#"[ "foo", "bar", 5 ]"#;
		let expected = format_tokens![
			GroupToken::new(format_tokens![
				"[",
				FormatToken::indent(format_tokens![
					LineToken::soft(),
					"\"foo\"",
					",",
					LineToken::soft_or_space(),
					"\"bar\"",
					",",
					LineToken::soft_or_space(),
					"5",
				]),
				LineToken::soft(),
				"]",
			]),
			LineToken::hard(),
		];

		let result = tokenize_json(input);

		assert_eq!(expected, result);
	}
}
