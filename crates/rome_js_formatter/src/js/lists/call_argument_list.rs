use crate::generated::FormatJsCallArgumentList;
use crate::prelude::*;
use rome_js_syntax::JsCallArgumentList;

impl FormatRule<JsCallArgumentList> for FormatJsCallArgumentList {
    type Context = JsFormatContext;

    fn fmt(node: &JsCallArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(token(",")))
            .finish()
    }
}
