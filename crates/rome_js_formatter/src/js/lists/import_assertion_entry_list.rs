use crate::formatter::TrailingSeparator;
use crate::generated::FormatJsImportAssertionEntryList;
use crate::prelude::*;
use rome_js_syntax::JsImportAssertionEntryList;

impl FormatRule<JsImportAssertionEntryList> for FormatJsImportAssertionEntryList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsImportAssertionEntryList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::default())?,
        ))
    }
}
