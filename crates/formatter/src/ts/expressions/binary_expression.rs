use crate::{
	FormatElement,
	ToFormatElement,
};
use rslint_parser::ast::BinExpr;

impl ToFormatElement for BinExpr {
	fn to_format_element(&self, _formatter: &crate::Formatter) -> FormatElement {
		// TODO: to implement
		FormatElement::Empty
	}
}
