use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeParameters, TsTypeParametersFields};

impl FormatNodeFields<TsTypeParameters> for FormatNodeRule<TsTypeParameters> {
    fn format_fields(
        node: &TsTypeParameters,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsTypeParametersFields {
            items,
            r_angle_token,
            l_angle_token,
        } = node.as_fields();

        formatter
            .delimited(
                &l_angle_token?,
                formatted![formatter, [items.format()]]?,
                &r_angle_token?,
            )
            .soft_block_indent()
            .finish()
    }
}
