//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::TsAnyTypePredicateParameterName;
impl Format for TsAnyTypePredicateParameterName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsReferenceIdentifier(node) => node.format(formatter),
            Self::TsThisType(node) => node.format(formatter),
        }
    }
}
