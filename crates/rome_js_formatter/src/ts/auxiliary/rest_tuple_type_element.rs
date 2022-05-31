use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsRestTupleTypeElement, TsRestTupleTypeElementFields};

impl FormatNodeFields<TsRestTupleTypeElement> for FormatNodeRule<TsRestTupleTypeElement> {
    fn format_fields(
        node: &TsRestTupleTypeElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsRestTupleTypeElementFields {
            dotdotdot_token,
            ty,
        } = node.as_fields();
        let dotdotdot = dotdotdot_token.format();
        let ty = ty.format();
        formatted![formatter, [dotdotdot, ty]]
    }
}
