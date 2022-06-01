use crate::generated::FormatJsStatementList;
use crate::prelude::*;
use rome_js_syntax::JsStatementList;

impl FormatRule<JsStatementList> for FormatJsStatementList {
    type Context = JsFormatContext;

    fn format(node: &JsStatementList, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
