use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{TsRestTupleTypeElement, TsRestTupleTypeElementFields};

impl ToFormatElement for TsRestTupleTypeElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsRestTupleTypeElementFields {
            dotdotdot_token,
            ty,
        } = self.as_fields();
        let dotdotdot = dotdotdot_token.format(formatter)?;
        let ty = ty.format(formatter)?;
        Ok(format_elements![dotdotdot, ty])
    }
}
