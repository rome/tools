use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::{TsNamedTupleTypeElement, TsNamedTupleTypeElementFields};

impl FormatNodeFields<TsNamedTupleTypeElement> for FormatNodeRule<TsNamedTupleTypeElement> {
    fn format_fields(
        node: &TsNamedTupleTypeElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsNamedTupleTypeElementFields {
            ty,
            question_mark_token,
            colon_token,
            name,
            dotdotdot_token,
        } = node.as_fields();
        formatted![
            formatter,
            [
                dotdotdot_token.format(),
                name.format(),
                question_mark_token.format(),
                colon_token.format(),
                space_token(),
                ty.format(),
            ]
        ]
    }
}
