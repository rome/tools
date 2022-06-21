use crate::prelude::*;
use rome_js_syntax::{JsExportNamedSpecifierList, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportNamedSpecifierList;

impl FormatNodeRule<JsExportNamedSpecifierList> for FormatJsExportNamedSpecifierList {
    fn fmt_fields(
        &self,
        node: &JsExportNamedSpecifierList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA))
            .finish()
    }
}
