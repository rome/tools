//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyName;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyName;
impl FormatRule<TsAnyName> for FormatTsAnyName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyName::JsReferenceIdentifier(node) => node.format().fmt(f),
            TsAnyName::TsQualifiedName(node) => node.format().fmt(f),
        }
    }
}
