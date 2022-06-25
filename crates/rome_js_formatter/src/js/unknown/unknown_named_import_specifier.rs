use crate::prelude::*;

use rome_js_syntax::JsUnknownNamedImportSpecifier;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownNamedImportSpecifier;

impl FormatNodeRule<JsUnknownNamedImportSpecifier> for FormatJsUnknownNamedImportSpecifier {
    fn fmt_fields(
        &self,
        node: &JsUnknownNamedImportSpecifier,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
