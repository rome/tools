//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::TsAnyVariableAnnotation;
impl ToFormatElement for TsAnyVariableAnnotation {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsTypeAnnotation(node) => node.to_format_element(formatter),
            Self::TsDefiniteVariableAnnotation(node) => node.to_format_element(formatter),
        }
    }
}
