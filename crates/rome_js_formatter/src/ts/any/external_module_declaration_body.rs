//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyExternalModuleDeclarationBody;
use crate::prelude::*;
use rome_js_syntax::TsAnyExternalModuleDeclarationBody;
impl FormatRule<TsAnyExternalModuleDeclarationBody> for FormatTsAnyExternalModuleDeclarationBody {
    type Options = JsFormatOptions;
    fn format(
        node: &TsAnyExternalModuleDeclarationBody,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyExternalModuleDeclarationBody::TsModuleBlock(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
