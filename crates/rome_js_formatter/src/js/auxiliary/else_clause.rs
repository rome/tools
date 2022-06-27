use crate::js::statements::if_statement::FormatIfElseConsequentBlock;
use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsAnyStatement::JsIfStatement;
use rome_js_syntax::JsElseClause;
use rome_js_syntax::JsElseClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsElseClause;

impl FormatNodeRule<JsElseClause> for FormatJsElseClause {
    fn fmt_fields(&self, node: &JsElseClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsElseClauseFields {
            else_token,
            alternate,
        } = node.as_fields();

        write!(f, [space_token(), else_token.format()])?;

        match alternate? {
            JsIfStatement(if_statement) => {
                write!(f, [space_token(), if_statement.format()])
            }
            other => {
                write!(f, [FormatIfElseConsequentBlock::from(other)])
            }
        }
    }
}
