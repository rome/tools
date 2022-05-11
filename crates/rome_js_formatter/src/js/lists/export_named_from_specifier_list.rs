use crate::formatter::TrailingSeparator;
use crate::generated::FormatJsExportNamedFromSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsExportNamedFromSpecifierList;

impl FormatRule<JsExportNamedFromSpecifierList> for FormatJsExportNamedFromSpecifierList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsExportNamedFromSpecifierList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::default())?,
        ))
    }
}
