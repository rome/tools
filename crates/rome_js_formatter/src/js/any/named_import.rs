//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImport;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyNamedImport;
impl FormatRule<JsAnyNamedImport> for FormatJsAnyNamedImport {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyNamedImport, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyNamedImport::JsNamedImportSpecifiers(node) => node.format().fmt(f),
            JsAnyNamedImport::JsNamespaceImportSpecifier(node) => node.format().fmt(f),
        }
    }
}
