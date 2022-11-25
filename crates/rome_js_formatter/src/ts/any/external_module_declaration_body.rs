//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::TsAnyExternalModuleDeclarationBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyExternalModuleDeclarationBody;
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
