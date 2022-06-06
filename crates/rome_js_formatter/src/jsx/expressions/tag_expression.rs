use crate::prelude::*;
use crate::utils::jsx_utils::{get_wrap_state, WrapState};
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsxTagExpression;

impl FormatNodeFields<JsxTagExpression> for FormatNodeRule<JsxTagExpression> {
    fn fmt_fields(node: &JsxTagExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match get_wrap_state(node.syntax()) {
            WrapState::WrapOnBreak => write![
                f,
                [group_elements(&format_args![
                    if_group_breaks(&token("(")),
                    soft_block_indent(&format_args![node.tag().format()]),
                    if_group_breaks(&token(")"))
                ])]
            ],
            WrapState::AlwaysWrap => write![
                f,
                [group_elements(&format_args![
                    token("("),
                    soft_block_indent(&format_args![node.tag().format()]),
                    token(")")
                ])]
            ],
            WrapState::NoWrap => write![f, [node.tag().format()]],
        }
    }
}
