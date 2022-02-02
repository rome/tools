use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, group_elements, soft_block_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsCatchClause, JsCatchDeclaration, JsFinallyClause, JsTryFinallyStatement, JsTryStatement,
};

impl ToFormatElement for JsTryStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.try_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?,
            space_token(),
            self.catch_clause().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsTryFinallyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let formatted_catch_clause = self
            .catch_clause()
            .format_with_or_empty(formatter, |catch_clause| {
                format_elements![space_token(), catch_clause]
            })?;

        Ok(format_elements![
            self.try_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?,
            formatted_catch_clause,
            space_token(),
            self.finally_clause().format(formatter)?
        ])
    }
}

impl ToFormatElement for JsCatchClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.declaration().try_format_with_or(
            formatter,
            |declaration| {
                Ok(format_elements![
                    self.catch_token().format(formatter)?,
                    space_token(),
                    declaration,
                    space_token(),
                    self.body().format(formatter)?
                ])
            },
            || {
                Ok(format_elements![
                    self.catch_token().format(formatter)?,
                    space_token(),
                    self.body().format(formatter)?
                ])
            },
        )
    }
}

impl ToFormatElement for JsCatchDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(formatter.format_delimited(
            &self.l_paren_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    self.binding().format(formatter)?,
                    close_token_leading,
                ]))
            },
            &self.r_paren_token()?,
        )?))
    }
}

impl ToFormatElement for JsFinallyClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.finally_token().format(formatter)?,
            space_token(),
            self.body().format(formatter)?
        ])
    }
}
