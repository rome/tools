//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyPropertyParameter;
impl ToFormatElement for TsAnyPropertyParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsPropertyParameter(node) => node.to_format_element(formatter),
            Self::TsReadonlyPropertyParameter(node) => node.to_format_element(formatter),
        }
    }
}

