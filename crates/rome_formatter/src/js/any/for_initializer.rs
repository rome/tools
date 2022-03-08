//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyForInitializer;
impl ToFormatElement for JsAnyForInitializer {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsVariableDeclaration(node) => node.to_format_element(formatter),
            Self::JsAnyExpression(node) => node.to_format_element(formatter),
        }
    }
}
