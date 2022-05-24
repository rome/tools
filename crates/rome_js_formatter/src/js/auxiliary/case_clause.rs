use crate::prelude::*;
use rome_js_syntax::JsAnyStatement;
use rome_rowan::AstNodeList;

use crate::FormatNodeFields;
use rome_js_syntax::JsCaseClause;
use rome_js_syntax::JsCaseClauseFields;

impl FormatNodeFields<JsCaseClause> for FormatNodeRule<JsCaseClause> {
    fn format_fields(
        node: &JsCaseClause,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsCaseClauseFields {
            case_token,
            test,
            colon_token,
            consequent,
        } = node.as_fields();

        let is_first_child_block_stmt = matches!(
            consequent.iter().next(),
            Some(JsAnyStatement::JsBlockStatement(_))
        );
        let case_word = case_token.format();
        let colon = colon_token.format();
        let test = test.format();
        let cons = formatter.format_list(&consequent);

        let cons = if cons.is_empty() {
            // Skip inserting an indent block is the consequent is empty to print
            // the trailing comments for the case clause inline if there is no
            // block to push them into
            hard_line_break()
        } else if is_first_child_block_stmt {
            formatted![formatter, [space_token(), cons]]?
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            indent(formatted![formatter, [hard_line_break(), cons]]?)
        };

        formatted![formatter, [case_word, space_token(), test, colon, cons]]
    }
}
