//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyObjectMemberName;
impl ToFormatElement for JsAnyObjectMemberName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsLiteralMemberName(node) => node.to_format_element(formatter),
            Self::JsComputedMemberName(node) => node.to_format_element(formatter),
        }
    }
}
