use crate::prelude::*;
use rome_js_syntax::{JsExportNamedFromSpecifierList, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportNamedFromSpecifierList;

impl FormatNodeRule<JsExportNamedFromSpecifierList> for FormatJsExportNamedFromSpecifierList {
    fn fmt_fields(
        &self,
        node: &JsExportNamedFromSpecifierList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA))
            .finish()
    }
}
