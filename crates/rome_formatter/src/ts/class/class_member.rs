use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyClassMember;
use rslint_parser::AstNode;

impl ToFormatElement for JsAnyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyClassMember::JsEmptyClassMember(empty_statement) => {
                empty_statement.to_format_element(formatter)
            }
            JsAnyClassMember::JsMethodClassMember(method) => method.to_format_element(formatter),
            JsAnyClassMember::JsPropertyClassMember(class_prop) => {
                class_prop.to_format_element(formatter)
            }
            JsAnyClassMember::JsConstructorClassMember(constructor) => {
                constructor.to_format_element(formatter)
            }
            JsAnyClassMember::JsGetterClassMember(getter) => getter.to_format_element(formatter),
            JsAnyClassMember::JsSetterClassMember(setter) => setter.to_format_element(formatter),
            JsAnyClassMember::JsUnknownMember(unknown_member) => {
                Ok(formatter.format_verbatim(unknown_member.syntax()))
            }
            JsAnyClassMember::TsIndexSignature(_) => todo!(),
            JsAnyClassMember::JsStaticInitializationBlockClassMember(_) => todo!(),
        }
    }
}
