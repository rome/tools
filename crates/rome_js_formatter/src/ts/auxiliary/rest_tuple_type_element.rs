use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{TsRestTupleTypeElement, TsRestTupleTypeElementFields};

impl FormatNodeFields<TsRestTupleTypeElement> for FormatNodeRule<TsRestTupleTypeElement> {
    fn fmt_fields(node: &TsRestTupleTypeElement, f: &mut JsFormatter) -> FormatResult<()> {
        let TsRestTupleTypeElementFields {
            dotdotdot_token,
            ty,
        } = node.as_fields();
        let dotdotdot = dotdotdot_token.format();
        let ty = ty.format();
        write![f, [dotdotdot, ty]]
    }
}
