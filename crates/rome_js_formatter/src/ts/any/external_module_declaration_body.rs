//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyExternalModuleDeclarationBody;
impl Format for TsAnyExternalModuleDeclarationBody {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsEmptyExternalModuleDeclarationBody(node) => node.format(formatter),
            Self::TsModuleBlock(node) => node.format(formatter),
        }
    }
}
