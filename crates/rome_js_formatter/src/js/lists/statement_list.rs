use crate::generated::FormatJsStatementList;
use crate::prelude::*;
use rome_js_syntax::JsStatementList;

impl FormatRule<JsStatementList> for FormatJsStatementList {
    fn format(node: &JsStatementList, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_list(node))
    }
}
