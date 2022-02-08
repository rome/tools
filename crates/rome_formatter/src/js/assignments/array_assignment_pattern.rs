use crate::{
    format_elements, group_elements, join_elements, soft_block_indent, soft_line_break_or_space,
    token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsArrayAssignmentPattern;

impl ToFormatElement for JsArrayAssignmentPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let elements = formatter.format_separated(self.elements(), || token(","))?;
        Ok(group_elements(formatter.format_delimited(
            &self.l_brack_token()?,
            |open_token_trailing, close_token_leading| {
                Ok(soft_block_indent(format_elements![
                    open_token_trailing,
                    join_elements(soft_line_break_or_space(), elements),
                    close_token_leading,
                ]))
            },
            &self.r_brack_token()?,
        )?))
    }
}
