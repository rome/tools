use crate::formatter::TrailingSeparator;
use crate::generated::FormatJsExportNamedFromSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsExportNamedFromSpecifierList;

impl FormatRule<JsExportNamedFromSpecifierList> for FormatJsExportNamedFromSpecifierList {
    fn format(
        node: &JsExportNamedFromSpecifierList,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::default())?,
        ))
    }
}
