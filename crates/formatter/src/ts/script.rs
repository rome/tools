use crate::{concat_elements, hard_line_break, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Script;

impl ToFormatElement for Script {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let mut tokens = vec![];

		if let Some(shebang) = self.shebang_token() {
			tokens.push(formatter.format_token(&shebang));
			tokens.push(hard_line_break());
		}

		tokens.extend(self.items().map(|item| formatter.format_node(item)));

		concat_elements(tokens)
	}
}
