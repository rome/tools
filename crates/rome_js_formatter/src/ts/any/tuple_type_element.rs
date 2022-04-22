//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::TsAnyTupleTypeElement;
impl Format for TsAnyTupleTypeElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsNamedTupleTypeElement(node) => node.format(formatter),
            Self::TsType(node) => node.format(formatter),
            Self::TsRestTupleTypeElement(node) => node.format(formatter),
            Self::TsOptionalTupleTypeElement(node) => node.format(formatter),
        }
    }
}
