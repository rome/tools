use crate::prelude::*;
use crate::{format_or_verbatim, FormatNodeFields};
use rome_formatter::{format_args, write};
use rome_js_syntax::JsDefaultClause;
use rome_js_syntax::{JsAnyStatement, JsDefaultClauseFields};
use rome_rowan::AstNodeList;

impl FormatNodeFields<JsDefaultClause> for FormatNodeRule<JsDefaultClause> {
    fn fmt_fields(node: &JsDefaultClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDefaultClauseFields {
            default_token,
            colon_token,
            consequent,
        } = node.as_fields();

        let first_child_is_block_stmt = matches!(
            consequent.iter().next(),
            Some(JsAnyStatement::JsBlockStatement(_))
        );

        write!(
            f,
            [default_token.format(), colon_token.format(), space_token()]
        )?;

        let format_statements = format_with(|f| {
            let mut join = f.join_nodes_with_hardline();

            for stmt in &consequent {
                join.entry(stmt.syntax(), &format_or_verbatim(&stmt));
            }
            join.finish()
        });

        if consequent.is_empty() {
            write!(f, [hard_line_break()])
        } else if first_child_is_block_stmt {
            write!(f, [space_token(), format_statements])
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            write!(
                f,
                [indent(&format_args!(hard_line_break(), format_statements))]
            )
        }
    }
}
