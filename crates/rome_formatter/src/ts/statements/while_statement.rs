use crate::{
    format_elements, group_elements, soft_indent, space_token, FormatElement, FormatResult,
    Formatter, ToFormatElement,
};
use rslint_parser::ast::JsWhileStatement;

impl ToFormatElement for JsWhileStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.while_token()?)?,
            space_token(),
            group_elements(format_elements![
                formatter.format_token(&self.l_paren_token()?)?,
                soft_indent(formatter.format_node(self.test()?)?),
                formatter.format_token(&self.r_paren_token()?)?
            ]),
            space_token(),
            formatter.format_node(self.body()?)?
        ])
    }
}
