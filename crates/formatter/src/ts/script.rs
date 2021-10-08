use crate::{
	concat_elements, hard_line_break, syntax_token, FormatContext, FormatElement, ToFormatElement,
};
use rslint_parser::ast::Script;

impl ToFormatElement for Script {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		let mut tokens = vec![];

		if let Some(shebang) = self.shebang_token() {
			tokens.push(syntax_token(&shebang));
			tokens.push(hard_line_break());
		}

		tokens.extend(self.items().map(|item| context.format_node(item)));

		concat_elements(tokens)
	}
}
