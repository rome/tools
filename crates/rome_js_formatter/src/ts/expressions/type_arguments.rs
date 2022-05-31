use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeArguments, TsTypeArgumentsFields};

impl FormatNodeFields<TsTypeArguments> for FormatNodeRule<TsTypeArguments> {
    fn format_fields(
        node: &TsTypeArguments,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTypeArgumentsFields {
            l_angle_token,
            ts_type_argument_list,
            r_angle_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_angle_token?,
                formatted![formatter, [ts_type_argument_list.format()]]?,
                &r_angle_token?,
            )
            .soft_block_indent()
            .finish()
    }
}
