use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyObjectMember;

impl ToFormatElement for JsAnyObjectMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyObjectMember::LiteralProp(literal_prop) => {
				literal_prop.to_format_element(formatter)
			}
			JsAnyObjectMember::Getter(getter) => getter.to_format_element(formatter),
			JsAnyObjectMember::Setter(setter) => setter.to_format_element(formatter),
			JsAnyObjectMember::SpreadProp(_) => todo!(),
			JsAnyObjectMember::InitializedProp(_) => todo!(),
			JsAnyObjectMember::JsShorthandPropertyObjectMember(ident) => {
				ident.to_format_element(formatter)
			}
			JsAnyObjectMember::Method(_) => todo!(),
			JsAnyObjectMember::JsUnknownMember(_) => todo!(),
		}
	}
}
