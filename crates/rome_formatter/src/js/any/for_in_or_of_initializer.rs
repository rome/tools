//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyForInOrOfInitializer;
impl ToFormatElement for JsAnyForInOrOfInitializer {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyAssignmentPattern(node) => node.format(formatter),
            Self::JsForVariableDeclaration(node) => node.format(formatter),
        }
    }
}
