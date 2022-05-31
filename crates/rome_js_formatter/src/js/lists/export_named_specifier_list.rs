use crate::generated::FormatJsExportNamedSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsExportNamedSpecifierList;

impl FormatRule<JsExportNamedSpecifierList> for FormatJsExportNamedSpecifierList {
    type Context = JsFormatContext;

    fn format(
        node: &JsExportNamedSpecifierList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","))?,
        ))
    }
}
