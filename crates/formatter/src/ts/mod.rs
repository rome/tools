mod arg_list;
mod class;
mod condition;
mod declarators;
mod expr_or_block;
mod expr_or_spread;
mod expressions;
mod getter;
mod ident;
mod method;
mod name;
mod parameter_list;
mod patterns;
mod props;
mod script;
mod setter;
mod spread;
mod statements;
mod tokens;

#[cfg(test)]
mod test {
	use rslint_parser::parse_text;

	use crate::Formatter;

	#[test]
	fn arrow_function() {
		let src = "let v = (value  , second_value) =>    true";
		let tree = parse_text(src, 0);
		let result = Formatter::default().format_root(&tree.syntax());
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
		let result = Formatter::default().format_root(&tree.syntax());
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
		let result = Formatter::default().format_root(&tree.syntax());
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
		let result = Formatter::default().format_root(&tree.syntax());
		assert_eq!(
			result.code(),
			r#"let a1 = [{}, {}];
"#
		);
	}
}
