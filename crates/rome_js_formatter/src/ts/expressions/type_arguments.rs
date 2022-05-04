use crate::{Format, FormatElement, FormatNode, Formatter, JsFormatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsTypeArguments, TsTypeArgumentsFields};

impl FormatNode for TsTypeArguments {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
