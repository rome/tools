use crate::{
	block_indent, empty_element, format_elements, group_elements, hard_line_break, join_elements,
	space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsClassDeclaration;

impl ToFormatElement for JsClassDeclaration {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let extends = if let Some(extends_clause) = self.extends_clause() {
			format_elements![space_token(), formatter.format_node(extends_clause)?]
		} else {
			empty_element()
		};

		Ok(format_elements![
			formatter.format_token(&self.class_token()?)?,
			space_token(),
			formatter.format_node(self.id()?)?,
			extends,
			space_token(),
			group_elements(format_elements![
				formatter.format_token(&self.l_curly_token()?)?,
				block_indent(join_elements(
					hard_line_break(),
					formatter.format_nodes(self.members())?
				)),
				formatter.format_token(&self.r_curly_token()?)?
			])
		])
	}
}
