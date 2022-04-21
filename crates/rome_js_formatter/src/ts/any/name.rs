//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::TsAnyName;
impl Format for TsAnyName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsReferenceIdentifier(node) => node.format(formatter),
            Self::TsQualifiedName(node) => node.format(formatter),
        }
    }
}
