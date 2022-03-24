//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyTupleTypeElement;
impl ToFormatElement for TsAnyTupleTypeElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsNamedTupleTypeElement(node) => node.to_format_element(formatter),
            Self::TsType(node) => node.to_format_element(formatter),
            Self::TsRestTupleTypeElement(node) => node.to_format_element(formatter),
            Self::TsOptionalTupleTypeElement(node) => node.to_format_element(formatter),
        }
    }
}
