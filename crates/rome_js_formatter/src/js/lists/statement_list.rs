use crate::generated::FormatJsStatementList;
use crate::prelude::*;
use rome_js_syntax::JsStatementList;

impl FormatRule<JsStatementList> for FormatJsStatementList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsStatementList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(formatter.format_list_with_hard_line(node))
    }
}
