//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyBinding;
impl ToFormatElement for JsAnyBinding {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsIdentifierBinding(node) => node.to_format_element(formatter),
            Self::JsUnknownBinding(node) => node.to_format_element(formatter),
        }
    }
}
