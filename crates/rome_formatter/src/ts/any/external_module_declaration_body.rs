//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsAnyExternalModuleDeclarationBody;
impl ToFormatElement for TsAnyExternalModuleDeclarationBody {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::TsEmptyExternalModuleDeclarationBody(node) => node.to_format_element(formatter),
            Self::TsModuleBlock(node) => node.to_format_element(formatter),
        }
    }
}
