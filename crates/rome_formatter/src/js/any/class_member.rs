//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyClassMember;
impl ToFormatElement for JsAnyClassMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsConstructorClassMember(node) => node.to_format_element(formatter),
            Self::JsStaticInitializationBlockClassMember(node) => node.to_format_element(formatter),
            Self::JsPropertyClassMember(node) => node.to_format_element(formatter),
            Self::JsMethodClassMember(node) => node.to_format_element(formatter),
            Self::JsGetterClassMember(node) => node.to_format_element(formatter),
            Self::JsSetterClassMember(node) => node.to_format_element(formatter),
            Self::TsConstructorSignatureClassMember(node) => node.to_format_element(formatter),
            Self::TsPropertySignatureClassMember(node) => node.to_format_element(formatter),
            Self::TsMethodSignatureClassMember(node) => node.to_format_element(formatter),
            Self::TsGetterSignatureClassMember(node) => node.to_format_element(formatter),
            Self::TsSetterSignatureClassMember(node) => node.to_format_element(formatter),
            Self::TsIndexSignatureClassMember(node) => node.to_format_element(formatter),
            Self::JsEmptyClassMember(node) => node.to_format_element(formatter),
            Self::JsUnknownMember(node) => node.to_format_element(formatter),
        }
    }
}
