use crate::prelude::*;
use rome_js_syntax::{JsImportAssertionEntryList, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsImportAssertionEntryList;

impl FormatRule<JsImportAssertionEntryList> for FormatJsImportAssertionEntryList {
    type Context = JsFormatContext;

    fn fmt(node: &JsImportAssertionEntryList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(JsSyntaxKind::COMMA))
            .finish()
    }
}
