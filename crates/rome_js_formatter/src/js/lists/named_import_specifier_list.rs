use crate::generated::FormatJsNamedImportSpecifierList;
use crate::prelude::*;
use rome_js_syntax::JsNamedImportSpecifierList;

impl FormatRule<JsNamedImportSpecifierList> for FormatJsNamedImportSpecifierList {
    type Context = JsFormatContext;

    fn format(
        node: &JsNamedImportSpecifierList,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","))?,
        ))
    }
}
