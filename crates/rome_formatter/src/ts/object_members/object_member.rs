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
            JsAnyObjectMember::JsSpread(spread) => spread.to_format_element(formatter),
            JsAnyObjectMember::JsShorthandPropertyObjectMember(ident) => {
                ident.to_format_element(formatter)
            }
            JsAnyObjectMember::JsMethodObjectMember(method_object_member) => {
                method_object_member.to_format_element(formatter)
            }
            JsAnyObjectMember::JsUnknownMember(_) => todo!(),
        }
    }
}
