mod any_js_array_element;
mod arg_list;
mod assignment;
mod auxiliary;
mod bindings;
mod class;
mod directives;
mod expressions;
mod ident;
mod import;
mod object_members;
mod parameter_list;
mod root;
mod statements;
mod unknown;

pub(crate) use statements::format_statements;

#[cfg(test)]
mod test {
    use rslint_parser::parse_script;

    use crate::{format, FormatOptions};

    #[test]
    fn arrow_function() {
        let src = "let v = (value  , second_value) =>    true";
        let tree = parse_script(src, 0);
        let result = format(FormatOptions::default(), &tree.syntax()).unwrap();
        assert_eq!(
            result.as_code(),
            "let v = (value, second_value) => true;
"
        );
    }

    #[test]
    fn function_block() {
        let src = r#"function foo() { return 'something' }"#;

        let tree = parse_script(src, 0);
        let result = format(FormatOptions::default(), &tree.syntax()).unwrap();
        assert_eq!(
            result.as_code(),
            r#"function foo() {
	return "something";
}
"#
        );
    }

    #[test]
    fn array() {
        let src = r#"let users = [   'john', 'chandler', true ]"#;
        let tree = parse_script(src, 0);
        let result = format(FormatOptions::default(), &tree.syntax()).unwrap();
        assert_eq!(
            result.as_code(),
            r#"let users = ["john", "chandler", true];
"#
        );
    }

    #[test]
    fn poc() {
        let src = r#"let a1 = [{}, {}];
"#;
        let tree = parse_script(src, 0);
        let result = format(FormatOptions::default(), &tree.syntax()).unwrap();
        assert_eq!(
            result.as_code(),
            r#"let a1 = [{}, {}];
"#
        );
    }
}
