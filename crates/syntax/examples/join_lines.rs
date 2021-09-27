use rowan::NodeOrToken;
use syntax::SyntaxNode;
use syntax::{hacky, parse};

fn main() {
	let src = "function foo() {\n    console.log(2+2);\n};";

	let root = parse(&src).unwrap();
	let root = root.clone_for_update();
	println!("Before:\n{}", root.to_string());
	join_lines(root.clone());
	println!("After:\n{}", root.to_string());
}

// Obviously not a real implementation
fn join_lines(node: SyntaxNode) {
	for child in node.children_with_tokens() {
		match child {
			NodeOrToken::Node(n) => join_lines(n),
			NodeOrToken::Token(token) => {
				if token.kind().is_whitespace() && token.text().contains("\n") {
					hacky::replace_with_whitespace(token, " ");
				}
			}
		}
	}
}
