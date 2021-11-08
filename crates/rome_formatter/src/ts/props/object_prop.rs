use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::ObjectProp;

impl ToFormatElement for ObjectProp {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			ObjectProp::LiteralProp(literal_prop) => literal_prop.to_format_element(formatter),
			ObjectProp::Getter(getter) => getter.to_format_element(formatter),
			ObjectProp::Setter(setter) => setter.to_format_element(formatter),
			ObjectProp::SpreadProp(_) => todo!(),
			ObjectProp::InitializedProp(_) => todo!(),
			ObjectProp::IdentProp(ident) => ident.to_format_element(formatter),
			ObjectProp::Method(_) => todo!(),
			ObjectProp::JsUnknownMember(_) => todo!(),
		}
	}
}
