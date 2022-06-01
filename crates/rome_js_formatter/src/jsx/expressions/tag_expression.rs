use crate::prelude::*;
use crate::utils::jsx_utils::{get_wrap_state, WrapState};
use crate::FormatNodeFields;
use rome_js_syntax::JsxTagExpression;

impl FormatNodeFields<JsxTagExpression> for FormatNodeRule<JsxTagExpression> {
    fn format_fields(
        node: &JsxTagExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let tag = formatted![formatter, [node.tag().format()]]?;
        Ok(match get_wrap_state(node.syntax()) {
            WrapState::WrapOnBreak => group_elements(formatted![
                formatter,
                [
                    if_group_breaks(token("(")),
                    soft_block_indent(tag),
                    if_group_breaks(token(")"))
                ]
            ]?),
            WrapState::AlwaysWrap => group_elements(formatted![
                formatter,
                [token("("), soft_block_indent(tag), token(")")]
            ]?),
            WrapState::NoWrap => tag,
        })
    }
}
