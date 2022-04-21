//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::TsAnyTypeMember;
impl Format for TsAnyTypeMember {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsCallSignatureTypeMember(node) => node.format(formatter),
            Self::TsPropertySignatureTypeMember(node) => node.format(formatter),
            Self::TsConstructSignatureTypeMember(node) => node.format(formatter),
            Self::TsMethodSignatureTypeMember(node) => node.format(formatter),
            Self::TsGetterSignatureTypeMember(node) => node.format(formatter),
            Self::TsSetterSignatureTypeMember(node) => node.format(formatter),
            Self::TsIndexSignatureTypeMember(node) => node.format(formatter),
            Self::JsUnknownMember(node) => node.format(formatter),
        }
    }
}
