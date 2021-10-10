use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Pattern;

impl ToFormatElement for Pattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		match self {
			Pattern::RestPattern(_) => todo!(),
			Pattern::AssignPattern(pattern) => pattern.to_format_element(formatter),
			Pattern::ObjectPattern(_) => todo!(),
			Pattern::ArrayPattern(_) => todo!(),
			Pattern::ExprPattern(_) => todo!(),
			Pattern::SinglePattern(single) => single.to_format_element(formatter),
		}
	}
}
