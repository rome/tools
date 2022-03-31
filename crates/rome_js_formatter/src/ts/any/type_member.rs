//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyTypeMember;
impl ToFormatElement for TsAnyTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsCallSignatureTypeMember(node) => node.to_format_element(formatter),
            Self::TsPropertySignatureTypeMember(node) => node.to_format_element(formatter),
            Self::TsConstructSignatureTypeMember(node) => node.to_format_element(formatter),
            Self::TsMethodSignatureTypeMember(node) => node.to_format_element(formatter),
            Self::TsGetterSignatureTypeMember(node) => node.to_format_element(formatter),
            Self::TsSetterSignatureTypeMember(node) => node.to_format_element(formatter),
            Self::TsIndexSignatureTypeMember(node) => node.to_format_element(formatter),
            Self::JsUnknownMember(node) => node.to_format_element(formatter),
        }
    }
}
