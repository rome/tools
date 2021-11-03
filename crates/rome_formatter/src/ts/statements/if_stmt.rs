use crate::{
	format_elements, group_elements, space_token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::IfStmt;

impl ToFormatElement for IfStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let mut result = format_elements![
			group_elements(format_elements![
				formatter.format_token(&self.if_token()?)?,
				space_token(),
				formatter.format_node(self.condition()?)?,
				space_token(),
			]),
			formatter.format_node(self.cons()?)?
		];

		if let Some(else_token) = self.else_token() {
			result = format_elements![
				result,
				space_token(),
				formatter.format_token(&else_token)?,
				space_token(),
				formatter.format_node(self.alt()?)?,
			]
		};

		Some(result)
	}
}
