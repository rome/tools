//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyModuleReference;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyModuleReference;
impl FormatRule<TsAnyModuleReference> for FormatTsAnyModuleReference {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyModuleReference, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyModuleReference::TsAnyName(node) => node.format().fmt(f),
            TsAnyModuleReference::TsExternalModuleReference(node) => node.format().fmt(f),
        }
    }
}
