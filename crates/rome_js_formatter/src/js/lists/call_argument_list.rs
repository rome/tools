use crate::generated::FormatJsCallArgumentList;
use crate::prelude::*;
use rome_js_syntax::JsCallArgumentList;

impl FormatRule<JsCallArgumentList> for FormatJsCallArgumentList {
    type Context = JsFormatContext;

    fn format(
        node: &JsCallArgumentList,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","))?,
        ))
    }
}
