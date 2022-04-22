//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::TsAnyPropertyAnnotation;
impl Format for TsAnyPropertyAnnotation {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsTypeAnnotation(node) => node.format(formatter),
            Self::TsOptionalPropertyAnnotation(node) => node.format(formatter),
            Self::TsDefinitePropertyAnnotation(node) => node.format(formatter),
        }
    }
}
