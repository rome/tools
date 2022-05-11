use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeArguments, TsTypeArgumentsFields};

impl FormatNodeFields<TsTypeArguments> for FormatNodeRule<TsTypeArguments> {
    fn format_fields(
        node: &TsTypeArguments,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let TsTypeArgumentsFields {
            l_angle_token,
            ts_type_argument_list,
            r_angle_token,
        } = node.as_fields();

        formatter.format_delimited_soft_block_indent(
            &l_angle_token?,
            formatted![formatter, [ts_type_argument_list.format()]]?,
            &r_angle_token?,
        )
    }
}
