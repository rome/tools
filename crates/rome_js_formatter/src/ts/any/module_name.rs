//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyModuleName;
use crate::prelude::*;
use rome_js_syntax::TsAnyModuleName;
impl FormatRule<TsAnyModuleName> for FormatTsAnyModuleName {
    type Context = JsFormatContext;
    fn fmt(node: &TsAnyModuleName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyModuleName::TsIdentifierBinding(node) => node.format().fmt(f),
            TsAnyModuleName::TsQualifiedModuleName(node) => node.format().fmt(f),
        }
    }
}
