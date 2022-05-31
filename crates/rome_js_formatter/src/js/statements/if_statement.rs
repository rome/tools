use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsSyntaxToken;
use rome_js_syntax::{JsAnyStatement, JsElseClauseFields, JsIfStatement};
use rome_js_syntax::{JsElseClause, JsIfStatementFields};

impl FormatNodeFields<JsIfStatement> for FormatNodeRule<JsIfStatement> {
    fn format_fields(node: &JsIfStatement, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        let (head, mut else_clause) = format_if_element(formatter, None, node)?;

        let mut if_chain = vec![head];
        while let Some(clause) = else_clause.take() {
            let JsElseClauseFields {
                else_token,
                alternate,
            } = clause.as_fields();

            match alternate? {
                JsAnyStatement::JsIfStatement(stmt) => {
                    let (head, alternate) = format_if_element(formatter, Some(else_token?), &stmt)?;

                    if_chain.push(head);
                    else_clause = alternate;
                }
                alternate => {
                    if_chain.push(formatted![
                        formatter,
                        [
                            space_token(),
                            else_token.format(),
                            into_block(formatter, alternate)?,
                        ]
                    ]?);
                }
            }
        }

        Ok(concat_elements(if_chain))
    }
}

/// Format a single `else? if(test) consequent` element, returning the next else clause
fn format_if_element(
    formatter: &JsFormatter,
    else_token: Option<JsSyntaxToken>,
    stmt: &JsIfStatement,
) -> FormatResult<(FormatElement, Option<JsElseClause>)> {
    let JsIfStatementFields {
        if_token,
        l_paren_token,
        test,
        r_paren_token,
        consequent,
        else_clause,
    } = stmt.as_fields();

    let head = formatted![
        formatter,
        [
            else_token.format().with_or_empty(|token| formatted![
                formatter,
                [space_token(), token, space_token(),]
            ]),
            if_token.format(),
            space_token(),
            formatter
                .delimited(
                    &l_paren_token?,
                    formatted![formatter, [test.format()]]?,
                    &r_paren_token?,
                )
                .soft_block_indent()
                .finish()?,
            into_block(formatter, consequent?)?,
        ]
    ]?;

    Ok((head, else_clause))
}

/// Wraps the statement into a block if its not already a JsBlockStatement
fn into_block(formatter: &JsFormatter, stmt: JsAnyStatement) -> FormatResult<FormatElement> {
    if matches!(stmt, JsAnyStatement::JsBlockStatement(_)) {
        return formatted![formatter, [space_token(), stmt.format()]];
    }

    // If the body is an empty statement, force a line break to ensure behavior
    // is coherent with `is_non_collapsable_empty_block`
    if matches!(stmt, JsAnyStatement::JsEmptyStatement(_)) {
        return formatted![formatter, [stmt.format(), hard_line_break()]];
    }

    formatted![
        formatter,
        [
            space_token(),
            token("{"),
            block_indent(formatted![formatter, [stmt.format()]]?),
            token("}"),
        ]
    ]
}
