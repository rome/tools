//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyExternalModuleDeclarationBody;
use crate::prelude::*;
use rome_js_syntax::TsAnyExternalModuleDeclarationBody;
impl FormatRule<TsAnyExternalModuleDeclarationBody> for FormatTsAnyExternalModuleDeclarationBody {
    type Context = JsFormatContext;
    fn format(node: &TsAnyExternalModuleDeclarationBody, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(node) => {
                node.format().format(f)
            }
            TsAnyExternalModuleDeclarationBody::TsModuleBlock(node) => node.format().format(f),
        }
    }
}
