//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyObjectTypeMember;
impl ToFormatElement for TsAnyObjectTypeMember {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsCallSignatureObjectTypeMember(node) => node.to_format_element(formatter),
            Self::TsPropertySignatureObjectTypeMember(node) => node.to_format_element(formatter),
            Self::TsConstructSignatureObjectTypeMember(node) => node.to_format_element(formatter),
            Self::TsMethodSignatureObjectTypeMember(node) => node.to_format_element(formatter),
            Self::TsGetterSignatureObjectTypeMember(node) => node.to_format_element(formatter),
            Self::TsSetterSignatureObjectTypeMember(node) => node.to_format_element(formatter),
            Self::TsIndexSignatureObjectTypeMember(node) => node.to_format_element(formatter),
        }
    }
}
