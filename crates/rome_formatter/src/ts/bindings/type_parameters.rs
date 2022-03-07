use crate::formatter::TrailingSeparator;
use crate::{
    join_elements, soft_line_break_or_space, token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{TsTypeParameters, TsTypeParametersFields};

impl ToFormatElement for TsTypeParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeParametersFields {
            items,
            r_angle_token,
            l_angle_token,
        } = self.as_fields();
        let items =
            formatter.format_separated(&items, || token(","), TrailingSeparator::default())?;

        formatter.format_delimited_soft_block_indent(
            &l_angle_token?,
            join_elements(soft_line_break_or_space(), items),
            &r_angle_token?,
        )
    }
}
