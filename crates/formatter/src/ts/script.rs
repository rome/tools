use crate::{
	format_elements, hard_line_break, join_elements, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::Script;

impl ToFormatElement for Script {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let mut tokens = vec![];

		if let Some(shebang) = self.shebang_token() {
			tokens.push(formatter.format_token(&shebang));
			tokens.push(hard_line_break());
		}

		let elements = self.items().map(|item| formatter.format_node(item));

		tokens.push(join_elements(hard_line_break(), elements));

		format_elements![concat_elements(tokens), hard_line_break()]
	}
}
