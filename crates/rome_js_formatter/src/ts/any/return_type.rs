//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::TsAnyReturnType;
impl Format for TsAnyReturnType {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsType(node) => node.format(formatter),
            Self::TsPredicateReturnType(node) => node.format(formatter),
            Self::TsAssertsReturnType(node) => node.format(formatter),
        }
    }
}
