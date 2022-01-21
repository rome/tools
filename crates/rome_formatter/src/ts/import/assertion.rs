use crate::{
    format_elements, group_elements, join_elements, soft_indent, soft_line_break_or_space,
    space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsImportAssertion;

impl ToFormatElement for JsImportAssertion {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let assert_token = formatter.format_token(&self.assert_token()?)?;
        let separated_assertions = formatter.format_separated(self.assertions(), space_token)?;

        let assertions = group_elements(formatter.format_delimited(
            &self.l_curly_token()?,
            |leading, trailing| {
                Ok(soft_indent(format_elements![
                    leading,
                    join_elements(soft_line_break_or_space(), separated_assertions),
                    trailing
                ]))
            },
            &self.r_curly_token()?,
        )?);

        Ok(format_elements![assert_token, space_token(), assertions])
    }
}
