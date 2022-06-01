use crate::generated::FormatJsImportAssertionEntryList;
use crate::prelude::*;
use rome_js_syntax::JsImportAssertionEntryList;

impl FormatRule<JsImportAssertionEntryList> for FormatJsImportAssertionEntryList {
    type Context = JsFormatContext;

    fn format(
        node: &JsImportAssertionEntryList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","))?,
        ))
    }
}
