//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyNamedImport;
use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImport;
impl FormatRule<JsAnyNamedImport> for FormatJsAnyNamedImport {
    fn format(node: &JsAnyNamedImport, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyNamedImport::JsNamedImportSpecifiers(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyNamedImport::JsNamespaceImportSpecifier(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
