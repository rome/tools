use crate::formatter_traits::FormatToken;
use crate::{
    empty_element, format_elements, group_elements, soft_indent, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsCatchClause, JsCatchDeclaration, JsFinallyClause, JsTryFinallyStatement, JsTryStatement,
};

impl ToFormatElement for JsTryStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.try_token().format(formatter),
            space_token(),
            formatter.format_node(&self.body()?)?,
            space_token(),
            formatter.format_node(&self.catch_clause()?)?
        ])
    }
}

impl ToFormatElement for JsTryFinallyStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let formatted_catch_clause = if let Some(catch_clause) = self.catch_clause() {
            format_elements![space_token(), formatter.format_node(&catch_clause)?]
        } else {
            empty_element()
        };

        Ok(format_elements![
            self.try_token().format(formatter),
            space_token(),
            formatter.format_node(&self.body()?)?,
            formatted_catch_clause,
            space_token(),
            formatter.format_node(&self.finally_clause()?)?
        ])
    }
}

impl ToFormatElement for JsCatchClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        if let Some(declaration) = self.declaration() {
            Ok(format_elements![
                self.catch_token().format(formatter),
                space_token(),
                formatter.format_node(&declaration)?,
                space_token(),
                formatter.format_node(&self.body()?)?
            ])
        } else {
            Ok(format_elements![
                self.catch_token().format(formatter),
                space_token(),
                formatter.format_node(&self.body()?)?
            ])
        }
    }
}

impl ToFormatElement for JsCatchDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(group_elements(formatter.format_delimited(
            &self.l_paren_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_indent(format_elements![
                    open_token_trailing,
                    formatter.format_node(&self.binding()?)?,
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
            self.finally_token().format(formatter),
            space_token(),
            formatter.format_node(&self.body()?)?
        ])
    }
}
