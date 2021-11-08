use crate::{
	format_elements, group_elements, space_token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::IfStmt;

impl ToFormatElement for IfStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let mut result = format_elements![
			group_elements(format_elements![
				formatter.format_token(&self.if_token()?)?,
				space_token(),
				formatter.format_node(self.condition()?)?,
				space_token(),
			]),
			// TODO: #1725 this will change when we will review the grammar
			formatter.format_node(self.cons().unwrap())?
		];

		if let Ok(else_token) = self.else_token() {
			if let Some(alt) = self.alt() {
				result = format_elements![
					result,
					space_token(),
					formatter.format_token(&else_token)?,
					space_token(),
					formatter.format_node(alt)?,
				]
			}
		};

		Ok(result)
	}
}
