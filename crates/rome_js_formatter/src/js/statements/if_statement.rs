use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    block_indent, concat_elements, group_elements, hard_group_elements, hard_line_break, token,
};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::SyntaxToken;
use rome_js_syntax::{JsAnyStatement, JsElseClauseFields, JsIfStatement};
use rome_js_syntax::{JsElseClause, JsIfStatementFields};

impl ToFormatElement for JsIfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let (head, mut else_clause) = format_if_element(formatter, None, self)?;

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
                    if_chain.push(format_elements![
                        space_token(),
                        else_token.format(formatter)?,
                        space_token(),
                        into_block(formatter, alternate)?,
                    ]);
                }
            }
        }

        Ok(hard_group_elements(concat_elements(if_chain)))
    }
}

/// Format a single `else? if(test) consequent` element, returning the next else clause
fn format_if_element(
    formatter: &Formatter,
    else_token: Option<SyntaxToken>,
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

    let head = format_elements![
        else_token.format_with_or_empty(formatter, |token| format_elements![
            space_token(),
            token,
            space_token(),
        ])?,
        if_token.format(formatter)?,
        space_token(),
        formatter.format_delimited_soft_block_indent(
            &l_paren_token?,
            test.format(formatter)?,
            &r_paren_token?,
        )?,
        space_token(),
        into_block(formatter, consequent?)?,
    ];

    Ok((head, else_clause))
}

/// Wraps the statement into a block if its not already a JsBlockStatement
fn into_block(formatter: &Formatter, stmt: JsAnyStatement) -> FormatResult<FormatElement> {
    if matches!(stmt, JsAnyStatement::JsBlockStatement(_)) {
        return stmt.format(formatter);
    }

    // If the body is an empty statement, force a line break to ensure behavior
    // is coherent with `is_non_collapsable_empty_block`
    if matches!(stmt, JsAnyStatement::JsEmptyStatement(_)) {
        return Ok(format_elements![
            token("{"),
            stmt.format(formatter)?,
            hard_line_break(),
            token("}")
        ]);
    }

    Ok(group_elements(format_elements![
        token("{"),
        block_indent(stmt.format(formatter)?),
        token("}"),
    ]))
}
