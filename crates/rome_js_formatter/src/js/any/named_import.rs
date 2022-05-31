//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyNamedImport;
use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImport;
impl FormatRule<JsAnyNamedImport> for FormatJsAnyNamedImport {
    type Context = JsFormatContext;
    fn format(node: &JsAnyNamedImport, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyNamedImport::JsNamedImportSpecifiers(node) => node.format().format(f),
            JsAnyNamedImport::JsNamespaceImportSpecifier(node) => node.format().format(f),
        }
    }
}
