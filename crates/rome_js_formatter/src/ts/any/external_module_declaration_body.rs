//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyExternalModuleDeclarationBody;
use crate::prelude::*;
use rome_js_syntax::TsAnyExternalModuleDeclarationBody;
impl FormatRule<TsAnyExternalModuleDeclarationBody> for FormatTsAnyExternalModuleDeclarationBody {
    type Context = JsFormatContext;
    fn fmt(node: &TsAnyExternalModuleDeclarationBody, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(node) => {
                node.format().fmt(f)
            }
            TsAnyExternalModuleDeclarationBody::TsModuleBlock(node) => node.format().fmt(f),
        }
    }
}
