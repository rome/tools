//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::TsAnyName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyName;
impl FormatRule<TsAnyName> for FormatTsAnyName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyName::JsReferenceIdentifier(node) => node.format().fmt(f),
            TsAnyName::TsQualifiedName(node) => node.format().fmt(f),
        }
    }
}
