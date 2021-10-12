use crate::ts::statements::format_statements;
use crate::{format_elements, hard_line_break, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Script;

impl ToFormatElement for Script {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let mut elements = vec![];

		if let Some(shebang) = self.shebang_token() {
			elements.push(formatter.format_token(&shebang));
			elements.push(hard_line_break());
		}

		elements.push(format_statements(self.items(), formatter));

		format_elements![concat_elements(elements), hard_line_break()]
	}
}
