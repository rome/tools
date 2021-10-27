use rslint_parser::ast::PropName;

use crate::{FormatElement, Formatter, ToFormatElement};

impl ToFormatElement for PropName {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			PropName::ComputedPropertyName(_) => todo!(),
			PropName::Literal(_) => todo!(),
			PropName::Ident(ident) => ident.to_format_element(formatter),
		}
	}
}
