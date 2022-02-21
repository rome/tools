use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{concat_elements, group_elements, hard_group_elements, soft_block_indent};

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::{JsAnyStatement, JsElseClauseFields, JsIfStatement};
use rslint_parser::ast::{JsElseClause, JsIfStatementFields};
use rslint_parser::SyntaxToken;

impl ToFormatElement for JsIfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let (head, mut else_clause) = format_if_item(formatter, None, self)?;

        let mut if_chain = vec![head];
        while let Some(clause) = else_clause.take() {
            let JsElseClauseFields {
                else_token,
                alternate,
            } = clause.as_fields();

            match alternate? {
                JsAnyStatement::JsIfStatement(stmt) => {
                    let (head, alternate) = format_if_item(formatter, Some(else_token?), &stmt)?;
                    if_chain.push(head);
                    else_clause = alternate;
                }
                block @ JsAnyStatement::JsBlockStatement(_) => {
                    if_chain.push(format_elements![
                        space_token(),
                        else_token.format(formatter)?,
                        space_token(),
                        block.format(formatter)?,
                    ]);
                }
                alternate => {
                    if_chain.push(format_elements![
                        space_token(),
                        else_token.format(formatter)?,
                        space_token(),
                        soft_block_indent(alternate.format(formatter)?),
                    ]);
                }
            }
        }

        Ok(group_elements(concat_elements(if_chain)))
    }
}

fn format_if_item(
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
    ];

    let body = consequent?;
    let head = if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
        hard_group_elements(format_elements![head, body.format(formatter)?])
    } else {
        format_elements![
            hard_group_elements(head),
            soft_block_indent(body.format(formatter)?),
        ]
    };

    Ok((head, else_clause))
}
