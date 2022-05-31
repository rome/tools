use crate::prelude::*;

use crate::formatter::unknown_node;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownNamedImportSpecifier;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownNamedImportSpecifier>
    for FormatNodeRule<JsUnknownNamedImportSpecifier>
{
    fn format_fields(
        node: &JsUnknownNamedImportSpecifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        unknown_node(node.syntax()).format(formatter)
    }
}
