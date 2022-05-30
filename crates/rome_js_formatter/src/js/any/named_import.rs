//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyNamedImport;
use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImport;
impl FormatRule<JsAnyNamedImport> for FormatJsAnyNamedImport {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyNamedImport,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
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
