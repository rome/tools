//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyModuleName;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyModuleName;
impl FormatRule<TsAnyModuleName> for FormatTsAnyModuleName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyModuleName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyModuleName::TsIdentifierBinding(node) => node.format().fmt(f),
            TsAnyModuleName::TsQualifiedModuleName(node) => node.format().fmt(f),
        }
    }
}
