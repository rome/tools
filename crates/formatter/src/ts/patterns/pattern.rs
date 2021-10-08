use crate::{FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::Pattern;

impl ToFormatElement for Pattern {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		match self {
			Pattern::RestPattern(_) => todo!(),
			Pattern::AssignPattern(pattern) => pattern.to_format_element(context),
			Pattern::ObjectPattern(_) => todo!(),
			Pattern::ArrayPattern(_) => todo!(),
			Pattern::ExprPattern(_) => todo!(),
			Pattern::SinglePattern(single) => single.to_format_element(context),
		}
	}
}
