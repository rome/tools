//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyForInOrOfInitializer;
impl ToFormatElement for JsAnyForInOrOfInitializer {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyAssignmentPattern(node) => node.to_format_element(formatter),
            Self::JsForVariableDeclaration(node) => node.to_format_element(formatter),
        }
    }
}

