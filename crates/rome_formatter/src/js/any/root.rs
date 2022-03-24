//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyRoot;
impl ToFormatElement for JsAnyRoot {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsScript(node) => node.format(formatter),
            Self::JsModule(node) => node.format(formatter),
            Self::JsExpressionSnipped(node) => node.format(formatter),
        }
    }
}
