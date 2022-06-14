use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsOptionalTupleTypeElement, TsOptionalTupleTypeElementFields};

impl FormatNodeFields<TsOptionalTupleTypeElement> for FormatNodeRule<TsOptionalTupleTypeElement> {
    fn fmt_fields(node: &TsOptionalTupleTypeElement, f: &mut JsFormatter) -> FormatResult<()> {
        let TsOptionalTupleTypeElementFields {
            ty,
            question_mark_token,
        } = node.as_fields();
        let ty = ty.format();
        let question_mark = question_mark_token.format();
        write![f, [ty, question_mark]]
    }
}
