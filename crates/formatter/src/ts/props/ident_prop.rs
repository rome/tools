use rslint_parser::ast::IdentProp;

use crate::ToFormatElement;

impl ToFormatElement for IdentProp {
	fn to_format_element(&self, formatter: &crate::Formatter) -> crate::FormatElement {
		formatter.format_node(self.name().unwrap())
	}
}
