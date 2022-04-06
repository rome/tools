use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{TsTypeArguments, TsTypeArgumentsFields};

impl ToFormatElement for TsTypeArguments {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeArgumentsFields {
            l_angle_token,
            ts_type_argument_list,
            r_angle_token,
        } = self.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_angle_token?,
            ts_type_argument_list.format(formatter)?,
            &r_angle_token?,
        )
    }
}
