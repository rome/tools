use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsTypeArguments, TsTypeArgumentsFields};

impl FormatNodeFields<TsTypeArguments> for FormatNodeRule<TsTypeArguments> {
    fn fmt_fields(node: &TsTypeArguments, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeArgumentsFields {
            l_angle_token,
            ts_type_argument_list,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [format_delimited(
                &l_angle_token?,
                &ts_type_argument_list.format(),
                &r_angle_token?,
            )
            .soft_block_indent()]
        )
    }
}
