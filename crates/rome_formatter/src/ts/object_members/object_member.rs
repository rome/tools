use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyObjectMember;

impl ToFormatElement for JsAnyObjectMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyObjectMember::JsPropertyObjectMember(member) => {
				member.to_format_element(formatter)
			}
			JsAnyObjectMember::JsGetterObjectMember(getter) => getter.to_format_element(formatter),
			JsAnyObjectMember::JsSetterObjectMember(setter) => setter.to_format_element(formatter),
			JsAnyObjectMember::SpreadProp(_) => todo!(),
			JsAnyObjectMember::InitializedProp(_) => todo!(),
			JsAnyObjectMember::JsShorthandPropertyObjectMember(ident) => {
				ident.to_format_element(formatter)
			}
			JsAnyObjectMember::JsMethodObjectMember(_) => todo!(),
			JsAnyObjectMember::JsUnknownMember(_) => todo!(),
		}
	}
}
