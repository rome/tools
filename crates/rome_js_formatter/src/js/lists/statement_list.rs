use crate::generated::FormatJsStatementList;
use crate::prelude::*;
use rome_js_syntax::JsStatementList;

impl FormatRule<JsStatementList> for FormatJsStatementList {
    type Context = JsFormatContext;

    fn fmt(node: &JsStatementList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for statement in node.iter() {
            join.entry(statement.syntax(), &format_or_verbatim(&statement));
        }

        join.finish()
    }
}
