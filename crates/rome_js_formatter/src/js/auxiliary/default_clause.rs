use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsDefaultClause;
use rome_js_syntax::{JsAnyStatement, JsDefaultClauseFields};
use rome_rowan::AstNodeList;

impl FormatNodeFields<JsDefaultClause> for FormatNodeRule<JsDefaultClause> {
    fn format_fields(
        node: &JsDefaultClause,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsDefaultClauseFields {
            default_token,
            colon_token,
            consequent,
        } = node.as_fields();

        let first_child_is_block_stmt = matches!(
            consequent.iter().next(),
            Some(JsAnyStatement::JsBlockStatement(_))
        );

        let default = default_token.format();
        let colon = colon_token.format();
        let statements = formatter.format_list_with_hard_line(&consequent);

        let formatted_cons = if statements.is_empty() {
            hard_line_break()
        } else if first_child_is_block_stmt {
            formatted![formatter, [space_token(), statements]]?
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            indent(formatted![formatter, [hard_line_break(), statements]]?)
        };
        formatted![formatter, [default, colon, space_token(), formatted_cons]]
    }
}
