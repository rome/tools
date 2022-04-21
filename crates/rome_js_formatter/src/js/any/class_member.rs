//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyClassMember;
impl Format for JsAnyClassMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsConstructorClassMember(node) => node.format(formatter),
            Self::JsStaticInitializationBlockClassMember(node) => node.format(formatter),
            Self::JsPropertyClassMember(node) => node.format(formatter),
            Self::JsMethodClassMember(node) => node.format(formatter),
            Self::JsGetterClassMember(node) => node.format(formatter),
            Self::JsSetterClassMember(node) => node.format(formatter),
            Self::TsConstructorSignatureClassMember(node) => node.format(formatter),
            Self::TsPropertySignatureClassMember(node) => node.format(formatter),
            Self::TsMethodSignatureClassMember(node) => node.format(formatter),
            Self::TsGetterSignatureClassMember(node) => node.format(formatter),
            Self::TsSetterSignatureClassMember(node) => node.format(formatter),
            Self::TsIndexSignatureClassMember(node) => node.format(formatter),
            Self::JsEmptyClassMember(node) => node.format(formatter),
            Self::JsUnknownMember(node) => node.format(formatter),
        }
    }
}
