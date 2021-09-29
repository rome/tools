use crate::format_token::{GroupToken, LineToken};
use crate::{format_token::FormatToken, format_tokens, FormatContext, NodeToken};
use rslint_parser::ast::{
	ArrayExpr, GroupingExpr, Literal, LiteralProp, ObjectExpr, ObjectProp, UnaryExpr,
};
use rslint_parser::{parse_text, AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

fn tokenize_token(token: SyntaxToken) -> FormatToken {
	match token.kind() {
		SyntaxKind::NULL_KW
		| SyntaxKind::TRUE_KW
		| SyntaxKind::FALSE_KW
		| SyntaxKind::STRING
		| SyntaxKind::NUMBER
		| SyntaxKind::MINUS => FormatToken::Token(token.into()),
		_ => panic!("Unsupported JSON token {:?}", token),
	}
}

fn tokenize_node(node: SyntaxNode, context: &mut FormatContext) -> FormatToken {
	NodeToken::new(node.green().clone(), tokenize_node_content(node, context)).into()
}

fn tokenize_node_content(node: SyntaxNode, context: &mut FormatContext) -> FormatToken {
	match node.kind() {
		SyntaxKind::LITERAL => {
			let literal = Literal::cast(node).unwrap();
			tokenize_token(literal.token())
		}
		SyntaxKind::UNARY_EXPR => {
			let expr = UnaryExpr::cast(node).unwrap();
			format_tokens![
				tokenize_token(expr.op_token().unwrap()),
				tokenize_node(expr.expr().unwrap().syntax().clone(), context)
			]
		}
		SyntaxKind::LITERAL_PROP => {
			let prop = LiteralProp::cast(node).unwrap();
			format_tokens![
				tokenize_node(prop.key().unwrap().syntax().clone(), context),
				context.tokens.colon(),
				FormatToken::Space,
				tokenize_node(prop.value().unwrap().syntax().clone(), context),
			]
		}
		SyntaxKind::OBJECT_EXPR => {
			let object = ObjectExpr::cast(node).unwrap();

			let separator = format_tokens![context.tokens.comma(), LineToken::soft_or_space(),];

			let properties_list: Vec<FormatToken> = object
				.props()
				.map(|prop| match prop {
					ObjectProp::LiteralProp(prop) => tokenize_node(prop.syntax().clone(), context),
					_ => panic!("Unsupported prop type {:?}", prop),
				})
				.collect();

			let properties = format_tokens![
				LineToken::soft(),
				FormatToken::join(separator, properties_list),
			];

			FormatToken::Group(GroupToken::new(format_tokens![
				context.tokens.left_brace(),
				FormatToken::indent(properties),
				LineToken::soft(),
				context.tokens.right_brace(),
			]))
		}
		SyntaxKind::ARRAY_EXPR => {
			let array = ArrayExpr::cast(node).unwrap();

			let separator = format_tokens![context.tokens.comma(), LineToken::soft_or_space()];

			let elements = format_tokens![
				LineToken::soft(),
				FormatToken::join(
					separator,
					array
						.elements()
						.map(|element| tokenize_node(element.syntax().clone(), context)),
				),
			];

			FormatToken::Group(GroupToken::new(format_tokens![
				context.tokens.left_bracket(),
				FormatToken::indent(elements),
				LineToken::soft(),
				context.tokens.right_bracket(),
			]))
		}
		_ => panic!("Unsupported JSON kind: {:?}", node.kind()),
	}
}

pub fn tokenize_json(content: &str) -> FormatToken {
	let script = parse_text(format!("({})", content).as_str(), 0);

	let grouping = GroupingExpr::cast(
		script
			.syntax()
			.descendants()
			.find(|e| e.kind() == SyntaxKind::GROUPING_EXPR)
			.unwrap(),
	)
	.unwrap();
	let json_content = grouping.inner().unwrap();

	let mut context = FormatContext::default();
	NodeToken::new(
		json_content.syntax().green().clone(),
		format_tokens![
			tokenize_node_content(json_content.syntax().clone(), &mut context),
			LineToken::hard(),
		],
	)
	.into()
}

#[cfg(test)]
mod test {
	use crate::{format_tokens, FormatToken, NodeToken, Tokens};
	use rslint_parser::SyntaxKind;
	use rslint_rowan::{GreenNode, GreenToken, NodeOrToken};

	use super::tokenize_json;
	use crate::format_token::LineToken;

	fn create_green_node<I>(kind: SyntaxKind, children: I) -> GreenNode
	where
		I: IntoIterator<Item = NodeOrToken<GreenNode, GreenToken>>,
		I::IntoIter: ExactSizeIterator,
	{
		GreenNode::new(rslint_rowan::SyntaxKind(kind.into()), children)
	}

	#[test]
	fn tokenize_number() {
		let mut tokens = Tokens::default();

		let token = tokens.get(SyntaxKind::NUMBER, "6.45");
		let literal =
			create_green_node(SyntaxKind::LITERAL, vec![NodeOrToken::Token(token.clone())]);

		assert_eq!(
			FormatToken::Node(NodeToken::new(
				literal,
				format_tokens![token, LineToken::hard()]
			)),
			tokenize_json("6.45")
		);
	}

	#[test]
	fn tokenize_string() {
		let mut tokens = Tokens::default();

		let token = tokens.double_quoted_string(r#"foo"#);
		let literal =
			create_green_node(SyntaxKind::LITERAL, vec![NodeOrToken::Token(token.clone())]);

		assert_eq!(
			FormatToken::Node(NodeToken::new(
				literal,
				format_tokens![token, LineToken::hard()]
			)),
			tokenize_json(r#""foo""#)
		);
	}

	#[test]
	fn tokenize_boolean_false() {
		let mut tokens = Tokens::default();

		let token = tokens.get(SyntaxKind::FALSE_KW, "false");
		let literal =
			create_green_node(SyntaxKind::LITERAL, vec![NodeOrToken::Token(token.clone())]);

		assert_eq!(
			FormatToken::Node(NodeToken::new(
				literal,
				format_tokens![token, LineToken::hard()]
			)),
			tokenize_json("false")
		);
	}

	#[test]
	fn tokenize_boolean_true() {
		let mut tokens = Tokens::default();

		let token = tokens.get(SyntaxKind::TRUE_KW, "true");
		let literal =
			create_green_node(SyntaxKind::LITERAL, vec![NodeOrToken::Token(token.clone())]);

		assert_eq!(
			FormatToken::Node(NodeToken::new(
				literal,
				format_tokens![token, LineToken::hard()]
			)),
			tokenize_json("true")
		);
	}

	#[test]
	fn tokenize_null() {
		let mut tokens = Tokens::default();

		let token = tokens.get(SyntaxKind::NULL_KW, "null");
		let literal =
			create_green_node(SyntaxKind::LITERAL, vec![NodeOrToken::Token(token.clone())]);

		assert_eq!(
			FormatToken::Node(NodeToken::new(
				literal,
				format_tokens![token, LineToken::hard()]
			)),
			tokenize_json("null")
		);
	}
}
