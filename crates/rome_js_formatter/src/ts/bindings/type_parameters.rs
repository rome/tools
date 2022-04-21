use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::{TsTypeParameters, TsTypeParametersFields};

impl FormatNode for TsTypeParameters {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeParametersFields {
            items,
            r_angle_token,
            l_angle_token,
        } = self.as_fields();
        let items = items.format(formatter)?;

        formatter.format_delimited_soft_block_indent(&l_angle_token?, items, &r_angle_token?)
    }
}
