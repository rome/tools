use crate::prelude::*;
use rome_js_syntax::{JsAnyStatement, JsStatementList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsStatementList;

impl FormatRule<JsStatementList> for FormatJsStatementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsStatementList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for statement in node.iter() {
            match statement {
                JsAnyStatement::JsEmptyStatement(empty) => {
                    join.entry_no_separator(&empty.format());
                }
                _ => {
                    join.entry(statement.syntax(), &format_or_verbatim(&statement));
                }
            }
        }

        join.finish()
    }
}
