use crate::generated::FormatJsExportNamedSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsExportNamedSpecifierList;

impl FormatRule<JsExportNamedSpecifierList> for FormatJsExportNamedSpecifierList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsExportNamedSpecifierList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","))?,
        ))
    }
}
