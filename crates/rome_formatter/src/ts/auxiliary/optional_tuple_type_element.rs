use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{TsOptionalTupleTypeElement, TsOptionalTupleTypeElementFields};

impl ToFormatElement for TsOptionalTupleTypeElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsOptionalTupleTypeElementFields {
            ty,
            question_mark_token,
        } = self.as_fields();
        let ty = ty.format(formatter)?;
        let question_mark = question_mark_token.format(formatter)?;
        Ok(format_elements![ty, question_mark])
    }
}
