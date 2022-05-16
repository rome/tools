//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyNamedImport;
use crate::prelude::*;
use rome_js_syntax::JsAnyNamedImport;
impl FormatRule<JsAnyNamedImport> for FormatJsAnyNamedImport {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyNamedImport,
        formatter: &Formatter<Self::Options>,
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
