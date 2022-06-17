use crate::generated::FormatJsExportNamedFromSpecifierList;
use crate::prelude::*;
use rome_js_syntax::{JsExportNamedFromSpecifierList, JsSyntaxKind};

impl FormatRule<JsExportNamedFromSpecifierList> for FormatJsExportNamedFromSpecifierList {
    type Context = JsFormatContext;

    fn fmt(node: &JsExportNamedFromSpecifierList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA))
            .finish()
    }
}
