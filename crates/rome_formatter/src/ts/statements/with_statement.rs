use crate::{
    format_elements, group_elements, soft_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_parser::ast::JsWithStatement;

impl ToFormatElement for JsWithStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.with_token()?)?,
            space_token(),
            group_elements(formatter.format_delimited(
                &self.l_paren_token()?,
                |open_token_trailing, close_token_leading| Ok(soft_indent(format_elements![
                    open_token_trailing,
                    formatter.format_node(self.object()?)?,
                    close_token_leading,
                ])),
                &self.r_paren_token()?,
            )?),
            space_token(),
            formatter.format_node(self.body()?)?
        ])
    }
}
