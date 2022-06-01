use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsOptionalTupleTypeElement, TsOptionalTupleTypeElementFields};

impl FormatNodeFields<TsOptionalTupleTypeElement> for FormatNodeRule<TsOptionalTupleTypeElement> {
    fn format_fields(
        node: &TsOptionalTupleTypeElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsOptionalTupleTypeElementFields {
            ty,
            question_mark_token,
        } = node.as_fields();
        let ty = ty.format();
        let question_mark = question_mark_token.format();
        formatted![formatter, [ty, question_mark]]
    }
}
