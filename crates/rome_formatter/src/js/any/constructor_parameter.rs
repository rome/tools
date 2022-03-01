//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyConstructorParameter;
impl ToFormatElement for JsAnyConstructorParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyFormalParameter(node) => node.to_format_element(formatter),
            Self::JsRestParameter(node) => node.to_format_element(formatter),
            Self::TsPropertyParameter(node) => node.to_format_element(formatter),
        }
    }
}
