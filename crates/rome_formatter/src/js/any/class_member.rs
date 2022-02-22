//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyClassMember;
impl ToFormatElement for JsAnyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsConstructorClassMember(node) => node.to_format_element(formatter),
            Self::JsStaticInitializationBlockClassMember(node) => node.to_format_element(formatter),
            Self::JsPropertyClassMember(node) => node.to_format_element(formatter),
            Self::JsMethodClassMember(node) => node.to_format_element(formatter),
            Self::JsGetterClassMember(node) => node.to_format_element(formatter),
            Self::JsSetterClassMember(node) => node.to_format_element(formatter),
            Self::JsEmptyClassMember(node) => node.to_format_element(formatter),
            Self::TsIndexSignatureClassMember(node) => node.to_format_element(formatter),
            Self::JsUnknownMember(node) => node.to_format_element(formatter),
        }
    }
}
