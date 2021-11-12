use crate::{
	empty_element, format_elements, join_elements, space_token, token, FormatElement, FormatResult,
	Formatter, ToFormatElement,
};
use rslint_parser::ast::{
	JsVariableDeclaration, JsVariableDeclarationStatement, JsVariableDeclarator,
};

impl ToFormatElement for JsVariableDeclarationStatement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.declaration()?)?,
			token(";"),
		])
	}
}

impl ToFormatElement for JsVariableDeclaration {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let mut declarators = Vec::with_capacity(self.declarators().len());

		for declarator in self.declarators().iter() {
			declarators.push(formatter.format_node(declarator)?);
		}

		Ok(format_elements![
			formatter.format_token(&self.kind_token()?)?,
			space_token(),
			join_elements(token(","), declarators),
		])
	}
}

impl ToFormatElement for JsVariableDeclarator {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let initializer = if let Some(initializer) = self.init() {
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
