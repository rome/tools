use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsOptionalTupleTypeElement, TsOptionalTupleTypeElementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsOptionalTupleTypeElement;

impl FormatNodeRule<TsOptionalTupleTypeElement> for FormatTsOptionalTupleTypeElement {
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
