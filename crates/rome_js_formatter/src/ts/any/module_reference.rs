//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyModuleReference;
use crate::prelude::*;
use rome_js_syntax::TsAnyModuleReference;
impl FormatRule<TsAnyModuleReference> for FormatTsAnyModuleReference {
    type Context = JsFormatContext;
    fn format(node: &TsAnyModuleReference, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyModuleReference::TsAnyName(node) => node.format().format(f),
            TsAnyModuleReference::TsExternalModuleReference(node) => node.format().format(f),
        }
    }
}
