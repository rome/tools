//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyClassMemberName;
impl ToFormatElement for JsAnyClassMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsLiteralMemberName(node) => node.to_format_element(formatter),
            Self::JsComputedMemberName(node) => node.to_format_element(formatter),
            Self::JsPrivateClassMemberName(node) => node.to_format_element(formatter),
        }
    }
}
