use crate::prelude::*;
use crate::{format_or_verbatim, FormatNodeFields};
use rome_formatter::{format_args, write};
use rome_js_syntax::JsAnyStatement;
use rome_js_syntax::JsCaseClause;
use rome_js_syntax::JsCaseClauseFields;
use rome_rowan::AstNodeList;

impl FormatNodeFields<JsCaseClause> for FormatNodeRule<JsCaseClause> {
    fn fmt_fields(node: &JsCaseClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCaseClauseFields {
            case_token,
            test,
            colon_token,
            consequent,
        } = node.as_fields();

        write!(
            f,
            [
                case_token.format(),
                space_token(),
                test.format(),
                colon_token.format()
            ]
        )?;

        let is_first_child_block_stmt = matches!(
            consequent.iter().next(),
            Some(JsAnyStatement::JsBlockStatement(_))
        );

        let format_consequent = format_with(|f| {
            let mut join = f.join_nodes_with_hardline();
            for stmt in &consequent {
                join.entry(stmt.syntax(), &format_or_verbatim(&stmt));
            }
            join.finish()
        });

        if consequent.is_empty() {
            // Skip inserting an indent block is the consequent is empty to print
            // the trailing comments for the case clause inline if there is no
            // block to push them into
            return write!(f, [hard_line_break()]);
        } else if is_first_child_block_stmt {
            write![f, [space_token(), format_consequent]]
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            write!(
                f,
                [indent(&format_args![hard_line_break(), format_consequent])]
            )
        }
    }
}
