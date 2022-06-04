use crate::generated::FormatJsConstructorParameterList;
use crate::prelude::*;
use rome_js_syntax::JsConstructorParameterList;

impl FormatRule<JsConstructorParameterList> for FormatJsConstructorParameterList {
    type Context = JsFormatContext;

    fn fmt(node: &JsConstructorParameterList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(token(",")))
            .finish()
    }
}
