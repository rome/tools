use crate::{format_tokens, FormatToken, FormatValue, GroupToken, LineToken, ListToken};
use syntax::{NodeOrToken, SyntaxKind, SyntaxNode, SyntaxToken};

mod array;
mod arrow_function;
mod function;
mod params;
mod return_statement;
mod statement;
mod string_literal;

pub fn format_syntax_kind(kind: SyntaxKind, node: SyntaxNode) -> FormatToken {
	match kind {
		SyntaxKind::ROOT | SyntaxKind::Program => format_tokens_and_nodes(node),
		SyntaxKind::Array => array::format_node(node),
		// The length of the whitespace is intentionally ignored
		SyntaxKind::Whitespace => FormatToken::Space,
		SyntaxKind::LetToken
		| SyntaxKind::Identifier
		| SyntaxKind::EqToken
		| SyntaxKind::EqEqEqToken
		| SyntaxKind::GtEqToken
		| SyntaxKind::LbraceToken
		| SyntaxKind::PipeRbraceToken
		| SyntaxKind::CommaToken => FormatToken::string(node.text().to_string().as_str()),

		SyntaxKind::StringLiteral => string_literal::format(node),

		SyntaxKind::True | SyntaxKind::False => {
			FormatToken::string(node.text().to_string().as_str())
		}
		SyntaxKind::LexicalDeclaration | SyntaxKind::ExpressionStatement => {
			let result = format_tokens_and_nodes(node);
			// TODO: temporary, here we should check if the last token is a CommaToken an use that, otherwise we use this line
			format_tokens!(result, ";")
		}

		SyntaxKind::VariableDeclarator => format_tokens_and_nodes(node),
		SyntaxKind::ArrowFunction => arrow_function::format(node),
		SyntaxKind::ERROR => format_tokens_and_nodes(node),
		SyntaxKind::ReturnStatement => return_statement::format_node(node),
		SyntaxKind::StatementBlock => {
			// doesn't have any children, so it's an empty block
			if let None = node.first_child() {
				return format_tokens!("{}");
			}
			let group = GroupToken::new(format_tokens!(
				"{",
				FormatToken::indent(format_tokens!(
					LineToken::soft_or_space(),
					ListToken::join(LineToken::soft_or_space(), format_nodes(node))
				)),
				LineToken::soft_or_space(),
				"}"
			));

			FormatToken::from(group)
		}

		SyntaxKind::FormalParameters => params::format(node),

		_ => FormatToken::Space,
	}
}

fn format_syntax_token(token: SyntaxToken) -> FormatToken {
	match token.kind() {
		SyntaxKind::FunctionToken => function::format_token(token),
		SyntaxKind::Whitespace => FormatToken::Space,

		_ => {
			format_tokens!(token.text().to_string().as_str())
		}
	}
}

fn format_tokens_and_nodes(node: SyntaxNode) -> FormatToken {
	FormatToken::from(
		node.children_with_tokens()
			.map(|node_or_token| match node_or_token {
				NodeOrToken::Node(node) => node.format(),
				NodeOrToken::Token(token) => format_syntax_token(token),
			})
			.collect::<Vec<FormatToken>>(),
	)
}

fn format_nodes(node: SyntaxNode) -> Vec<FormatToken> {
	node.children()
		.map(|node| node.format())
		.collect::<Vec<FormatToken>>()
}

impl FormatValue for SyntaxNode {
	fn format(&self) -> crate::FormatToken {
		format_syntax_kind(self.kind(), self.clone())
	}
}

#[cfg(test)]
mod test {
	use crate::{format_token, FormatOptions, FormatValue};
	use syntax::parse;

	#[test]
	fn arrow_function() {
		let src = "let v = (value  , second_value) =>    true";
		let tree = parse(src).unwrap();
		let result = format_token(&tree.format(), FormatOptions::default());
		assert_eq!(result.code(), "let v = (value, second_value) => true;");
	}

	#[test]
	fn function_block() {
		let src = r#"function() foo { return 'something' }"#;
		let tree = parse(src).unwrap();
		let result = format_token(&tree.format(), FormatOptions::default());
		assert_eq!(result.code(), r#"function() foo { return "something"; }"#);
	}

	#[test]
	fn array() {
		let src = r#"let users = [   'john', 'chandler', true ]"#;
		let tree = parse(src).unwrap();
		let result = format_token(&tree.format(), FormatOptions::default());
		assert_eq!(result.code(), r#"let users = ["john", "chandler", true,];"#);
	}

	#[test]
	fn poc() {
		let src = r#"function foo { let var1 = [true, false]
	let broken = [-, 45, 54]
	let var2 = (var1, var2) => {}
}"#;
		let tree = parse(src).unwrap();
		let result = format_token(&tree.format(), FormatOptions::default());
		assert_eq!(
			result.code(),
			r#"function foo {
	let var1 = [true, false,];
	let broken = [-, 45, 54,];
	let var2 = (var1, var2) => {};
}"#
		);
	}
}
