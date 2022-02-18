use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, group_elements, hard_group_elements, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};

use rslint_parser::ast::JsIfStatementFields;
use rslint_parser::ast::{JsAnyStatement, JsIfStatement};

impl ToFormatElement for JsIfStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsIfStatementFields {
            if_token,
            l_paren_token,
            test,
            r_paren_token,
            consequent,
            else_clause,
        } = self.as_fields();

        let head = group_elements(format_elements![
            if_token.format(formatter)?,
            space_token(),
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                test.format(formatter)?,
                &r_paren_token?,
            )?,
            space_token(),
        ]);

        let consequent = consequent?;
        let has_consequent_block = matches!(consequent, JsAnyStatement::JsBlockStatement(_));

        let has_else_block = else_clause
            .as_ref()
            .and_then(|else_clause| {
                let alternate = else_clause.alternate().ok()?;
                Some(matches!(
                    alternate,
                    JsAnyStatement::JsBlockStatement(_) | JsAnyStatement::JsIfStatement(_)
                ))
            })
            .unwrap_or(false);

        let else_clause = else_clause.format_with_or_empty(formatter, |else_clause| {
            format_elements![space_token(), else_clause]
        })?;

        match (has_consequent_block, has_else_block) {
            (false, false) => Ok(format_elements![
                hard_group_elements(head),
                consequent.format(formatter)?,
                else_clause,
            ]),
            (false, true) => Ok(format_elements![
                head,
                hard_group_elements(format_elements![consequent.format(formatter)?, else_clause]),
            ]),
            (true, false) => Ok(format_elements![
                hard_group_elements(format_elements![head, consequent.format(formatter)?]),
                else_clause
            ]),
            (true, true) => Ok(hard_group_elements(format_elements![
                head,
                consequent.format(formatter)?,
                else_clause
            ])),
        }
    }
}
