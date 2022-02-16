use crate::{
    join_elements, soft_line_break_or_space, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rslint_parser::ast::TsTypeParameters;
impl ToFormatElement for TsTypeParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let items = formatter.format_separated(self.items(), || token(","))?;

        formatter.format_delimited_soft_block_indent(
            &self.l_angle_token()?,
            join_elements(soft_line_break_or_space(), items),
            &self.r_angle_token()?,
        )
    }
}
