use crate::{
    format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space, token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsCallArguments;

impl ToFormatElement for JsCallArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let args_tokens = formatter.format_separated(self.args(), || token(","))?;

        Ok(group_elements(formatter.format_delimited_group(
            &self.l_paren_token()?,
            |leading, trailing| {
                Ok(soft_indent(format_elements![
                    leading,
                    join_elements(soft_line_break_or_space(), args_tokens),
                    trailing
                ]))
            },
            &self.r_paren_token()?,
        )?))
    }
}
