//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyTupleTypeElement;
impl ToFormatElement for TsAnyTupleTypeElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsNamedTupleTypeElement(node) => node.format(formatter),
            Self::TsType(node) => node.format(formatter),
            Self::TsRestTupleTypeElement(node) => node.format(formatter),
            Self::TsOptionalTupleTypeElement(node) => node.format(formatter),
        }
    }
}
