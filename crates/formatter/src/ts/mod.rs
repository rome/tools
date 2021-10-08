mod declarators;
mod expr_or_block;
mod expr_or_spread;
mod expressions;
mod name;
mod parameter_list;
mod patterns;
mod script;
mod spread;
mod statements;

#[cfg(test)]
mod test {
	use rslint_parser::{ast::Script, parse_text, AstNode};

	use crate::{format_element, FormatContext, FormatOptions};

	#[test]
	fn arrow_function() {
		let src = "let v = (value  , second_value) =>    true";
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let element = FormatContext::default().format_root(child.syntax());

		let result = format_element(&element, FormatOptions::default());
		assert_eq!(result.code(), "let v = (value, second_value) => true;");
	}

	#[test]
	fn function_block() {
		let src = r#"function foo() { return 'something' }"#;
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let element = FormatContext::default().format_root(child.syntax());
		let result = format_element(&element, FormatOptions::default());
		assert_eq!(result.code(), r#"function foo() {return "something";}"#);
	}

	#[test]
	fn array() {
		let src = r#"let users = [   'john', 'chandler', true ]"#;
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let element = FormatContext::default().format_root(child.syntax());
		let result = format_element(&element, FormatOptions::default());
		assert_eq!(result.code(), r#"let users = ["john", "chandler", true];"#);
	}

	#[test]
	fn poc() {
		let src = r#"function foo() { let var1 = [true, false]
	let broken = [   45, 54]
	let var2 = (var1, var2) => {}
}"#;
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let element = FormatContext::default().format_root(child.syntax());
		let result = format_element(&element, FormatOptions::default());
		assert_eq!(
			result.code(),
			r#"function foo() {
	let var1 = [true, false];
	let broken = [45, 54];
	let var2 = (var1, var2) => {};
}"#
		);
	}
}
