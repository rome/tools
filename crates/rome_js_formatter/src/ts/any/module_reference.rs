//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::TsAnyModuleReference;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyModuleReference;
impl FormatRule<TsAnyModuleReference> for FormatTsAnyModuleReference {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyModuleReference, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyModuleReference::TsAnyName(node) => node.format().fmt(f),
            TsAnyModuleReference::TsExternalModuleReference(node) => node.format().fmt(f),
        }
    }
}
