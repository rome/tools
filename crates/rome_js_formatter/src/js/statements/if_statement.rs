use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsSyntaxToken;
use rome_js_syntax::{JsAnyStatement, JsElseClauseFields, JsIfStatement};
use rome_js_syntax::{JsElseClause, JsIfStatementFields};

impl FormatNodeFields<JsIfStatement> for FormatNodeRule<JsIfStatement> {
    fn fmt_fields(node: &JsIfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let mut else_clause = write_if_element(f, None, node)?;

        while let Some(clause) = else_clause.take() {
            let JsElseClauseFields {
                else_token,
                alternate,
            } = clause.as_fields();

            match alternate? {
                JsAnyStatement::JsIfStatement(stmt) => {
                    let alternate = write_if_element(f, Some(else_token?), &stmt)?;

                    else_clause = alternate;
                }
                alternate => {
                    write![f, [space_token(), else_token.format()]]?;
                    write_consequent_block(f, alternate)?;
                }
            }
        }

        Ok(())
    }
}

/// Format a single `else? if(test) consequent` element, returning the next else clause
fn write_if_element(
    f: &mut JsFormatter,
    else_token: Option<JsSyntaxToken>,
    stmt: &JsIfStatement,
) -> FormatResult<Option<JsElseClause>> {
    let JsIfStatementFields {
        if_token,
        l_paren_token,
        test,
        r_paren_token,
        consequent,
        else_clause,
    } = stmt.as_fields();

    if let Some(else_token) = else_token {
        write!(f, [space_token(), else_token.format(), space_token()])?;
    }

    write![
        f,
        [
            if_token.format(),
            space_token(),
            format_delimited(&l_paren_token?, &test.format(), &r_paren_token?).soft_block_indent(),
        ]
    ]?;

    write_consequent_block(f, consequent?)?;

    Ok(else_clause)
}

/// Wraps the statement into a block if its not already a JsBlockStatement
fn write_consequent_block(f: &mut JsFormatter, stmt: JsAnyStatement) -> FormatResult<()> {
    if matches!(stmt, JsAnyStatement::JsBlockStatement(_)) {
        return write![f, [space_token(), stmt.format()]];
    }

    // If the body is an empty statement, force a line break to ensure behavior
    // is coherent with `is_non_collapsable_empty_block`
    if matches!(stmt, JsAnyStatement::JsEmptyStatement(_)) {
        return write![f, [stmt.format(), hard_line_break()]];
    }

    write![
        f,
        [
            space_token(),
            token("{"),
            block_indent(&stmt.format()),
            token("}"),
        ]
    ]
}
