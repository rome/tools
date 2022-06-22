//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImport;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyNamedImport;
impl FormatRule<JsAnyNamedImport> for FormatJsAnyNamedImport {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyNamedImport, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyNamedImport::JsNamedImportSpecifiers(node) => node.format().fmt(f),
            JsAnyNamedImport::JsNamespaceImportSpecifier(node) => node.format().fmt(f),
        }
    }
}
