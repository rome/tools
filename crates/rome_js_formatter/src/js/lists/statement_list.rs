use crate::formatter::TryFormatNodeListExtension;
use crate::generated::FormatJsStatementList;
use crate::prelude::*;
use rome_js_syntax::JsStatementList;

impl FormatRule<JsStatementList> for FormatJsStatementList {
    type Context = JsFormatContext;

    fn format(node: &JsStatementList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&hard_line_break())
            .entries(node.try_format_nodes())
            .finish()
    }
}
