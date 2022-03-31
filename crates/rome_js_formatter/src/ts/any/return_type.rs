//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyReturnType;
impl ToFormatElement for TsAnyReturnType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsType(node) => node.to_format_element(formatter),
            Self::TsPredicateReturnType(node) => node.to_format_element(formatter),
            Self::TsAssertsReturnType(node) => node.to_format_element(formatter),
        }
    }
}
