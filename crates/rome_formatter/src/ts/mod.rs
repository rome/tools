mod any_js_array_element;
mod arg_list;
mod assignment_target;
mod auxiliary;
mod bindings;
mod class;
mod declarators;
mod expressions;
mod ident;
mod name;
mod object_members;
mod parameter_list;
mod patterns;
mod script;
mod spread;
mod statements;

#[cfg(test)]
mod test {
	use rslint_parser::parse_text;

	use crate::Formatter;

	#[test]
	fn arrow_function() {
		let src = "let v = (value  , second_value) =>    true";
		let tree = parse_text(src, 0);
		let result = Formatter::default().format_root(&tree.syntax()).unwrap();
		assert_eq!(
			result.code(),
			"let v = (value, second_value) => true;
"
		);
	}

	#[test]
	fn function_block() {
		let src = r#"function foo() { return 'something' }"#;

		let tree = parse_text(src, 0);
		let result = Formatter::default().format_root(&tree.syntax()).unwrap();
		assert_eq!(
			result.code(),
			r#"function foo() {
	return "something";
}
"#
		);
	}

	#[test]
	fn array() {
		let src = r#"let users = [   'john', 'chandler', true ]"#;
		let tree = parse_text(src, 0);
		let result = Formatter::default().format_root(&tree.syntax()).unwrap();
		assert_eq!(
			result.code(),
			r#"let users = ["john", "chandler", true];
"#
		);
	}

	#[test]
	fn poc() {
		let src = r#"let a1 = [{}, {}];
"#;
		let tree = parse_text(src, 0);
		let result = Formatter::default().format_root(&tree.syntax()).unwrap();
		assert_eq!(
			result.code(),
			r#"let a1 = [{}, {}];
"#
		);
	}
}
