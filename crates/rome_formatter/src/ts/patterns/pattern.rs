use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::Pattern;

impl ToFormatElement for Pattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			Pattern::RestPattern(_) => todo!(),
			Pattern::AssignPattern(pattern) => pattern.to_format_element(formatter),
			Pattern::ObjectPattern(_) => todo!(),
			Pattern::ArrayPattern(array_pattern) => array_pattern.to_format_element(formatter),
			Pattern::ExprPattern(_) => todo!(),
			Pattern::SinglePattern(single) => single.to_format_element(formatter),
		}
	}
}
