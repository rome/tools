//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyParameter;
impl ToFormatElement for JsAnyParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyFormalParameter(node) => node.format(formatter),
            Self::JsRestParameter(node) => node.format(formatter),
            Self::TsThisParameter(node) => node.format(formatter),
        }
    }
}
