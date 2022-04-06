//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyFunctionBody;
impl ToFormatElement for JsAnyFunctionBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyExpression(node) => node.to_format_element(formatter),
            Self::JsFunctionBody(node) => node.to_format_element(formatter),
        }
    }
}
