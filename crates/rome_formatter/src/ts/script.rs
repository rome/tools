use crate::ts::statements::format_statements;
use crate::{
	format_elements, hard_line_break, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsScript;

impl ToFormatElement for JsScript {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let mut elements = vec![];

		if let Some(interpreter) = self.interpreter_token() {
			elements.push(formatter.format_token(&interpreter)?);
			elements.push(hard_line_break());
		}

		elements.push(format_statements(self.statements(), formatter));

		Ok(format_elements![
			concat_elements(elements),
			hard_line_break()
		])
	}
}
