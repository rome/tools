use crate::{format_elements, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsOptionalTupleTypeElement, TsOptionalTupleTypeElementFields};

impl FormatNode for TsOptionalTupleTypeElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsOptionalTupleTypeElementFields {
            ty,
            question_mark_token,
        } = self.as_fields();
        let ty = ty.format(formatter)?;
        let question_mark = question_mark_token.format(formatter)?;
        Ok(format_elements![ty, question_mark])
    }
}
