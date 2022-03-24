//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyReturnType;
impl ToFormatElement for TsAnyReturnType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsType(node) => node.format(formatter),
            Self::TsPredicateReturnType(node) => node.format(formatter),
            Self::TsAssertsReturnType(node) => node.format(formatter),
        }
    }
}
