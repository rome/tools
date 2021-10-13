use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::ObjectProp;

impl ToFormatElement for ObjectProp {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		match self {
			ObjectProp::LiteralProp(_) => todo!(),
			ObjectProp::Getter(_) => todo!(),
			ObjectProp::Setter(_) => todo!(),
			ObjectProp::SpreadProp(_) => todo!(),
			ObjectProp::InitializedProp(_) => todo!(),
			ObjectProp::IdentProp(ident) => ident.to_format_element(formatter),
			ObjectProp::Method(_) => todo!(),
		}
	}
}
