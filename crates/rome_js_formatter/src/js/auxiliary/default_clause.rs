use crate::prelude::*;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsDefaultClause;
use rome_js_syntax::{JsAnyStatement, JsDefaultClauseFields};
use rome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsDefaultClause;

impl FormatNodeRule<JsDefaultClause> for FormatJsDefaultClause {
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

        if consequent.is_empty() {
            write!(f, [hard_line_break()])
        } else if first_child_is_block_stmt {
            write!(f, [space_token(), consequent.format()])
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            write!(
                f,
                [indent(&format_args!(
                    hard_line_break(),
                    consequent.format()
                ))]
            )
        }
    }
}
