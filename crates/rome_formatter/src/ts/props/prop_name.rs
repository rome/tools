use rslint_parser::ast::PropName;

use crate::{FormatElement, FormatError, Formatter, ToFormatElement};

impl ToFormatElement for PropName {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		match self {
			PropName::ComputedPropertyName(_) => todo!(),
			PropName::Literal(literal) => literal.to_format_element(formatter),
			PropName::Ident(ident) => ident.to_format_element(formatter),
			PropName::Name(name) => name.to_format_element(formatter),
		}
	}
}
