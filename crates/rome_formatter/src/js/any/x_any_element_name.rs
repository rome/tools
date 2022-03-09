//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsxAnyElementName;
impl ToFormatElement for JsxAnyElementName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxReferenceIdentifier(node) => node.to_format_element(formatter),
            Self::JsxMemberExpression(node) => node.to_format_element(formatter),
        }
    }
}
