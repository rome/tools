use crate::prelude::*;
use rome_js_syntax::{JsImportAssertionEntryList, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsImportAssertionEntryList;

impl FormatNodeRule<JsImportAssertionEntryList> for FormatJsImportAssertionEntryList {
    fn fmt_fields(
        &self,
        node: &JsImportAssertionEntryList,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA))
            .finish()
    }
}
