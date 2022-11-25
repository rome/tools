//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::TsAnyModuleName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyModuleName;
impl FormatRule<TsAnyModuleName> for FormatTsAnyModuleName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyModuleName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyModuleName::TsIdentifierBinding(node) => node.format().fmt(f),
            TsAnyModuleName::TsQualifiedModuleName(node) => node.format().fmt(f),
        }
    }
}
