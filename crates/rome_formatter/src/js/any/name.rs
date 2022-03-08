//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyName;
impl ToFormatElement for JsAnyName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsName(node) => node.to_format_element(formatter),
            Self::JsPrivateName(node) => node.to_format_element(formatter),
        }
    }
}
