//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyExternalModuleDeclarationBody;
use crate::prelude::*;
use rome_js_syntax::TsAnyExternalModuleDeclarationBody;
impl FormatRule<TsAnyExternalModuleDeclarationBody> for FormatTsAnyExternalModuleDeclarationBody {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyExternalModuleDeclarationBody,
        formatter: &Formatter<Self::Context>,
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
