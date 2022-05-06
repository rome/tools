use crate::{
    formatted, hard_line_break, indent, space_token, FormatElement, FormatNode,
    Formatter,
};
use crate::{Format, JsFormatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsDefaultClause;
use rome_js_syntax::{JsAnyStatement, JsDefaultClauseFields};
use rome_rowan::AstNodeList;

impl FormatNode for JsDefaultClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDefaultClauseFields {
            default_token,
            colon_token,
            consequent,
        } = self.as_fields();

        let first_child_is_block_stmt = matches!(
            consequent.iter().next(),
            Some(JsAnyStatement::JsBlockStatement(_))
        );

        let default = default_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;
        let statements = formatter.format_list(consequent);

        let formatted_cons = if statements.is_empty() {
            hard_line_break()
        } else if first_child_is_block_stmt {
            formatted![formatter, space_token(), statements]?
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            indent(formatted![formatter, hard_line_break(), statements]?)
        };
        formatted![formatter, default, colon, space_token(), formatted_cons]
    }
}
