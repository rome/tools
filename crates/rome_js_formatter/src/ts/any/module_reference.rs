//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyModuleReference;
impl Format for TsAnyModuleReference {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsAnyName(node) => node.format(formatter),
            Self::TsExternalModuleReference(node) => node.format(formatter),
        }
    }
}
