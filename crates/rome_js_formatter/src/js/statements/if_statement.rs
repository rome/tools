use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsIfStatementFields;
use rome_js_syntax::JsSyntaxKind;
use rome_js_syntax::{JsAnyStatement, JsIfStatement};

#[derive(Debug, Clone, Default)]
pub struct FormatJsIfStatement;

impl FormatNodeRule<JsIfStatement> for FormatJsIfStatement {
    fn fmt_fields(&self, node: &JsIfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsIfStatementFields {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = node.as_fields();

        write![
            f,
            [
                if_token.format(),
                space_token(),
                format_delimited(&l_paren_token?, &test.format(), &r_paren_token?)
                    .soft_block_indent(),
                FormatIfElseConsequentBlock::from(consequent?),
                else_clause.format()
            ]
        ]
    }
}

pub struct FormatIfElseConsequentBlock(JsAnyStatement);

impl From<JsAnyStatement> for FormatIfElseConsequentBlock {
    fn from(stmt: JsAnyStatement) -> Self {
        FormatIfElseConsequentBlock(stmt)
    }
}

impl Format<JsFormatContext> for FormatIfElseConsequentBlock {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let stmt = &self.0;

        if matches!(stmt, JsAnyStatement::JsBlockStatement(_)) {
            write!(f, [space_token(), stmt.format()])
        }
        // If the body is an empty statement, force a line break to ensure behavior
        // is coherent with `is_non_collapsable_empty_block`
        else if matches!(stmt, JsAnyStatement::JsEmptyStatement(_)) {
            write!(f, [stmt.format(), hard_line_break()])
        } else {
            write!(
                f,
                [
                    space_token(),
                    format_inserted(JsSyntaxKind::L_CURLY),
                    block_indent(&stmt.format()),
                    format_inserted(JsSyntaxKind::R_CURLY)
                ]
            )
        }
    }
}
