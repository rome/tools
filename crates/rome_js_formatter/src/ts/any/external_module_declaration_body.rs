//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyExternalModuleDeclarationBody;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyExternalModuleDeclarationBody;
impl FormatRule<TsAnyExternalModuleDeclarationBody> for FormatTsAnyExternalModuleDeclarationBody {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &TsAnyExternalModuleDeclarationBody,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            TsAnyExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(node) => {
                node.format().fmt(f)
            }
            TsAnyExternalModuleDeclarationBody::TsModuleBlock(node) => node.format().fmt(f),
        }
    }
}
