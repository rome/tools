use crate::prelude::*;
use rome_js_syntax::{JsNamedImportSpecifierList, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsNamedImportSpecifierList;

impl FormatNodeRule<JsNamedImportSpecifierList> for FormatJsNamedImportSpecifierList {
    fn fmt_fields(
        &self,
        node: &JsNamedImportSpecifierList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA))
            .finish()
    }
}
