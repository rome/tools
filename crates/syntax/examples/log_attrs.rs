use rowan::WalkEvent;
use syntax::parse;
use syntax::{
	ast::{self, AstNode},
	SyntaxNode,
};

fn main() {
	let src = "const foo = <Foo a={1} b={2}>Hi</Foo>;";

	let tree = parse(src).unwrap();
	for event in tree.preorder() {
		match event {
			WalkEvent::Enter(n) => {
				log_attrs(n);
			}
			_ => {}
		}
	}

	dbg!(tree.to_string());
}

fn log_attrs(n: SyntaxNode) -> Option<()> {
	let elem = ast::JsxElement::cast(n)?;
	let open = elem.open_tag()?;
	for attr in open.attribute() {
		dbg!(attr);
	}
	None
}
