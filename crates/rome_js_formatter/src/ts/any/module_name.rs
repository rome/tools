//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::TsAnyModuleName;
impl Format for TsAnyModuleName {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsIdentifierBinding(node) => node.format(formatter),
            Self::TsQualifiedModuleName(node) => node.format(formatter),
        }
    }
}
