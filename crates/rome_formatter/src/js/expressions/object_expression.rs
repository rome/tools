use crate::formatter_traits::FormatTokenAndNode;
use crate::{
    empty_element, format_elements, group_elements, soft_block_indent, space_token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsObjectExpression;

impl ToFormatElement for JsObjectExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let members = self.members().format(formatter)?;

        let space = if members.is_empty() {
            empty_element()
        } else {
            space_token()
        };

        Ok(group_elements(formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(format_elements!(
                    space.clone(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        members,
                        close_token_leading
                    ]),
                    space,
                ))
            },
            &self.r_curly_token()?,
        )?))
    }
}
