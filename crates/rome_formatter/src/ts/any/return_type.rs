//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyReturnType;
impl ToFormatElement for TsAnyReturnType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsType(node) => node.to_format_element(formatter),
            Self::TsTypePredicate(node) => node.to_format_element(formatter),
        }
    }
}
