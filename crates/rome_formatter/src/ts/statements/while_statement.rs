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
            group_elements(formatter.format_delimited_group(
                &self.l_paren_token()?,
                |leading, trailing| Ok(soft_indent(format_elements![
                    leading,
                    formatter.format_node(self.test()?)?,
                    trailing,
                ])),
                &self.r_paren_token()?,
            )?),
            space_token(),
            formatter.format_node(self.body()?)?
        ])
    }
}
