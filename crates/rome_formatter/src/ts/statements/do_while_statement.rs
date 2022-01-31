use crate::{
    format_elements, group_elements, soft_indent, space_token, token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_parser::ast::JsDoWhileStatement;

impl ToFormatElement for JsDoWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.do_token()?)?,
            space_token(),
            formatter.format_node(&self.body()?)?,
            space_token(),
            formatter.format_token(&self.while_token()?)?,
            space_token(),
            group_elements(formatter.format_delimited(
                &self.l_paren_token()?,
                |open_token_trailing, close_token_leading| Ok(soft_indent(format_elements![
                    open_token_trailing,
                    formatter.format_node(&self.test()?)?,
                    close_token_leading,
                ])),
                &self.r_paren_token()?,
            )?),
            formatter
                .format_token(&self.semicolon_token())?
                .unwrap_or_else(|| token(";"))
        ])
    }
}
