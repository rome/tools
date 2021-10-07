use crate::{FormatToken, FormatValue};

mod array;
mod arrow_function;
mod declarators;
mod expressions;
mod literal;
mod name;
mod name_ref;
mod paramter_list;
mod script;
mod single_pattern;
mod spread;
mod statements;
mod var_decl;

// NOTE: please leave this comment here, it will be removed once this logic will be ported to the proper place
// TODO: remove once logic is implemented somewhere else
// 		let group = GroupToken::new(format_tokens!(
// 			"{",
// 			FormatToken::indent(format_tokens!(
// 				LineToken::soft_or_space(),
// 				ListToken::join(LineToken::soft_or_space(), format_nodes(node))
// 			)),
// 			LineToken::soft_or_space(),
// 			"}"
// 		));

// 		FormatToken::from(group)

#[cfg(test)]
mod test {
	use rslint_parser::{ast::Script, parse_text, AstNode, Parser};

	use crate::{format_token, FormatOptions, FormatValue};

	#[test]
	fn arrow_function() {
		let src = "let v = (value  , second_value) =>    true";
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let result = format_token(&child.format(), FormatOptions::default());
		assert_eq!(result.code(), "let v = (value, second_value) => true;");
	}

	#[test]
	fn function_block() {
		let src = r#"function foo() { return 'something' }"#;
		let tree = parse_text(src, 0);
		let child = Script::cast(tree.syntax()).unwrap();
		let result = format_token(&child.format(), FormatOptions::default());
		assert_eq!(result.code(), r#"function foo() { return "something"; }"#);
	}

	#[test]
	fn array() {
		let src = r#"let users = [   'john', 'chandler', true ]"#;
		let tree = parse_text(src, 0);
		// let result = format_token(&tree.format(), FormatOptions::default());
		// assert_eq!(result.code(), r#"let users = ["john", "chandler", true,];"#);
	}

	#[test]
	fn poc() {
		let src = r#"function foo { let var1 = [true, false]
	let broken = [-, 45, 54]
	let var2 = (var1, var2) => {}
}"#;
		let tree = parse_text(src, 0);
		// 		let result = format_token(&tree.format(), FormatOptions::default());
		// 		assert_eq!(
		// 			result.code(),
		// 			r#"function foo {
		// 	let var1 = [true, false,];
		// 	let broken = [-, 45, 54,];
		// 	let var2 = (var1, var2) => {};
		// }"#
		// 		);
	}
}
