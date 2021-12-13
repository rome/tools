use crate::{
	empty_element, format_elements, join_elements, space_token, token, FormatElement, FormatResult,
	Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsVariableDeclaration, JsVariableDeclarationList, JsVariableStatement};

impl ToFormatElement for JsVariableStatement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.declaration_list()?)?,
			token(";"),
		])
	}
}

impl ToFormatElement for JsVariableDeclarationList {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.kind_token()?)?,
			space_token(),
			join_elements(
				space_token(),
				// TODO #1726 break multiple declarations across multiple lines if exceeding line width
				formatter.format_separated(self.declarations())?
			),
		])
	}
}

impl ToFormatElement for JsVariableDeclaration {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let initializer = if let Some(initializer) = self.initializer() {
			format_elements![space_token(), formatter.format_node(initializer)?]
		} else {
			empty_element()
		};

		Ok(format_elements![
			formatter.format_node(self.id()?)?,
			initializer
		])
	}
}
