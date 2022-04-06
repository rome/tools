use crate::formatter_traits::FormatTokenAndNode;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{TsTypeParameters, TsTypeParametersFields};

impl ToFormatElement for TsTypeParameters {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeParametersFields {
            items,
            r_angle_token,
            l_angle_token,
        } = self.as_fields();
        let items = items.format(formatter)?;

        formatter.format_delimited_soft_block_indent(&l_angle_token?, items, &r_angle_token?)
    }
}
